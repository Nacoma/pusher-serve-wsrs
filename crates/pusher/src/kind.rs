use crate::{OutgoingMessage, Socket};
use actix::Recipient;
use crate::app::App;

#[derive(Clone)]
pub struct WebSocket {
    pub id: Socket,
    pub conn: Recipient<OutgoingMessage>,
    pub presence_data: Option<String>,
    pub channels: Vec<String>,
    pub app: App,
}

impl WebSocket {
    pub fn new(
        id: Socket,
        conn: Recipient<OutgoingMessage>,
        presence_data: Option<String>,
    ) -> Self {
        Self {
            id,
            conn,
            presence_data,
            channels: Vec::default(),
            app: App::default(),
        }
    }
}
