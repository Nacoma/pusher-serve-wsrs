use actix::prelude::*;
use std::convert::{TryFrom};
use serde::de::Visitor;
use std::fmt::Formatter;
use serde::de::{Deserializer};
use serde::{Serialize, Deserialize};

/// Chat server sends this messages to session
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
    pub app: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct ClientEventMessage {
    pub id: usize,
    pub app: String,
    pub message: ClientEvent,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(try_from = "InterimClientEvent")]
pub enum ClientEvent {
    Subscribe(SubscribePayload),
    Unsubscribe(UnsubscribePayload),
    Broadcast(BroadcastPayload),
    Unknown(String, serde_json::Value),
    Ping,
}

#[derive(Deserialize, Debug)]
struct InterimClientEvent {
    event: String,
    data: serde_json::Value,
    channel: Option<String>,
}

impl TryFrom<InterimClientEvent> for ClientEvent {
    type Error = serde_json::Error;

    fn try_from(value: InterimClientEvent) -> Result<Self, Self::Error> {
        let res = match value.event.as_str() {
            "pusher:ping" => ClientEvent::Ping,
            "pusher:subscribe" => ClientEvent::Subscribe(
                serde_json::from_value(value.data)?
            ),
            "pusher:unsubscribe" => ClientEvent::Unsubscribe(
                serde_json::from_value(value.data)?
            ),
            _ => if value.event.starts_with("client-") {
                ClientEvent::Broadcast(BroadcastPayload {
                    event: value.event,
                    data: value.data,
                    channel: value.channel.unwrap(),
                })
            } else {
                ClientEvent::Unknown(
                    value.event,
                    value.data
                )
            }
        };

        Ok(res)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubscribePayload {
    pub channel: String,
    pub auth: Option<String>,
    #[serde(default, with = "serde_with::json::nested")]
    pub channel_data: Option<UserInfo>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UnsubscribePayload {
    pub channel: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BroadcastPayload {
    pub channel: String,
    pub data: serde_json::Value,
    pub event: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserInfo {
    pub user_id: String,
    pub user_info: serde_json::Value,
}

#[derive(Deserialize, Message, Debug)]
#[rtype(result = "()")]
pub struct BroadcastMessage {
    #[serde(skip_deserializing)]
    pub app: String,
    pub data: serde_json::Value,
    pub name: String,
    pub channels: Option<Vec<String>>,
    pub channel: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_socket_id")]
    pub socket_id: Option<usize>,
}


struct SocketIdVisitor;

impl<'de> Visitor<'de> for SocketIdVisitor {
    type Value = Option<usize>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("expected integer or ####.####")
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        if value >= usize::MAX as i64 || value <= usize::MIN as i64 {
            Err(E::custom(format!("integer out of range: {}", value)))
        } else {
            Ok(Some(value as usize))
        }
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        if value >= usize::MAX as u64 || value <= usize::MIN as u64 {
            Err(E::custom(format!("integer out of range: {}", value)))
        } else {
            Ok(Some(value as usize))
        }
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        Ok(Some(value.replace(".", "").parse::<usize>().unwrap()))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        Ok(None)
    }
}

fn deserialize_socket_id<'de, D>(deserializer: D) -> Result<Option<usize>, D::Error>
    where
        D: Deserializer<'de>,
{
    deserializer.deserialize_any(SocketIdVisitor)
}

#[cfg(test)]
mod tests {
    use super::BroadcastMessage;

    #[test]
    fn can_deserialize_socket_id_as_string() {
        let e: BroadcastMessage = serde_json::from_str(
            r#"{
            "app": "what",
            "data": "asdfasdf",
            "name": "asdfasfasdf",
            "socket_id": "1234.1234"
        }"#,
        )
            .unwrap();

        assert!(Option::is_some(&e.socket_id));
        assert_eq!(12341234, e.socket_id.unwrap());
    }

    #[test]
    fn can_deserialize_socket_id_as_number() {
        let e: BroadcastMessage = serde_json::from_str(
            r#"{
            "app": "what",
            "data": "asdfasdf",
            "name": "asdfasfasdf",
            "socket_id": 12341234
        }"#,
        )
            .unwrap();

        assert!(Option::is_some(&e.socket_id));
        assert_eq!(12341234, e.socket_id.unwrap());
    }

    #[test]
    fn can_deserialize_socket_id_as_none() {
        let e: BroadcastMessage = serde_json::from_str(
            r#"{
            "app": "what",
            "data": "asdfasdf",
            "name": "asdfasfasdf"
        }"#,
        )
            .unwrap();

        assert!(Option::is_none(&e.socket_id));
    }
}
