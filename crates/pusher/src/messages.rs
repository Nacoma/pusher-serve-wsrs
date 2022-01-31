use actix::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Message)]
#[rtype(result = "()")]
pub struct OutgoingMessage(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub struct PusherMessage {
    pub name: Option<String>,
    pub event: Option<String>,
    pub data: MessageData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageData {
    #[serde(with = "serde_with::json::nested", default)]
    pub channel_data: Option<PusherMessageChannelData>,
    pub channel: Option<String>,
    pub auth: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PusherMessageChannelData {
    pub user_id: String,
    pub user_info: serde_json::Value,
}
