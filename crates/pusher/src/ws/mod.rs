use std::cell::RefCell;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use crate::adapter::Adapter;
use crate::app::App;
use crate::kind::{Channel, Event};
use crate::messages::{PusherMessage};
use crate::socket::Socket;

use crate::namespace::Namespace;
use crate::ws::messages::{
    ChannelEvent, PresenceMemberRemovedData, PusherSubscribeMessage, PusherUnsubscribeMessage,
};
use crate::{OutgoingMessage, WebSocket};
use actix::prelude::*;
use actix::{Actor, Context, Handler};
use serde_json::json;

mod channel_managers;
mod errors;
mod messages;

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub ws: WebSocket,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
    pub app: App,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Broadcast {
    pub event: String,
    pub channels: Vec<String>,
    pub message: serde_json::Value,
    pub app: App,
    pub except: Option<usize>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct MessageWrapper {
    pub ws: WebSocket,
    pub message: PusherMessage,
}

#[derive(Clone)]
pub struct WebSocketHandler {
    adapter: Arc<Mutex<dyn Adapter>>,
}

impl Actor for WebSocketHandler {
    type Context = Context<Self>;
}

impl WebSocketHandler {
    pub fn new(adapter: Arc<Mutex<dyn Adapter>>) -> Self {
        Self {
            adapter
        }
    }

    fn subscribe(
        &mut self,
        id: usize,
        app: App,
        recipient: Recipient<OutgoingMessage>,
        m: PusherSubscribeMessage,
    ) {
        let mut adapter = self.adapter.lock().unwrap();

        let ns = adapter.namespace(&app.id);

        // // do authentication then
        // if let Err(e) = result {
        //     ws.conn.send(OutgoingMessage(e.to_string()));
        //
        //     return;
        // }

        match m.channel {
            Channel::Presence(_) => {
                recipient
                    .do_send(ChannelEvent::presence_sub_succeeded(
                        &m.channel,
                        ns.channel_members(&m.channel)
                            .iter()
                            .map(|(_, m)| m.clone())
                            .collect()
                    ))
                    .unwrap();
            }
            _ => {
                recipient
                    .do_send(ChannelEvent::sub_succeeded(&m.channel))
                    .unwrap();
            }
        };

        ns.add_socket(id, Clone::clone(&recipient));

        let count = ns.add_to_channel(id, &m.channel, m.channel_data);

        if matches!(m.channel, Channel::Presence(_)) {
            let presence_data = ns.get_presence_data(id, &m.channel).unwrap();

            for (recipient_id, recipient) in ns.channel_sockets(&m.channel) {
                if id != recipient_id {
                    recipient
                        .do_send(ChannelEvent::member_added(
                            &m.channel,
                            presence_data.clone()
                        ))
                        .unwrap();
                }
            }
        }
    }

    fn unsubscribe(
        &mut self,
        id: usize,
        app: App,
        _recipient: Recipient<OutgoingMessage>,
        m: PusherUnsubscribeMessage,
    ) {
        self.adapter.lock()
            .unwrap()
            .namespace(&app.id)
            .remove_from_channel(id, &m.channel);

        self.notify_unsubscribed(id, &app.id, &m.channel);
    }

    fn notify_unsubscribed(&self, id: usize, app_id: &String, channel: &Channel) {
        let mut adapter = self.adapter.lock().unwrap();

        let ns = adapter.namespace(app_id);

        if matches!(channel, Channel::Presence(_)) {
            for (recipient_id, recipient) in ns.channel_sockets(&channel) {
                if id != recipient_id {
                    recipient
                        .do_send(ChannelEvent::member_removed(&channel, id))
                        .unwrap();
                }
            }
        }
    }

    fn pong(&self, ws: WebSocket) {
        ws.conn.do_send(ChannelEvent::pong()).unwrap();
    }

    fn handle_client_event(&self, _ws: WebSocket) {
        todo!()
    }
}

impl Handler<Connect> for WebSocketHandler {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        self.adapter
            .lock()
            .unwrap()
            .namespace(&msg.ws.app.id)
            .add_socket(msg.ws.id, Clone::clone(&msg.ws.conn));

        let socket_id = Socket::default().id;

        msg.ws
            .conn
            .do_send(ChannelEvent::connection_established(socket_id, 30))
            .unwrap();

        socket_id
    }
}

impl Handler<Disconnect> for WebSocketHandler {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Self::Context) -> Self::Result {
        let mut adapter = self.adapter
            .lock()
            .unwrap();

        let mut ns = adapter.namespace(&msg.app.id);

        let channels = ns.channels_for_member(msg.id);

        for channel in &channels {
            ns.remove_from_channel(msg.id, &channel);
        }

        drop(adapter);

        for channel in &channels {
            self.notify_unsubscribed(msg.id, &msg.app.id, &channel);
        }

        self.adapter.lock()
            .unwrap()
            .namespace(&msg.app.id)
            .remove_socket(msg.id);
    }
}

impl Handler<MessageWrapper> for WebSocketHandler {
    type Result = ();

    fn handle(&mut self, msg: MessageWrapper, _ctx: &mut Self::Context) -> Self::Result {
        let event = Event::from(msg.message.event);

        match event {
            Event::Ping => {
                self.pong(msg.ws);
            }
            Event::Subscribe => {
                self.subscribe(
                    msg.ws.id,
                    msg.ws.app,
                    msg.ws.conn,
                    PusherSubscribeMessage {
                        channel: Channel::from(msg.message.data.channel),
                        channel_data: msg.message.data.channel_data,
                        auth: msg.message.data.auth,
                    },
                );
            }
            Event::Unsubscribe => {
                self.unsubscribe(
                    msg.ws.id,
                    msg.ws.app,
                    msg.ws.conn,
                    PusherUnsubscribeMessage {
                        channel: Channel::from(msg.message.data.channel),
                    },
                );
            }
            Event::Client(_) => {
                self.handle_client_event(msg.ws);
            }
            Event::Unknown(_) => {
                todo!();
            }
            Event::Invalid => {
                todo!();
            }
        };
    }
}

impl Handler<Broadcast> for WebSocketHandler {
    type Result = ();

    fn handle(&mut self, msg: Broadcast, ctx: &mut Self::Context) -> Self::Result {
        let mut adapter = self.adapter
            .lock()
            .unwrap();

        let mut ns = adapter.namespace(&msg.app.id);

        for channel in msg.channels {
            let sockets = ns.channel_sockets(&Channel::from(channel));

            for socket in sockets.values() {
                socket.do_send(OutgoingMessage(
                    json!({
                        "event": msg.event.clone(),
                        "data": msg.message.to_string(),
                    })
                        .to_string()
                ))
                    .unwrap();
            }
        }

    }
}
