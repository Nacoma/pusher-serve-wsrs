use actix::{Actor, Context, Handler};

use crate::kind::WebSocket;
use crate::messages::{Connect, Disconnect, MessagePayload, OutgoingMessage};
use crate::socket::Socket;

mod adapter;
mod channel_managers;
mod kind;
mod messages;
mod namespace;
mod session_manager;
mod socket;
mod ws_handler;
mod app;
mod auth;

#[derive(Default)]
pub struct Pusher {}

impl Pusher {
    pub fn send(&self, _recipients: &[usize], _payload: Box<dyn MessagePayload>) {
        todo!();
    }
}

impl Actor for Pusher {
    type Context = Context<Self>;
}

impl Handler<Connect> for Pusher {
    type Result = usize;

    fn handle(&mut self, _msg: Connect, _ctx: &mut Context<Self>) -> Self::Result {
        todo!();
    }
}

impl Handler<Disconnect> for Pusher {
    type Result = ();

    fn handle(&mut self, _msg: Disconnect, _ctx: &mut Context<Self>) -> Self::Result {
        todo!();
    }
}
