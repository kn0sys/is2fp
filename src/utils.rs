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
use rand::seq::*;
use serde::{Deserialize, Serialize};
use sha2::{
    Digest,
    Sha512,
};
use rocket::serde::json::Json;
use lazy_static::lazy_static;
use std::sync::Mutex;

const RELAY_KEY: &str = "b32";
const FLUFF_KEY: &str = "fluff";
const NETWORK_FLUFF: u64 = 32;
const POW_LIMIT: u64 = 123456789;

lazy_static! {
    /// used to prevent LMDB errors while propagating fluff messages
    static ref IS_FLUFF_LOCKED: Mutex<bool> = Mutex::new(false);
}


#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(crate = "rocket::serde")]
pub enum MessageType {
    B32Exchange,
    Stem,
    Fluff,
}

impl Default for MessageType {
    fn default() -> Self {
        Self::Stem
    }
}

#[derive(Debug)]
pub struct MessageLimits {
    mid: usize,
    data: usize,
    from: usize,
    to: usize,
}

impl Default for MessageLimits {
    fn default() -> Self {
        MessageLimits {
            mid: 128,
            data: 1048,
            from: 128,
            to: 128,
        }
    }
}

impl MessageLimits {
    pub fn validate(m: &Message) -> bool {
        let limit: MessageLimits = Default::default();
        m.mid.len() < limit.mid
        && m.data.len() < limit.data
        && m.from.len() < limit.from
        && m.to.len() < limit.to
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Message {
    pub mid: String,
    pub data: String,
    pub created: u64,
    pub from: String,
    pub to: String,
    pub m_type: MessageType,
    pub fluff_probability: f64,
    pub pow_problem: String,
    /// H(solution) = H(fluff_probability+R), where R is some
    /// random number 1-123456789.
    pub pow_solution: String,
}

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
    let l = &db::DATABASE_LOCK;
    let v = bincode::serialize(&i2p::ProxyStatus::Opening).unwrap_or_default();
    db::write_chunks(&l.env, &l.handle, i2p::I2P_STATUS.as_bytes(), &v)
        .map_err(|_| is2fp_error::Ip2pError::Database(MdbError::Panic))?;
    Ok(())
}

fn handle_messages(msg: Message, peer_id: libp2p::PeerId, local_peer_id: libp2p::PeerId) -> Result<(), is2fp_error::Ip2pError> {
    log::info!("handling message type: {:?}", &msg.m_type);
    if msg.m_type == MessageType::B32Exchange {
        log::info!("processin address {} for relays", &msg.data.clone());
        // save b32.i2p for stem selection
        let mid = &msg.mid.clone();
        let l = &db::DATABASE_LOCK;
        let key = format!("{}-{}", RELAY_KEY, &peer_id);
        let b_key = key.as_bytes().to_vec();
        let b32 = db::DatabaseEnvironment::read(&l.env, &l.handle, &b_key)
            .unwrap_or_default();
        if b32.is_empty() {
            log::info!("writing new relay:{:?} to lmdb", &peer_id);
            let bytes_b32 = bincode::serialize(&msg.data).unwrap_or_default();
            db::write_chunks(&l.env, &l.handle, &b_key, &bytes_b32)
                .unwrap_or_else(|_| log::error!("failed to add b32: {} for peer {}", &msg.data, &peer_id));
        } else {
            // TODO: environment variable for saving all messages
            // for now, just save messages directed to our peer_id
            let is_valid = MessageLimits::validate(&msg);
            let inbox_key = "inbox".as_bytes().to_vec();
            let mut our_messages: Vec<Message> = Vec::new();
            let b_old_messages = db::DatabaseEnvironment::read(&l.env, &l.handle, &inbox_key)
                .map_err(|_| is2fp_error::Ip2pError::Database(MdbError::Panic))?;
            let mut old_messages: Vec<Message> = bincode::deserialize(&b_old_messages[..]).unwrap_or_default();
            our_messages.append(&mut old_messages);
            if is_valid && &msg.to == &format!("{local_peer_id}") {
                log::debug!("saving new message to db");
                our_messages.push(msg);
            }
            let b_our_messages = bincode::serialize(&our_messages).unwrap_or_default();
            db::write_chunks(&l.env, &l.handle, &inbox_key, &b_our_messages)
                .unwrap_or_else(|_| log::error!("failed to add message {} to db", mid));
        }
    }
    Ok(())
}

pub async fn select_invisible_stem(mut msg: Message, peers: Vec<&libp2p::PeerId>) -> Result<(), is2fp_error::Ip2pError> {
    log::info!("start invisible stem selection");
    // get random peer and their b32 address
    let r_peer = *peers.choose(&mut rand::rng()).unwrap();
    let l = &db::DATABASE_LOCK;
    let b32_key = format!("{}-{}", RELAY_KEY, r_peer);
    let bytes_b32_key = b32_key.as_bytes().to_vec();
    let b_b32 = db::DatabaseEnvironment::read(&l.env, &l.handle, &bytes_b32_key)
        .unwrap_or_default();
    let relay_b32: String = bincode::deserialize(&b_b32[..]).unwrap_or_default();
    // generate pow problem
    let big_r: u64 = rand::random_range(0..POW_LIMIT);
    let mut hasher = Sha512::new();
    hasher.update(format!("{}", big_r + NETWORK_FLUFF).as_bytes());
    let hash = hasher.finalize().to_owned();
    let s_hash = hex::encode(&hash[..]);
    log::debug!("pow hash: {}", &s_hash);
    msg.pow_problem = String::from(&s_hash);
    // pass message to invisible stem via i2p http proxy
    info!("broadcasting message to relay: {}", &relay_b32);
    let host = get_i2p_http_proxy();
    let proxy = reqwest::Proxy::http(&host)
        .map_err(|_| is2fp_error::Ip2pError::Relay)?;
    let client = reqwest::Client::builder().proxy(proxy).build();
    match client.map_err(|_| is2fp_error::Ip2pError::Relay)?
        .get(format!("http://{}/message", &relay_b32))
        .send()
        .await
    {
        Ok(response) => {
            let res = response.json::<Message>().await;
            match res {
                Ok(_) => log::info!("relay success"),
                _ => log::warn!("unknown relay status"),
            }
        }
        Err(e) => {
            error!("failed to relay due to: {:?}", e);
        }
    }
    Ok(())
}

fn extract_fluff() -> Vec<Message> {
    let l = &db::DATABASE_LOCK;
    let k = FLUFF_KEY.as_bytes().to_vec();
    let b_fluff = db::DatabaseEnvironment::read(&l.env, &l.handle, &k).unwrap_or_default();
    let v_fluff: Vec<Message> = bincode::deserialize(&b_fluff[..]).unwrap_or_default();
    if v_fluff.is_empty() {
        log::info!("no messages found for fluff extraction");
    }
    v_fluff
}

/// Consume a message over i2p relay from a random peer.
///
/// Solve the proof-of-work and timestamp the message into LMDB.
///
/// Fluff propagation vector is consumed by the network event loop on
///
/// a randomly, rotating basis. A `Mutex<bool>` prevents access while
///
/// mutating the vector.
pub fn inject_fluff(j_msg: Json<Message>) -> Result<(), is2fp_error::Ip2pError> {
    *IS_FLUFF_LOCKED.lock().unwrap() = true;
    use::std::time::{SystemTime, UNIX_EPOCH };
    log::info!("injecting fluff msg: {}", &j_msg.mid.clone());
    let l = &db::DATABASE_LOCK;
    let k = FLUFF_KEY.as_bytes().to_vec();
    let b_old_fluff = db::DatabaseEnvironment::read(&l.env, &l.handle, &k)
        .unwrap_or_default();
    let mut old_fluff: Vec<Message> = bincode::deserialize(&b_old_fluff[..])
        .unwrap_or_default();
    let start = SystemTime::now();
    let created = start.duration_since(UNIX_EPOCH).map_err(|_| is2fp_error::Ip2pError::Unknown)?.as_secs(); 
    let data = j_msg.data.clone();
    let pow_problem = j_msg.pow_problem.clone();
    let pow_solution = do_pow(&pow_problem)?;
    let m_type = MessageType::Fluff;
    let new_msg: Message = Message { created, data, m_type, pow_problem, pow_solution, ..Default::default() };
    old_fluff.push(new_msg);
    db::DatabaseEnvironment::delete(&l.env, &l.handle, &k)
        .unwrap_or_else(|_| log::error!("failed to delete fluff injection vector"));
    let new_fluff = bincode::serialize(&old_fluff).unwrap_or_default();
    db::write_chunks(&l.env, &l.handle, &k, &new_fluff)
        .unwrap_or_else(|_| log::error!("failed to write new fluff injection vector"));
    *IS_FLUFF_LOCKED.lock().unwrap() = false;
    Ok(())
}

fn do_pow(problem: &String) -> Result<String, is2fp_error::Ip2pError> {
    // generate pow solution
    for n in 0..POW_LIMIT {
        let mut hasher = Sha512::new();
        hasher.update(format!("{}", n + NETWORK_FLUFF));
        let hash = hasher.finalize().to_owned();
        let s_hash = hex::encode(&hash[..]);
        if s_hash == *problem {
            log::info!("found solution to: {}", problem);
            return Ok(s_hash);
        }
    }
    log::error!("failure to solve: {}", problem);
    Err(is2fp_error::Ip2pError::PowError)
}

pub async fn run_network() {
    log::info!("IS2FP Console v0.1.0-alpha\n
                add peer /ip4/<IP>/tcp/<PORT>/p2p/<PEER_ID>\n
                send <MESSAGE>");
    // fluff probability and stem extension timeout will be randomized per message
    let mut node = DandelionNode::new(
        0.0,
        Duration::from_millis(0),
    ).await.unwrap();

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
        // Use network fluff as millisecond range generated randomly on network event loop
        let r_tick = rand::random_range(0..NETWORK_FLUFF);
        let tick = tokio::time::sleep(Duration::from_millis(r_tick));
        let fluff_msgs: Vec<Message> = extract_fluff();
        if !fluff_msgs.is_empty() && !*IS_FLUFF_LOCKED.lock().unwrap() {
            for m in fluff_msgs {    
                let b_msg = bincode::serialize(&m).unwrap_or_default();
                if let Err(e) = node.broadcast_message(b_msg, fluff_topic.clone()) {
                    log::error!("fluff propagation failed for msg id: {} because: {:?}", &m.mid, e);
                }
            }
        }
        select! {
            Ok(Some(line)) = stdin.next_line() => {
                if line.starts_with("add peer ") {
                    let address = &line.split("add peer ").collect::<Vec<&str>>().join("");
                    log::info!("adding peer: {}", address);
                    let ma = address.parse::<libp2p::Multiaddr>().unwrap();
                    if let Err(e) = node.connect(ma).await {
                        log::error!("failed to connect to manually: {:?}", e);
                    } 
                } else if line.starts_with("send ") {
                    let p_msg = &line.split("send ").collect::<Vec<&str>>().join("");
                    log::info!("sending message: {}", &p_msg);
                    // TODO: send to invisible stem extension
                    let mut msg: Message = Default::default();
                    msg.data = String::from(p_msg);
                    let peers = node.swarm.connected_peers().collect::<Vec<_>>();
                    select_invisible_stem(msg, peers).await
                        .unwrap_or_else(|_| log::error!("failed to select invisible stem"));
                    // TODO: option for clear message broadcasting (i.e. debug mode)
                    //if let Err(e) = node.broadcast_message(b_msg, fluff_topic.clone()) {
                    //    log::error!("Publish error: {e:?}");
                    //}
                }
            }
            event = node.swarm.select_next_some() => match event {
                SwarmEvent::Behaviour(DandelionBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                    for (peer_id, multiaddr) in list {
                        log::info!("mDNS discovered a new stem: {multiaddr}");
                        if let Err(e) = node.connect(multiaddr).await {
                            log::error!("failed to connect to {:?}: {:?}", &peer_id, e);
                        }
                    }
                },
                SwarmEvent::Behaviour(DandelionBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                    for (peer_id, _multiaddr) in list {
                        println!("mDNS stem has expired: {peer_id}");
                        if let Err(e) = node.handle_peer_disconnect(peer_id).await {
                            log::error!("failed to handle {peer_id} disconnect: {:?}", e);
                        }
                    }
                },
                SwarmEvent::Behaviour(DandelionBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                    propagation_source: peer_id,
                    message_id: _id,
                    message,
                })) => {
                    let msg: Message = bincode::deserialize(&message.data).unwrap_or_default();
                    log::info!("anon: {}", &msg.data);
                    let local_peer_id = node.swarm.local_peer_id();
                    if let Err(e) = handle_messages(msg.clone(), peer_id, *local_peer_id) {
                        log::error!("failed to handle {:?}: {:?}", &msg.m_type, e);
                    }
                }
                SwarmEvent::NewListenAddr { address, .. } => {
                    log::info!("Local node is listening on {address}/p2p/{}", node.swarm.local_peer_id());
                },
                SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                    log::info!("Connected to peer: {:?}", peer_id);
                    // TODO: optimize waiting for protocol confirmation
                    tokio::time::sleep(Duration::from_secs(3)).await; 
                    // execute b32 address exchange
                    let mut msg: Message = Default::default();
                    msg.data = i2p::get_destination().unwrap_or_default();
                    msg.m_type = MessageType::B32Exchange;
                    let b_msg = bincode::serialize(&msg).unwrap_or_default();
                    let topic = gossipsub::IdentTopic::new(format!("stem-{}", &peer_id));
                    // dont use the first peer as a relay
                    let peers = node.swarm.connected_peers().collect::<Vec<_>>();
                    if peers.len() > 1 {
                        if let Err(e) = node.broadcast_message(b_msg, topic) {
                            log::error!("b32 exchange with {:?} failed, {:?}", &peer_id, e);
                        }
                    }
                },
                _ => {}
            },
            _ = tick => {
                // exit the network loop and check for fluff propagation messages
                continue;
            }
        }
    }
}

/// primary is2fp entry point
///
/// Start i2p first, then the lip2p2 swarm.
///
/// The initial server startup won't output
///
/// the i2p fluff propagation b32 address.
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
                        log::warn!("i2p has not warmed up yet, check wrapper.log");
                    } else {
                        break;
                    }
                    std::thread::sleep(std::time::Duration::from_secs(60));
                }
                log::info!("i2p fluff propagation server online");
                // i2p relay server is up, start the swarm
                run_network().await;
        });
    }
    info!("dandelion-is2fp is online");
    let destination = i2p::get_destination();
    info!("relay server address - {}", destination?);
    Ok(())
}
