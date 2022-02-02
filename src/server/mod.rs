use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use actix::prelude::*;
use log::debug;
use rand::{self, rngs::ThreadRng, Rng};

use crate::pusher::socket_id::SocketId;
use crate::pusher::Pusher;
use crate::server::messages::{BroadcastMessage, ClientEventMessage, Connect, Disconnect, Message};

pub mod errors;
pub mod messages;
pub mod routes;
pub mod session;

pub struct Server {
    pusher: Arc<Mutex<Pusher>>,
    sessions: HashMap<usize, Recipient<crate::server::messages::Message>>,
    rng: ThreadRng,
}

pub struct Sendable {
    pub recipients: HashSet<usize>,
    pub message: Box<dyn JsonMessage>,
}

pub trait JsonMessage: erased_serde::Serialize {}

impl Server {
    pub fn new(pusher: Arc<Mutex<Pusher>>) -> Server {
        Server {
            pusher,
            sessions: HashMap::new(),
            rng: rand::thread_rng(),
        }
    }

    pub fn send(&self, s: Sendable) {
        let msg = serde_json::to_string(&s.message).unwrap();

        debug!("sending: {}", msg);

        for id in s.recipients {
            if let Some(session) = self.sessions.get(&id) {
                session.do_send(Message(msg.clone())).unwrap();
            }
        }
    }
}

/// Make actor from `Server`
impl Actor for Server {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}

/// Handler for Connect message.
///
/// Register new session and assign unique id to this session
impl Handler<Connect> for Server {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        // register session with random id
        let id = self.rng.gen::<usize>();

        self.sessions.insert(id, msg.addr);

        let sendable = self
            .pusher
            .clone()
            .lock()
            .unwrap()
            .add_connection(msg.app.to_owned(), SocketId::from(id));

        self.send(sendable);

        id
    }
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        let sendables = self
            .pusher
            .clone()
            .lock()
            .unwrap()
            .remove_connection(msg.id);

        for sendable in sendables {
            self.send(sendable);
        }
    }
}

impl Handler<BroadcastMessage> for Server {
    type Result = ();

    fn handle(&mut self, msg: BroadcastMessage, _: &mut Context<Self>) -> Self::Result {
        let sendables = self.pusher.clone().lock().unwrap().broadcast(msg);

        for sendable in sendables {
            self.send(sendable);
        }
    }
}

impl Handler<ClientEventMessage> for Server {
    type Result = ();

    fn handle(&mut self, msg: ClientEventMessage, _: &mut Context<Self>) {
        let sendables = self
            .pusher
            .clone()
            .lock()
            .unwrap()
            .process_client_event(msg);

        if let Some(sendables) = sendables {
            for sendable in sendables {
                self.send(sendable);
            }
        }
    }
}
