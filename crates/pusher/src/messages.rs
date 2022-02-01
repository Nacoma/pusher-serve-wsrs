use actix::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Message)]
#[rtype(result = "()")]
pub struct OutgoingMessage(pub Box<dyn JsonMessage>);

pub trait JsonMessage: erased_serde::Serialize + Send {}

erased_serde::serialize_trait_object!(JsonMessage);

impl Into<String> for OutgoingMessage {
    fn into(self) -> String {
        todo!()
    }
}

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
