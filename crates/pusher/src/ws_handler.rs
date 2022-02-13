use actix::prelude::*;
use actix::{Actor, Addr, AsyncContext, Running};
use actix_web_actors::ws;
use actix_web_actors::ws::{ProtocolError, WebsocketContext};

use std::time::{Duration, Instant};

use serde::Deserialize;

use crate::messages::{PusherMessage, PusherMessageData};
use crate::{OutgoingMessage, WebSocket};

use crate::ws::{Connect, Disconnect, MessageWrapper, WebSocketHandler};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);


pub struct Session {
    id: usize,
    pub hb: Instant,
    addr: Addr<WebSocketHandler>,
    app_id: i64,
}

impl Session {
    pub fn new(app_id: i64, addr: Addr<WebSocketHandler>) -> Self {
        Self {
            id: 0,
            hb: Instant::now(),
            app_id,
            addr,
        }
    }

    fn start_hb(&self, ctx: &mut WebsocketContext<Self>) {
        let app_id = self.app_id;
        ctx.run_interval(HEARTBEAT_INTERVAL, move |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // notify chat server
                act.addr.do_send(Disconnect {
                    id: act.id,
                    app_id: app_id,
                });

                println!("stopping because missed heartbeat");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
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
                    app_id: self.app_id,
                    channels: vec![],
                    conn: address.recipient(),
                },
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => match res {
                        Ok(id) => act.id = id,
                        Err(e) => {
                            println!("{:?}", e);
                            ctx.stop();
                        }
                    },
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        self.addr.do_send(Disconnect {
            id: self.id,
            app_id: self.app_id,
        });

        Running::Stop
    }
}

impl Handler<OutgoingMessage> for Session {
    type Result = ();

    fn handle(&mut self, msg: OutgoingMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(serde_json::to_string(&msg.0).unwrap());
    }
}

impl StreamHandler<Result<ws::Message, ProtocolError>> for Session {
    fn handle(&mut self, item: Result<ws::Message, ProtocolError>, ctx: &mut Self::Context) {
        match item {
            Ok(msg) => {
                match msg {
                    ws::Message::Text(txt) => {
                        let message: IncomingMessage = serde_json::from_str(&txt).unwrap();

                        if let MessageData::Other(data) = message.data {
                            let pusher_message = MessageWrapper {
                                message: PusherMessage {
                                    data,
                                    name: message.name,
                                    event: message.event,
                                },
                                ws: WebSocket {
                                    channels: vec![],
                                    presence_data: None,
                                    conn: ctx.address().recipient(),
                                    id: self.id,
                                    app_id: self.app_id,
                                },
                            };

                            self.addr
                                .send(pusher_message)
                                .into_actor(self)
                                .then(|res, _, ctx| {
                                    match res {
                                        Ok(res) => match res {
                                            Ok(_) => (),
                                            Err(_e) => ctx.stop(),
                                        },
                                        _ => ctx.stop(),
                                    };

                                    fut::ready(())
                                })
                                .wait(ctx);
                        }
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

#[derive(Debug, Deserialize)]
struct IncomingMessage {
    name: Option<String>,
    event: Option<String>,
    data: MessageData,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum MessageData {
    String(String),
    Other(PusherMessageData),
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::ws_handler::{IncomingMessage, MessageData};

    #[test]
    fn can_deserialize_message_data() {
        let r1 = serde_json::from_str::<MessageData>("\"{}\"");

        assert!(r1.is_ok());

        let r2 = serde_json::from_str::<MessageData>("{}");

        assert!(r2.is_ok());
    }

    #[test]
    fn can_deserialize_ping() {
        let v = json!({
            "event": "pusher:ping",
            "data": {}
        })
        .to_string();

        let result = serde_json::from_str::<IncomingMessage>(v.as_str());

        assert!(result.is_ok());
    }
}
