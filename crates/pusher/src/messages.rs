use crate::socket::Socket;

use actix::prelude::*;
use serde::{Serialize, Deserialize};
use std::convert::TryFrom;

#[derive(Message)]
#[rtype(result = "()")]
pub struct OutgoingMessage(pub String);

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<OutgoingMessage>,
    pub app: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: Socket,
}

pub struct MessageWrapper {
    pub socket: Socket,
    pub message: Message,
}

#[derive(Serialize, Deserialize)]
pub enum Message {
    Ping,
    Subscribe(SubscribePayload),
    Unsubscribe(UnsubscribePayload),
    Broadcast(BroadcastPayload),
    Unknown(String, serde_json::Value),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubscribePayload {
    pub channel: String,
    pub auth: Option<String>,
    #[serde(default, with = "serde_with::json::nested")]
    pub channel_data: Option<String>,
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

struct InterimMessage {
    event: String,
    data: serde_json::Value,
    channel: Option<String>
}

impl TryFrom<InterimMessage> for Message {
    type Error = serde_json::Error;

    fn try_from(value: InterimMessage) -> Result<Self, Self::Error> {
        let res = match value.event.as_str() {
            "pusher:ping" => Message::Ping,
            "pusher:subscribe" => Message::Subscribe(
                serde_json::from_value(value.data)?
            ),
            "pusher:unsubscribe" => Message::Unsubscribe(
                serde_json::from_value(value.data)?
            ),
            _ => if value.event.starts_with("client-") {
                Message::Broadcast(BroadcastPayload {
                    event: value.event,
                    data: value.data,
                    channel: value.channel.unwrap(),
                })
            } else {
                Message::Unknown(value.event, value.data)
            }
        };

        Ok(res)
    }
}

#[derive(Serialize, Clone)]
#[serde(tag = "event")]
pub enum SystemMessage {
    #[serde(rename = "pusher:connection_established")]
    PusherConnectionEstablished {
        #[serde(with = "serde_with::json::nested")]
        data: ConnectionEstablishedPayload,
    },
    #[serde(rename = "pusher:error")]
    PusherError { message: String, code: u16 },
}

#[derive(Serialize, Clone)]
pub struct ConnectionEstablishedPayload {
    pub socket_id: Socket,
    pub activity_timeout: u32,
}
