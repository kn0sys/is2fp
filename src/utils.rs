use kn0sys_lmdb_rs::MdbError;
use crate::{i2p, db, error as is2fp_error};
use log::*;
use tokio::{io, select, io::AsyncBufReadExt};
use std::{
    time::Duration,
};
use libp2p_dandelion::*;
use futures::stream::StreamExt;
use libp2p::{
    gossipsub, mdns,
    swarm::{SwarmEvent},
};

/// app port
pub fn get_app_port() -> u16 {
    // attempt environment variable extraction, fall to default
    let port = std::env::var(i2p::IS2FP_PORT)
        .unwrap_or(i2p::DEFAULT_APP_PORT.to_string());
    if port.is_empty() {
        i2p::DEFAULT_APP_PORT
    } else {
        port.parse::<u16>().unwrap_or(i2p::DEFAULT_APP_PORT)
    }
}

/// i2p http proxy
pub fn get_i2p_http_proxy() -> String {
    // attempt environment variable extraction, fall to default
    let proxy = std::env::var(i2p::I2P_PROXY_HOST);
    proxy.unwrap_or(
        format!("{}:{}",
            i2p::DEFAULT_HTTP_PROXY_HOST,
            i2p::DEFAULT_HTTP_PROXY_PORT
        )
    )
}

fn reset_i2p_status() -> Result<(), is2fp_error::Ip2pError> {
    let db = &db::DATABASE_LOCK;
    let v = bincode::serialize(&i2p::ProxyStatus::Opening).unwrap_or_default();
    db::write_chunks(&db.env, &db.handle, i2p::I2P_STATUS.as_bytes(), &v)
        .map_err(|_| is2fp_error::Ip2pError::Database(MdbError::Panic))?;
    Ok(())
}

pub async fn run_network() -> Result<(), is2fp_error::Ip2pError> {
    log::info!("Enter messages via STDIN and they will be sent to connected peers using Gossipsub");

    let mut node = DandelionNode::new(
        0.3,  // fluff probability
        Duration::from_secs(30), // stem timeout
    ).await.map_err(|_| is2fp_error::Ip2pError::Unknown)?;

    // Create a Gossipsub topic and subscribe to our own topic
    let broadcast_topic = gossipsub::IdentTopic::new(format!("stem-{}", node.swarm.local_peer_id()));
    node.subscribe(&broadcast_topic).unwrap();
    // Create the main topic for listening for fluff messages
    let fluff_topic = gossipsub::IdentTopic::new(format!("fluff"));
    node.subscribe(&fluff_topic).unwrap();

    // Read from standard input for chat
    let mut stdin = io::BufReader::new(io::stdin()).lines();

    // Kick it off
    loop {
        select! {
            Ok(Some(line)) = stdin.next_line() => {
                if let Err(e) = node.broadcast_message(line.as_bytes().to_vec(), fluff_topic.clone()) {
                    log::error!("Publish error: {e:?}");
                }
            }
            event = node.swarm.select_next_some() => match event {
                SwarmEvent::Behaviour(DandelionBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                    for (_peer_id, multiaddr) in list {
                        println!("mDNS discovered a new peer: {multiaddr}");
                        node.connect(multiaddr).await.map_err(|_| is2fp_error::Ip2pError::Unknown)?;
                        // TODO: i2p base32 handshake
                    }
                },
                SwarmEvent::Behaviour(DandelionBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                    for (peer_id, _multiaddr) in list {
                        println!("mDNS discover peer has expired: {peer_id}");
                        node.handle_peer_disconnect(peer_id).await.map_err(|_| is2fp_error::Ip2pError::Unknown)?;
                    }
                },
                SwarmEvent::Behaviour(DandelionBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                    propagation_source: peer_id,
                    message_id: id,
                    message,
                })) => println!(
                        "Got message: '{}' with id: {id} from peer: {peer_id}",
                        String::from_utf8_lossy(&message.data),
                        // TODO: save directed messages to db
                    ),
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Local node is listening on {address}");
                }
                _ => {}
            }
        }
    }

}

pub async fn start_up() -> Result<(), is2fp_error::Ip2pError> {
    info!("is2fp is starting up");
    reset_i2p_status()?;
    i2p::start()?;
    // start async background tasks here
    {
        tokio::spawn(async move {
                loop {
                    let is_i2p_online = i2p::check_connection().await;
                    let i2p_status = is_i2p_online.unwrap_or(i2p::ProxyStatus::Opening);
                    if i2p_status == i2p::ProxyStatus::Opening {
                        log::error!("i2p has not warmed up yet, check wrapper.log");
                    } else {
                        break;
                    }
                    std::thread::sleep(std::time::Duration::from_secs(60));
                }
                log::info!("i2p fluff propagation server online");
                // i2p relay server is up, start the swarm
                run_network().await.unwrap_or_else(|_| log::error!("failed to start swarm"));
        });
    }
    info!("dandelion-is2fp is online");
    let destination = i2p::get_destination();
    info!("relay server address - {}", destination?);
    Ok(())
}
