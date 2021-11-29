pub mod responses;

use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

use crate::server::messages::{UserInfo};
use crate::server::JsonMessage;

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

impl JsonMessage for SystemEvent {}

serialize_trait_object!(JsonMessage);

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
        data: UserInfo,
    },
    #[serde(rename = "pusher_internal:member_removed")]
    PusherInternalMemberRemoved {
        channel: String,
        data: PresenceMemberRemovedData,
    },
}

impl JsonMessage for ChannelEvent {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PresenceMemberRemovedData {
    pub user_id: String,
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

#[derive(Serialize, Debug)]
pub struct SendClientEvent {
    pub event: String,
    pub channel: String,
    pub data: serde_json::Value,
}

impl JsonMessage for SendClientEvent {}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct ConnectionEstablishedPayload {
    #[serde(serialize_with = "serialize_socket_id")]
    pub socket_id: usize,
    pub activity_timeout: u32,
}

fn serialize_socket_id<S>(v: &usize, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer {
    let mut v = v.to_string();

    v.insert(4, '.');

    s.serialize_str(v.as_str())
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

#[derive(Queryable, Serialize, Debug)]
pub struct AppModel {
    pub id: i32,
    pub key: String,
    pub secret: String,
    pub name: String,
}

use crate::schema::apps;
#[derive(Insertable, Debug)]
#[table_name="apps"]
pub struct NewApp<'a> {
    pub key:  &'a str,
    pub name: &'a str,
    pub secret: &'a str,
}
