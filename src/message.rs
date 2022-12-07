use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscription {
    pub device_id: i64,
    pub device_finger_print: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum VisitFailureReason {
    RemoteReject,
    InvalidPassword,
    InternalError,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    Pong(i64),
    VisitRequest {
        active_device_id: i64,
        passive_device_id: i64,
        visit_desktop: bool,
        #[serde(with = "serde_bytes")]
        password_salt: Vec<u8>,
        #[serde(with = "serde_bytes")]
        secret: Vec<u8>,
        #[serde(with = "serde_bytes")]
        secret_nonce: Vec<u8>,
    },
}

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    Ping(i64),
    VisitResponse {
        active_device_id: i64,
        passive_device_id: i64,
        #[serde_as(as = "Result<serde_with::Bytes, _>")]
        result: Result<Vec<u8>, VisitFailureReason>,
    },
}
