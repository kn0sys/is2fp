//! embedded i2p module

use crate::{utils, db, error as ip2p_error, i2p};
/// Environment variable for the application custom port
pub const IS2FP_PORT:                   &str = "IS2FP_PORT";
/// Default app port
pub const DEFAULT_APP_PORT:             u16 = 5555;
/// Default http proxy port
pub const DEFAULT_HTTP_PROXY_PORT:      u16 = 4242;
/// Default app host
pub const DEFAULT_HTTP_PROXY_HOST:      &str = "127.0.0.1";
/// I2P CONNECTION CHECK
pub const I2P_STATUS:                   &str = "I2P_STATUS";
/// Environment variable for the i2p proxy host
pub const I2P_PROXY_HOST:               &str = "I2P_PROXY_HOST";
/// LMDB key for the base 32 address
pub const APP_B32_DEST:                 &str = "app-b32";
/// LMDB key for the relay server secret key
pub const APP_I2P_SK:                   &str = "app-i2p-sk";
/// Override router creation and startup for machines with existing router instances
pub const IS2FP_ROUTER_OVERRIDE:        &str = "IS2FP_ROUTER_OVERRIDE";

use lazy_static::lazy_static;
use std::sync::Mutex;
use j4i2prs::{
    router_wrapper as rw,
    tunnel_control as tc,
};
use kn0sys_lmdb_rs::MdbError;
use log::*;
use serde::{
    Deserialize,
    Serialize,
};
use std::{
    fs::File,
    io::{
        self,
        BufRead,
    },
    path::Path,
    sync::mpsc::{
        Receiver,
        Sender,
    },
    thread,
};

// global
lazy_static! {
    /// prevents infinite loop when checking for running router
    static ref IS_ROUTER_RUNNING: Mutex<bool> = Mutex::new(false);
}

struct Listener {
    run_tx: Sender<bool>,
    run_rx: Receiver<bool>,
}

impl Default for Listener {
    fn default() -> Self {
        let (run_tx, run_rx) = std::sync::mpsc::channel();
        Listener {
            run_tx,
            run_rx,
        }
    }
}

/// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HttpProxyStatus {
    pub open: bool,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum ProxyStatus {
    Opening,
    Open,
}

impl ProxyStatus {
    pub fn value(&self) -> String {
        match *self {
            ProxyStatus::Opening => String::from("opening\n"),
            ProxyStatus::Open => String::from("open\n"),
        }
    }
}

/// Extract i2p port from command line arg
fn get_i2p_proxy_port() -> String {
    let proxy_host = utils::get_i2p_http_proxy();
    let values = proxy_host.split(":");
    let mut v: Vec<String> = values.map(String::from).collect();
    v.remove(1)
}

/// This is the `dest` value of the app i2p tunnels
pub fn get_destination() -> Result<String, ip2p_error::Ip2pError> {
    let db = &db::DATABASE_LOCK;
    let r_app_b32_dest = db::DatabaseEnvironment::read(
        &db.env,
        &db.handle,
        &i2p::APP_B32_DEST.as_bytes().to_vec(),
    )
    .map_err(|_| ip2p_error::Ip2pError::Database(MdbError::Panic))?;
    let app_b32_dest: String = bincode::deserialize(&r_app_b32_dest[..]).unwrap_or_default();
    Ok(app_b32_dest)
}

/// Read base 32 destination address from LMDB
pub async fn check_connection() -> Result<ProxyStatus, ip2p_error::Ip2pError> {
    let db = &db::DATABASE_LOCK;
    let r =
        db::DatabaseEnvironment::read(&db.env, &db.handle, &i2p::I2P_STATUS.as_bytes().to_vec())
            .map_err(|_| ip2p_error::Ip2pError::Database(MdbError::Panic))?;
    if r.is_empty() {
        error!("i2p status not found");
        return Err(ip2p_error::Ip2pError::Database(MdbError::NotFound));
    }
    let result: ProxyStatus = bincode::deserialize(&r[..]).unwrap_or(ProxyStatus::Opening);
    Ok(result)
}

/// Create app tunnel if it don't exist yet
fn create_server_tunnel() -> Result<tc::Tunnel, ip2p_error::Ip2pError> {
    let port: u16 = utils::get_app_port();
    let b32_key = i2p::APP_B32_DEST.as_bytes();
    let sk_key = i2p::APP_I2P_SK.as_bytes();
    let db = &db::DATABASE_LOCK;
    let tunnel: tc::Tunnel =
        tc::Tunnel::new("127.0.0.1".to_string(), port, tc::TunnelType::Server).unwrap_or_default();
    let b32_dest: String = tunnel.get_destination();
    log::debug!("destination: {}", &b32_dest);
    let v_b32_dest = bincode::serialize(&b32_dest).unwrap_or_default();
    let v_sk = bincode::serialize(&tunnel.get_sk()).unwrap_or_default();
    db::write_chunks(&db.env, &db.handle, b32_key, &v_b32_dest)
        .map_err(|_| ip2p_error::Ip2pError::Database(MdbError::Panic))?;
    db::write_chunks(&db.env, &db.handle, sk_key, &v_sk)
        .map_err(|_| ip2p_error::Ip2pError::Database(MdbError::Panic))?;
    Ok(tunnel)
}

