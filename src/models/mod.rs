use actix::prelude::Message;
use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "event")]
pub enum SystemEvent {
    #[serde(rename = "pusher:connection_established")]
    PusherConnectionEstablished {
        #[serde(with = "serde_with::json::nested")]
        data: ConnectionEstablishedPayload,
    },
    #[serde(rename = "pusher:error")]
    PusherError { message: String, code: u16 },
}

pub fn deserialize_string_from_number<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNumber {
        String(String),
        Number(i64),
        Float(f64),
    }

    match StringOrNumber::deserialize(deserializer)? {
        StringOrNumber::String(s) => Ok(s),
        StringOrNumber::Number(i) => Ok(i.to_string()),
        StringOrNumber::Float(f) => Ok(f.to_string()),
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "event")]
pub enum ChannelEvent {
    #[serde(rename = "pusher_internal:subscription_succeeded")]
    PusherInternalSubscriptionSucceeded {
        channel: String,
        #[serde(with = "serde_with::json::nested")]
        data: SubscriptionData,
    },
    #[serde(rename = "pusher_internal:member_added")]
    PusherInternalMemberAdded {
        channel: String,
        #[serde(with = "serde_with::json::nested")]
        data: PresenceChannelData,
    },
    #[serde(rename = "pusher_internal:member_removed")]
    PusherInternalMemberRemoved {
        channel: String,
        data: PresenceMemberRemovedData,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PresenceMemberRemovedData {
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum DataType {
    String(String),
    Map(HashMap<String, serde_json::Value>),
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct SubscriptionMessage {
    pub id: usize,
    pub app: String,
    pub event: SubscriptionEvent,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
pub enum SubscriptionEvent {
    #[serde(rename = "pusher:subscribe")]
    Subscribe {
        channel: String,
        auth: Option<String>,
        #[serde(default, with = "serde_with::json::nested")]
        channel_data: Option<PresenceChannelData>,
    },

    #[serde(rename = "pusher:unsubscribe")]
    Unsubscribe { channel: String },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SubscriptionData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence: Option<PresenceInternalData>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PresenceInternalData {
    pub ids: Vec<String>,
    pub hash: HashMap<String, HashMap<String, Value>>,
    pub count: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PresenceChannelData {
    #[serde(deserialize_with = "deserialize_string_from_number")]
    pub user_id: String,
    pub user_info: HashMap<String, Value>,
}

#[derive(Deserialize, Message, Debug)]
#[rtype(result = "()")]
pub struct ClientEvent {
    #[serde(skip_deserializing)]
    pub app: String,
    pub data: DataType,
    pub name: String,
    pub channels: Option<Vec<String>>,
    pub channel: Option<String>,
    pub socket_id: Option<usize>,
}

#[derive(Serialize, Debug)]
pub struct SendClientEvent {
    pub event: String,
    pub channel: String,
    pub data: DataType,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct ConnectionEstablishedPayload {
    pub socket_id: String,
    pub activity_timeout: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TriggerEventResponse {
    pub channels: Vec<Channel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Channel {
    #[serde(skip_serializing)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_count: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_count: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occupied: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Auth {
    auth_key: Option<String>,
    auth_timestamp: Option<u64>,
    auth_version: Option<String>,
    body_md5: Option<String>,
    auth_signature: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: String,
}

#[cfg(test)]
mod tests {
    use crate::models::DataType;

    #[test]
    fn can_deserialize_system_events() {}
}
