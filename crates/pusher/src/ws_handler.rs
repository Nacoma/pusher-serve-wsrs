use std::sync::Arc;
use actix::prelude::*;
use actix::{Actor, Addr, AsyncContext, Running};
use actix_web_actors::ws;
use actix_web_actors::ws::{ProtocolError, WebsocketContext};
use std::time::Instant;

use crate::app::App;
use crate::messages::PusherMessage;
use crate::{OutgoingMessage, WebSocket};

use crate::ws::{Connect, Disconnect, MessageWrapper, WebSocketHandler};

pub struct Session {
    id: usize,
    pub hb: Instant,
    addr: Addr<WebSocketHandler>,
    app: App,
}

impl Session {
    pub fn new(app: App, addr: Addr<WebSocketHandler>) -> Self {
        Self {
            id: 0,
            hb: Instant::now(),
            app,
            addr,
        }
    }

    fn start_hb(&self, _ctx: &mut WebsocketContext<Self>) {
        // todo
    }
}

impl Actor for Session {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.start_hb(ctx);

        let address = ctx.address();

        self.addr
            .send(Connect {
                ws: WebSocket {
                    presence_data: None,
                    id: self.id,
                    app: self.app.clone(),
                    channels: vec![],
                    conn: address.recipient(),
                },
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    // something is wrong with chat server
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        self.addr.do_send(Disconnect {
            id: self.id,
            app: App {
                id: self.app.id.clone(),
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
                        let message: PusherMessage = serde_json::from_str(&txt).unwrap();

                        self.addr
                            .send(MessageWrapper {
                                message,
                                ws: WebSocket {
                                    channels: vec![],
                                    presence_data: None,
                                    conn: ctx.address().recipient(),
                                    id: self.id,
                                    app: self.app.clone(),
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
