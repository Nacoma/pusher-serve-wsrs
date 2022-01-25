use crate::socket::Socket;

use crate::WebSocket;
use actix::prelude::*;
use erased_serde::serialize_trait_object;
use pusher_message_derive::MessagePayload;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

pub trait MessagePayload: erased_serde::Serialize {}

serialize_trait_object!(MessagePayload);

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

#[derive(Message)]
#[rtype(result = "()")]
pub struct Subscribe {
    pub ws: WebSocket,
    pub channel: String,
    pub auth: Option<String>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct MessageWrapper {
    pub ws: WebSocket,
    pub message: ClientEvent,
}

#[derive(Serialize, Deserialize)]
pub enum ClientEvent {
    Ping,
    Subscribe(SubscribePayload),
    Unsubscribe(UnsubscribePayload),
    Broadcast(BroadcastPayload),
    Unknown(String, serde_json::Value),
}

pub struct PusherMessage {
    pub channel:  Option<String>,
    pub name: Option<String>,
    pub event: Option<String>,
    pub data: MessageData,
}

pub struct MessageData {
    pub channel_data: Option<String>,
    pub channel: Option<String>,
    pub auth: Option<String>,
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
    channel: Option<String>,
}

impl TryFrom<InterimMessage> for ClientEvent {
    type Error = serde_json::Error;

    fn try_from(value: InterimMessage) -> Result<Self, Self::Error> {
        let res = match value.event.as_str() {
            "pusher:ping" => ClientEvent::Ping,
            "pusher:subscribe" => ClientEvent::Subscribe(serde_json::from_value(value.data)?),
            "pusher:unsubscribe" => ClientEvent::Unsubscribe(serde_json::from_value(value.data)?),
            _ => {
                if value.event.starts_with("client-") {
                    ClientEvent::Broadcast(BroadcastPayload {
                        event: value.event,
                        data: value.data,
                        channel: value.channel.unwrap(),
                    })
                } else {
                    ClientEvent::Unknown(value.event, value.data)
                }
            }
        };

        Ok(res)
    }
}

#[derive(Serialize, MessagePayload, Clone)]
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

#[derive(Serialize, MessagePayload, Clone)]
pub struct ConnectionEstablishedPayload {
    pub socket_id: Socket,
    pub activity_timeout: u32,
}
