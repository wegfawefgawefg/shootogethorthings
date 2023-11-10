use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientToServerMessageBundle {
    // metadata
    pub client_id: u32,
    pub send_time: i64,
    pub received_time: i64,

    // actual message
    pub message: ClientToServerMessageData,
}

impl ClientToServerMessageBundle {
    pub fn new(client_id: u32, message: ClientToServerMessage) -> Self {
        Self {
            client_id,
            send_time: message.send_time,
            received_time: crate::common::util::get_utc_now(),
            message: message.data,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientToServerMessage {
    // metadata
    pub send_time: i64,

    // actual message
    pub data: ClientToServerMessageData,
}

impl ClientToServerMessage {
    pub fn new(data: ClientToServerMessageData) -> Self {
        Self {
            send_time: crate::common::util::get_utc_now(),
            data,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClientToServerMessageData {
    Connect,
    Disconnect,
    ChatMessage { message: String },
    RequestToSpawnPlayer,
    RequestAllPlayers,
    EntityPosition { entity_id: u32, pos: glam::Vec2 },
}
