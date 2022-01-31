use serde::Deserialize;
use crate::OutgoingMessage;
use crate::ws::Broadcast;

#[derive(Debug, Deserialize)]
pub struct Event {
    pub name: String,
    pub data: serde_json::Value,
    pub channels: Option<Vec<String>>,
    pub channel: Option<String>,
    pub socket_id: Option<String>,
}
