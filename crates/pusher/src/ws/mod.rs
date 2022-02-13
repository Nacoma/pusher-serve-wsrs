use crate::adapter::Adapter;
use crate::app::App;
use crate::kind::{Channel, Event};
use crate::messages::PusherMessage;
use crate::socket::Socket;

use std::sync::Arc;

use crate::messages::{JsonMessage, OutgoingMessage};
use crate::ws::errors::{ErrorKind, WsError};
use crate::ws::messages::{ChannelEvent, PusherSubscribeMessage, PusherUnsubscribeMessage};
use crate::{AppRepo, WebSocket};
use actix::prelude::*;
use actix::{Actor, Context, Handler};
use log::trace;
use parking_lot::Mutex;
use serde::Serialize;

mod channel_managers;
mod errors;
mod messages;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
    pub app_id: i64,
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
#[rtype(result = "Result<(), Box<dyn WsError>>")]
pub struct MessageWrapper {
    pub ws: WebSocket,
    pub message: PusherMessage,
}

#[derive(Clone)]
pub struct WebSocketHandler {
    adapter: Arc<dyn Adapter>,
    repo: Arc<Mutex<dyn AppRepo>>,
}

impl Actor for WebSocketHandler {
    type Context = Context<Self>;
}

impl WebSocketHandler {
    pub fn new(adapter: Arc<dyn Adapter>, repo: Arc<Mutex<dyn AppRepo>>) -> Self {
        Self { adapter, repo }
    }

    fn subscribe(
        &mut self,
        id: usize,
        app: App,
        recipient: Recipient<OutgoingMessage>,
        m: PusherSubscribeMessage,
    ) {
        let ns = self.adapter.namespace(app.id);

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
                            .collect(),
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

        let _count = ns.add_to_channel(id, &m.channel, m.channel_data);

        if matches!(m.channel, Channel::Presence(_)) {
            let presence_data = ns.get_presence_data(id, &m.channel).unwrap();

            for (recipient_id, recipient) in ns.channel_sockets(&m.channel) {
                println!("wanna send: {}", recipient_id);
                if id != recipient_id {
                    println!("is sending");
                    recipient
                        .do_send(ChannelEvent::member_added(
                            &m.channel,
                            presence_data.clone(),
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
        self.adapter
            .namespace(app.id)
            .remove_from_channel(id, &m.channel);

        self.notify_unsubscribed(id, app.id, &m.channel);
    }

    fn notify_unsubscribed(&self, id: usize, app_id: i64, channel: &Channel) {
        trace!("{}: begin notify unsubscribed to channel:{}", id, channel.to_string());

        let ns = self.adapter.namespace(app_id);

        trace!("{}: checking if presence channel:{}", id, channel.to_string());

        if matches!(channel, Channel::Presence(_)) {
            let recipients = ns.channel_sockets(channel);

            for (recipient_id, recipient) in recipients {
                if id != recipient_id {
                    trace!("{}: notifying unsubscribed to {}", id, recipient_id);
                    recipient
                        .try_send(ChannelEvent::member_removed(channel, id))
                        .unwrap();
                    trace!("{}: notified unsubscribed to {}", id, recipient_id);
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

#[derive(Message)]
#[rtype(result = "Result<usize, Box<dyn WsError>>")]
pub struct Connect {
    pub ws: WebSocket,
}

impl Handler<Connect> for WebSocketHandler {
    type Result = Result<usize, Box<dyn WsError>>;

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        let app = self.repo.lock().find_by_id(msg.ws.app_id);

        if let Some(app) = app {
            let id = Socket::default().id;

            let ns = self.adapter.namespace(app.id);

            ns.add_socket(msg.ws.id, Clone::clone(&msg.ws.conn));

            msg.ws
                .conn
                .try_send(ChannelEvent::connection_established(id, 30))
                .unwrap();

            Ok(id)
        } else {
            Err(Box::new(ErrorKind::AppNotFound))
        }
    }
}

impl Handler<Disconnect> for WebSocketHandler {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Self::Context) -> Self::Result {
        let ns = self.adapter.namespace(msg.app_id);

        let channels = ns.channels_for_member(msg.id);

        for channel in &channels {
            ns.remove_from_channel(msg.id, channel);
        }

        drop(ns);

        for channel in &channels {
            self.notify_unsubscribed(msg.id, msg.app_id, channel);
        }

        self.adapter.namespace(msg.app_id).remove_socket(msg.id);
    }
}

impl Handler<MessageWrapper> for WebSocketHandler {
    type Result = Result<(), Box<dyn WsError>>;

    fn handle(&mut self, msg: MessageWrapper, _ctx: &mut Self::Context) -> Self::Result {
        let event = Event::from(msg.message.event);

        let app = self.repo.lock().find_by_id(msg.ws.app_id);

        if let Some(app) = app {
            match event {
                Event::Ping => {
                    self.pong(msg.ws);
                }
                Event::Subscribe => {
                    self.subscribe(
                        msg.ws.id,
                        app,
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
                        app,
                        msg.ws.conn,
                        PusherUnsubscribeMessage {
                            channel: Channel::from(msg.message.data.channel),
                        },
                    );
                }
                Event::Client(_) => {
                    self.handle_client_event(msg.ws);
                }
                Event::Invalid => {
                    todo!();
                }
            };

            Ok(())
        } else {
            msg.ws.conn.do_send(ErrorKind::AppNotFound.msg()).unwrap();

            Err(Box::new(ErrorKind::AppNotFound))
        }
    }
}

#[derive(Serialize, JsonMessage)]
pub struct OutgoingBroadcast {
    channel: String,
    event: String,
    data: String,
}

impl Handler<Broadcast> for WebSocketHandler {
    type Result = ();

    fn handle(&mut self, msg: Broadcast, _ctx: &mut Self::Context) -> Self::Result {
        let ns = self.adapter.namespace(msg.app.id);

        for channel in msg.channels {
            let sockets = ns.channel_sockets(&Channel::from(channel.clone()));

            for socket in sockets.values() {
                socket
                    .do_send(OutgoingMessage(Box::new(OutgoingBroadcast {
                        channel: channel.clone(),
                        event: msg.event.clone(),
                        data: msg.message.to_string(),
                    })))
                    .unwrap();
            }
        }
    }
}
