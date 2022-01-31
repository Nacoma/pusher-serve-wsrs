use crate::kind::Channel;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::OutgoingMessage;
use std::collections::HashMap;
use serde_json::json;
use crate::messages::PusherMessageChannelData;

pub struct PusherSubscribeMessage {
    pub channel: Channel,
    pub auth: Option<String>,
    pub channel_data: Option<PusherMessageChannelData>,
}

pub struct PusherUnsubscribeMessage {
    pub channel: Channel,
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
        data: PusherMessageChannelData,
    },
    #[serde(rename = "pusher_internal:member_removed")]
    PusherInternalMemberRemoved {
        channel: String,
        data: PresenceMemberRemovedData,
    },
    #[serde(rename = "pusher:connection_established")]
    PusherConnectionEstablished {
        data: ConnectionEstablishedData,
    },
    #[serde(rename = "pusher:pong")]
    Ping {
        data: serde_json::Value,
    }
}

impl ChannelEvent {
    pub fn connection_established(socket_id: usize, activity_timeout: u32) -> OutgoingMessage {
        ChannelEvent::PusherConnectionEstablished {
            data: ConnectionEstablishedData {
                activity_timeout,
                socket_id,
            },
        }
        .msg()
    }

    pub fn pong() -> OutgoingMessage {
        ChannelEvent::Ping {
            data: json!({}),
        }
            .msg()
    }

    pub fn presence_sub_succeeded(
        channel: &Channel,
        members: Vec<PusherMessageChannelData>,
    ) -> OutgoingMessage {
        let count = members.len();

        let ids: Vec<String> = members
            .iter()
            .map(|m| m.user_id.clone())
            .collect();

        let hash: HashMap<String, serde_json::Value> = members.iter()
            .map(|m| (m.user_id.clone(), m.user_info.clone()))
            .collect();

        Self::PusherInternalSubscriptionSucceeded {
            channel: channel.to_string(),
            data: SubscriptionData {
                presence: Some(PresenceInternalData { count, ids, hash }),
            },
        }
        .msg()
    }

    pub fn sub_succeeded(channel: &Channel) -> OutgoingMessage {
        Self::PusherInternalSubscriptionSucceeded {
            channel: channel.to_string(),
            data: SubscriptionData { presence: None },
        }
        .msg()
    }

    pub fn member_added(channel: &Channel, data: PusherMessageChannelData) -> OutgoingMessage {
        Self::PusherInternalMemberAdded {
            channel: channel.to_string(),
            data,
        }
        .msg()
    }

    pub fn member_removed(channel: &Channel, id: usize) -> OutgoingMessage {
        Self::PusherInternalMemberRemoved {
            channel: channel.to_string(),
            data: PresenceMemberRemovedData {
                user_id: id.to_string(),
            },
        }
        .msg()
    }

    fn msg(&self) -> OutgoingMessage {
        OutgoingMessage(self.to_string())
    }
}

impl ToString for ChannelEvent {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

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
    pub hash: HashMap<String, serde_json::Value>,
    pub count: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PresenceChannelData {
    #[serde(deserialize_with = "deserialize_string_from_number")]
    pub user_id: String,
    pub user_info: HashMap<String, String>,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct ConnectionEstablishedData {
    #[serde(serialize_with = "serialize_socket_id")]
    pub socket_id: usize,
    pub activity_timeout: u32,
}

fn serialize_socket_id<S>(v: &usize, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut v = v.to_string();

    v.insert(4, '.');

    s.serialize_str(v.as_str())
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

#[derive(Serialize)]
pub struct PusherBasicResponse {
    event: String,
    data: Option<String>,
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn presence_serialize() {
        let mut hash = HashMap::default();
        let count = 1;
        let ids = vec![1];

        let ce = ChannelEvent::PusherInternalSubscriptionSucceeded {
            channel: "presence-some".to_string(),
            data: SubscriptionData {
                presence: Some(PresenceInternalData { count, ids, hash }),
            },
        };

        println!("{:?}", ce);

        let result = serde_json::to_value(ce).unwrap();

        let t = serde_json::to_string(&json!({
            "what": "hello",
        })).unwrap();

        let expected = json!({
            "event": "pusher_internal:subscription_succeeded",
            "channel": "presence-some",
            "data": json!({
                "presence": {
                    "ids": [1],
                    "count": 1,
                    "hash": {"1": {}},
                }
            })
            .to_string()
        });

        assert_eq!(expected, result);
    }
}
