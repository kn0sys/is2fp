use kn0sys_lmdb_rs::MdbError;
use thiserror::Error;
use serde::{Deserialize, Serialize};

/// Use for mapping errors in functions that can throw multiple errors.
#[derive(Debug, Error)]
#[error("ip2p error. See logs for more info.")]
pub enum Ip2pError {
    Database(MdbError),
    I2P,
    J4I2PRS,
    Message,
    RocketError(rocket::Error),
    Unknown,
}

/// For handling 404 and 500 error responses
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorResponse {
    pub error: String,
}