/// helper method for tunnel creation
fn process_tunnels(http_proxy_port: u16, app_sk: String) {
    if let Ok(lines) = read_lines("./router.config") {
        for line in lines.map_while(Result::ok) {
            if line.contains("i2np.udp.port") {
                let port = line.split("=").collect::<Vec<&str>>()[1];
                log::info!("router is running on external port = {}", port);
                log::info!("open this port for better connectivity");
                log::info!("this port was randomly assigned, keep it private");
                //l.is_running = true;
                *IS_ROUTER_RUNNING.lock().unwrap() = true;
                // start the http proxy
                let http_proxy: tc::Tunnel = tc::Tunnel::new(
                    "127.0.0.1".to_string(),
                    http_proxy_port,
                    tc::TunnelType::Http,
                ).unwrap_or_default();
                let _ = http_proxy.start(None);
                log::info!("http proxy on port {}", http_proxy.get_port());
                if app_sk.is_empty() {
                    let t = create_server_tunnel().unwrap_or_default();
                    let _ = t.start(None);
                } else {
                    let app_tunnel = tc::Tunnel::new(
                        "127.0.0.1".to_string(),
                        utils::get_app_port(),
                        tc::TunnelType::ExistingServer,
                    ).unwrap_or_default();
                    let _ = app_tunnel.start(Some(String::from(&app_sk)));
                }
                    let db = &db::DATABASE_LOCK;
                    let v = bincode::serialize(&ProxyStatus::Open).unwrap_or_default();
                    db::write_chunks(&db.env, &db.handle, i2p::I2P_STATUS.as_bytes(), &v)
                        .unwrap_or_else(|_| log::error!("failed to write i2p status."));
            }
        }
    }
}

/// Start router and automatic i2p tunnel creation
///
/// We'll check for an existing i2p secret key. If it doesn't
///
/// exist create a new one.
pub fn start() -> Result<(), ip2p_error::Ip2pError> {
    let http_proxy_port: u16 = get_i2p_proxy_port()
        .parse::<u16>()
        .unwrap_or(DEFAULT_HTTP_PROXY_PORT);
    // check for existing app and anon inbound server tunnels
    let db = &db::DATABASE_LOCK;
    let r_app_sk =
        db::DatabaseEnvironment::read(&db.env, &db.handle, &i2p::APP_I2P_SK.as_bytes().to_vec())
            .map_err(|_| ip2p_error::Ip2pError::Database(MdbError::Panic))?;
    let app_sk: String = bincode::deserialize(&r_app_sk[..]).unwrap_or_default();
    log::info!("starting j4i2prs...");
    // If you want to run multiple instances on the same machine set IS2FP_ROUTER_OVERRIDE=1
    let pre_router = rw::Wrapper::create_router().map_err(|_| ip2p_error::Ip2pError::I2P);
    let router_override_disabled = std::env::var(IS2FP_ROUTER_OVERRIDE)
        .unwrap_or("1".to_string()).is_empty();
    if pre_router.is_err() && router_override_disabled {
        panic!("fatal i2p router error (see wrapper.log)");
    }
    let mut o_router: Option<rw::Wrapper> = None;
    if router_override_disabled {
        o_router = Some(pre_router?);
    }
    let l: Listener = Default::default();
    let run_tx = l.run_tx.clone();
    let _ = thread::spawn(move || {
        log::info!("run thread started");
        run_tx
            .send(true)
            .unwrap_or_else(|_| log::error!("failed to run router"));
    });

    // run the main thread forever unless we get a router shutdown signal
    let _ = thread::spawn(move || {
        if !router_override_disabled {
            process_tunnels(http_proxy_port, app_sk);
        } else {
            // it should be safe to unwrap the router here...
            let r = o_router.unwrap();
            std::thread::sleep(std::time::Duration::from_secs(10));
            loop {
                if let Ok(run) = l.run_rx.try_recv() {
                    if run {
                        log::info!("starting router");
                    r.invoke_router(rw::METHOD_RUN)
                          .unwrap_or_else(|_| log::error!("failed to run router"));
                    }
                }
                let is_running: bool = match IS_ROUTER_RUNNING.lock() {
                    Ok(m) => *m,
                    Err(_) => false,
                };
                if is_running {
                    let is_router_on = r.is_running().unwrap_or_default();
                    if !is_router_on {
                        log::info!("router is warming up, please wait...");
                    }
                    std::thread::sleep(std::time::Duration::from_secs(60));
                    if is_router_on {
                        // check router config
                        process_tunnels(http_proxy_port, app_sk.clone());
                    }
                }
            }
        }
    });
    Ok(())
}
