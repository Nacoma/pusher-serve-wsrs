use crate::app::App;

use crate::OutgoingMessage;
use actix::Recipient;

#[derive(Clone)]
pub struct WebSocket {
    pub id: usize,
    pub conn: Recipient<OutgoingMessage>,
    pub presence_data: Option<String>,
    pub channels: Vec<String>,
    pub app_id: i64,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Channel {
    Presence(String),
    Private(String),
    Public(String),
    Invalid,
}

impl ToString for Channel {
    fn to_string(&self) -> String {
        match self {
            Channel::Presence(s) => s.clone(),
            Channel::Private(s) => s.clone(),
            Channel::Public(s) => s.clone(),
            Channel::Invalid => "".to_string(),
        }
    }
}

impl From<Option<String>> for Channel {
    fn from(s: Option<String>) -> Self {
        match s {
            None => Self::Invalid,
            Some(s) => Channel::from(s),
        }
    }
}

impl From<String> for Channel {
    fn from(s: String) -> Self {
        if s.starts_with("presence-") {
            Self::Presence(s)
        } else if s.starts_with("private-") {
            Self::Private(s)
        } else {
            Self::Public(s)
        }
    }
}

#[derive(Debug)]
pub enum Event {
    Ping,
    Subscribe,
    Unsubscribe,
    Client(String),
    Invalid,
}

impl From<Option<String>> for Event {
    fn from(s: Option<String>) -> Self {
        match s {
            None => Self::Invalid,
            Some(s) => Self::from(s),
        }
    }
}

impl From<String> for Event {
    fn from(s: String) -> Self {
        match s.as_str() {
            "pusher:ping" => Self::Ping,
            "pusher:subscribe" => Self::Subscribe,
            "pusher:unsubscribe" => Self::Unsubscribe,
            _ => Self::Client(s.clone()),
        }
    }
}
