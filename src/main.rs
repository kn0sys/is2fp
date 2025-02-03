#[macro_use]
extern crate rocket;

use rocket::{
    catch,
    get,
    http::Status,
    post,
    response::status::Custom,
    serde::json::Json,
};

use is2fp::{i2p, error as ip2p_error, utils};
use serde::{Deserialize, Serialize};


#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Message {
    pub mid: String,
    pub data: String,
    pub created: i64,
    pub from: String,
    pub to: String,
}

// Catchers
//----------------------------------------------------------------

#[catch(404)]
pub fn not_found() -> Custom<Json<ip2p_error::ErrorResponse>> {
    Custom(
        Status::NotFound,
        Json(ip2p_error::ErrorResponse {
            error: String::from("Resource does not exist"),
        }),
    )
}

#[catch(500)]
pub fn internal_error() -> Custom<Json<ip2p_error::ErrorResponse>> {
    Custom(
        Status::InternalServerError,
        Json(ip2p_error::ErrorResponse {
            error: String::from("Internal server error"),
        }),
    )
}

/// If i2p not in the state of rejecting tunnels this will return `open: true`
///
/// This also functions as a health check
#[get("/status")]
pub async fn get_i2p_status() -> Custom<Json<i2p::HttpProxyStatus>> {
    let status = i2p::check_connection().await;
    if status.unwrap_or(i2p::ProxyStatus::Opening) == i2p::ProxyStatus::Open {
        Custom(Status::Ok, Json(i2p::HttpProxyStatus { open: true }))
    } else {
        Custom(Status::Ok, Json(i2p::HttpProxyStatus { open: false }))
    }
}

/// Recieve messages here
#[post("/", data = "<message>")]
pub async fn message(message: Json<Message>) -> Custom<Json<Message>> {
    // TODO: broadcast message via MDNS
    Custom(Status::Ok, Json(Default::default()))
}

// Launch the i2p relay server
#[launch]
async fn rocket() -> _ {
    env_logger::init();
    let config = rocket::Config {
        ident: rocket::config::Ident::none(),
        ip_header: None,
        port: utils::get_app_port(),
        ..rocket::Config::debug_default()
    };
    utils::start_up().await.expect("i2p start failure");
    rocket::custom(&config)
        .register(
            "/",
            catchers![internal_error, not_found],
        )
        .mount("/message", routes![message])
        .mount("/i2p", routes![get_i2p_status])
}

