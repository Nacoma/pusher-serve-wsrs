use std::collections::HashMap;

use actix::{Actor, Context, Handler, Recipient};
use serde_json;

use crate::channel::Channel;
use crate::messages::{Connect, ConnectionEstablishedPayload, Disconnect, OutgoingMessage, SystemMessage};
use crate::socket::Socket;

pub mod channel;
mod session_manager;
mod messages;
mod socket;


pub struct Pusher {
    apps: HashMap<String, App>,
    connections: HashMap<usize, Recipient<OutgoingMessage>>,
}

impl Pusher {
    pub fn new() -> Self {
        Pusher {
            apps: HashMap::new(),
            connections: HashMap::new(),
        }
    }
}

impl Actor for Pusher {
    type Context = Context<Self>;
}

impl Handler<Connect> for Pusher {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _ctx: &mut Context<Self>) -> Self::Result {
        let socket = Socket::new();
        msg.addr.do_send(OutgoingMessage(
            serde_json::to_string(
                &SystemMessage::PusherConnectionEstablished {
                    data: ConnectionEstablishedPayload {
                        activity_timeout: 999,
                        socket_id: socket,
                    }
                }
            )
                .unwrap()
        ))
            .unwrap();

        self.connections.insert(socket.id, msg.addr);

        socket.id
    }
}

impl Handler<Disconnect> for Pusher {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Context<Self>) -> Self::Result {
        self.connections.remove(&msg.id.id);
    }
}


pub struct App {
    public: String,
    private: String,
    channels: HashMap<String, Channel>,
}
