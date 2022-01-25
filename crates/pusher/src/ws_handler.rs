use std::borrow::{Borrow, BorrowMut};
use actix::prelude::*;
use actix::{Actor, Addr, AsyncContext, Context, Running};
use actix_web_actors::ws;
use actix_web_actors::ws::{ProtocolError, WebsocketContext};
use std::time::Instant;

use crate::messages::{ClientEvent, PusherMessage, SubscribePayload};
use crate::{OutgoingMessage, Socket, WebSocket};
use crate::adapter::Adapter;
use crate::app::App;
use crate::channel_managers::{ChannelManger, PresenceChannelManager, PrivateChannelManager, PublicChannelManager};

pub struct Session {
    id: Socket,
    pub hb: Instant,
    service: Addr<WebSocketHandler>,
    conn: Recipient<OutgoingMessage>,
    app: String,
}

impl Session {
    fn start_hb(&self, _ctx: &mut WebsocketContext<Self>) {
        todo!();
    }
}

impl Actor for Session {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.start_hb(ctx);

        let address = ctx.address();

        self.service.send(Connect(WebSocket {
            presence_data: None,
            id: self.id,
            app: App {
                id: self.app.clone(),
                key: "".to_string(),
                secret: "".to_string(),
            },
            channels: vec![],
            conn: address.recipient(),
        }));
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        self.service.do_send(Disconnect {
            id: self.id,
            app: App {
                id: self.app.clone(),
                key: "".to_string(),
                secret: "".to_string(),
            },
        });

        Running::Stop
    }
}

impl Handler<OutgoingMessage> for Session {
    type Result = ();

    fn handle(&mut self, msg: OutgoingMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ProtocolError>> for Session {
    fn handle(&mut self, item: Result<ws::Message, ProtocolError>, ctx: &mut Self::Context) {
        match item {
            Ok(msg) => {
                match msg {
                    ws::Message::Text(txt) => {
                        let event: ClientEvent = serde_json::from_str(&txt).unwrap();

                        self.service
                            .send(Event {
                                event,
                                ws: WebSocket {
                                    channels: vec![],
                                    presence_data: None,
                                    conn: ctx.address().recipient(),
                                    id: self.id,
                                    app: App {
                                        id: self.app.clone(),
                                        key: "".to_string(),
                                        secret: "".to_string(),
                                    },
                                },
                            })
                            .into_actor(self)
                            .then(|_, _, _| fut::ready(()))
                            .wait(ctx);
                    }
                    ws::Message::Continuation(_) => {
                        ctx.stop();
                    }
                    ws::Message::Ping(ping) => {
                        self.hb = Instant::now();
                        ctx.pong(&ping);
                    }
                    ws::Message::Pong(_) => {
                        self.hb = Instant::now();
                    }
                    ws::Message::Close(reason) => {
                        ctx.close(reason);
                        ctx.stop();
                    }
                    _ => (),
                };
            }
            Err(_) => {
                ctx.stop();
            }
        };
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect(WebSocket);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    id: Socket,
    app: App,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Event {
    ws: WebSocket,
    event: ClientEvent,
}

pub struct WebSocketHandler {

}

impl WebSocketHandler {
    fn channel_manager(&self, channel: &String) -> Box<dyn ChannelManger> {
        return if channel.starts_with("presence-") {
            Box::new(PresenceChannelManager {})
        } else if channel.starts_with("private-") {
            Box::new(PrivateChannelManager {})
        } else {
            Box::new(PublicChannelManager {})
        }
    }

    fn subscribe_to_channel(&self, adapter: Box<dyn Adapter>, ws: WebSocket, payload: PusherMessage) {
        let channel_manager = self.channel_manager(&payload.channel.as_ref().unwrap());

        let result = channel_manager.join(&adapter, Clone::clone(&ws), payload);

        if result.success {
            adapter.namespace(&ws.app.id).add_socket(ws);
        }
    }
}

impl Actor for WebSocketHandler {
    type Context = Context<Self>;
}

impl Handler<Connect> for WebSocketHandler {
    type Result = ();

    fn handle(&mut self, _msg: Connect, _ctx: &mut Self::Context) -> Self::Result {}
}

impl Handler<Disconnect> for WebSocketHandler {
    type Result = ();

    fn handle(&mut self, _msg: Disconnect, _ctx: &mut Self::Context) -> Self::Result {
        todo!()
    }
}

impl Handler<Event> for WebSocketHandler {
    type Result = ();

    fn handle(&mut self, _msg: Event, _ctx: &mut Self::Context) -> Self::Result {
        todo!();
    }
}
