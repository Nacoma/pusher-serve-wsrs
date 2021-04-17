//! `ChatServer` is an actor. It maintains list of connection client session.
//! And manages available rooms. Peers send messages to other peers in same
//! room through `ChatServer`.

use std::collections::{HashMap, HashSet};

use actix::prelude::*;
use rand::{self, rngs::ThreadRng, Rng};

use crate::models::{
    ChannelEvent, ClientEvent, ConnectionEstablishedPayload, PresenceChannelData,
    PresenceInternalData, PresenceMemberRemovedData, SendClientEvent, SubscriptionData,
    SubscriptionEvent, SubscriptionMessage, SystemEvent,
};

/// Chat server sends this messages to session
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// Message for chat server communications

/// New chat session is created
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
    pub app: String,
}

/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

/// Send message to specific room
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    /// Id of the client session
    pub id: usize,
    /// Peer message
    pub msg: String,
    /// Room name
    pub room: String,
}

/// List of available rooms
pub struct ListRooms;

impl actix::Message for ListRooms {
    type Result = Vec<String>;
}

/// `ChatServer` manages chat rooms and responsible for coordinating chat
/// session. implementation is super primitive
pub struct PusherServer {
    app_keys: HashMap<String, String>,
    apps: HashMap<String, App>,
    rng: ThreadRng,
}

#[derive(Debug)]
struct Channel {
    internal_type: ChannelType,
    sessions_info: HashMap<usize, PresenceChannelData>,
    sessions: HashSet<usize>,
}

impl Channel {
    fn new(internal_type: ChannelType) -> Channel {
        Channel {
            internal_type,
            sessions_info: HashMap::new(),
            sessions: HashSet::new(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum ChannelType {
    Presence,
    Private,
    Public,
}

fn get_channel_type(name: &str) -> ChannelType {
    if name.starts_with("presence-") {
        ChannelType::Presence
    } else if name.starts_with("private-") {
        ChannelType::Private
    } else {
        ChannelType::Public
    }
}

#[derive(Debug)]
struct App {
    sessions: HashMap<usize, Recipient<Message>>,
    channels: HashMap<String, Channel>,
}

impl App {
    pub fn new() -> App {
        App {
            channels: HashMap::new(),
            sessions: HashMap::new(),
        }
    }
}

impl PusherServer {
    pub fn new(app_keys: HashMap<String, String>) -> PusherServer {
        PusherServer {
            app_keys,
            apps: HashMap::new(),
            rng: rand::thread_rng(),
        }
    }
}

impl PusherServer {
    /// Send message to all users in the room
    fn send_message<S>(&self, app: &str, channel: &str, message: S, socket_id: Option<usize>)
    where
    S: serde::Serialize
    {
        let message = serde_json::to_string(&message).unwrap();

        if let Some(app) = self.apps.get(app) {
            if let Some(channel) = app.channels.get(channel) {
                for id in &channel.sessions {
                    if let Some(socket_id) = socket_id {
                        if *id != socket_id {
                            if let Some(addr) = app.sessions.get(id) {
                                let _ = addr.do_send(Message(message.to_owned()));
                            }
                        }
                    } else {
                        if let Some(addr) = app.sessions.get(id) {
                            let _ = addr.do_send(Message(message.to_owned()));
                        }
                    }
                }
            }
        }
    }

    fn send_direct(&self, app: &str, message: &str, socket_id: usize) {
        if let Some(app) = self.apps.get(app) {
            if let Some(addr) = app.sessions.get(&socket_id) {
                let _ = addr.do_send(Message(message.to_owned()));
            }
        }
    }

    fn insert_channel_session(
        &mut self,
        app: &str,
        channel: &str,
        id: usize,
        data: Option<PresenceChannelData>,
    ) {
        let internal_type = get_channel_type(channel);

        let found_channel = self
            .apps
            .entry(app.to_string())
            .or_insert_with(App::new)
            .channels
            .entry(channel.to_owned())
            .or_insert_with(|| Channel::new(internal_type));

        found_channel.sessions.insert(id);

        match internal_type {
            ChannelType::Presence => {
                found_channel.sessions_info.insert(id, data.unwrap());
            }
            _ => {}
        };
    }

    fn remove_channel_session(&mut self, app: &str, channel: &str, id: usize) -> bool {
        let mut found = false;

        if let Some(app) = self.apps.get_mut(app) {
            if let Some(channel) = app.channels.get_mut(channel) {
                found = channel.sessions.remove(&id);
                channel.sessions_info.remove(&id);
            }
        }

        found
    }
}

/// Make actor from `ChatServer`
impl Actor for PusherServer {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}

/// Handler for Connect message.
///
/// Register new session and assign unique id to this session
impl Handler<Connect> for PusherServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        // register session with random id
        let id = self.rng.gen::<usize>();

        self.apps
            .entry(msg.app.to_owned())
            .or_insert_with(App::new)
            .sessions
            .insert(id, msg.addr);

        self.send_direct(
            msg.app.as_str(),
            serde_json::to_string(&SystemEvent::PusherConnectionEstablished {
                data: ConnectionEstablishedPayload {
                    socket_id: format!("{}.{}", id.to_string(), 1),
                    activity_timeout: 9000,
                },
            })
            .unwrap()
            .as_str(),
            id,
        );

        id
    }
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for PusherServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        for (app_name, app) in self.apps.iter() {
            for (channel_name, channel) in app.channels.iter() {
                let message = ChannelEvent::PusherInternalMemberRemoved {
                    channel: channel_name.to_string(),
                    data: PresenceMemberRemovedData {
                        user_id: msg.id.to_string(),
                    },
                };

                if channel.sessions.contains(&msg.id) {
                    match get_channel_type(channel_name.as_str()) {
                        ChannelType::Presence => {
                            self.send_message(
                                app_name.as_str(),
                                channel_name.clone().as_str(),
                                message,
                                Some(msg.id),
                            );
                        }
                        _ => {}
                    };
                }
            }
        }

        for (_, app) in self.apps.iter_mut() {
            app.sessions.remove(&msg.id);

            for (_, channel) in app.channels.iter_mut() {
                channel.sessions.remove(&msg.id);
                channel.sessions_info.remove(&msg.id);
            }
        }
    }
}

impl Handler<ClientEvent> for PusherServer {
    type Result = ();

    fn handle(&mut self, msg: ClientEvent, _: &mut Context<Self>) -> Self::Result {
        let app = self.app_keys.get(&msg.app.to_string()).unwrap();

        let mut channels: Vec<&String> = vec![];

        if let Some(ref channel) = msg.channel {
            channels.push(channel);
        }

        if let Some(ref _channels) = msg.channels {
            for channel in _channels {
                channels.push(channel);
            }
        }

        for channel in channels {
            self.send_message(
                app.as_str(),
                channel.as_str(),
                SendClientEvent {
                    channel: channel.to_string(),
                    event: msg.name.clone(),
                    data: msg.data.clone(),
                },
                None,
            );
        }
    }
}

impl Handler<SubscriptionMessage> for PusherServer {
    type Result = ();

    fn handle(&mut self, msg: SubscriptionMessage, _: &mut Context<Self>) {
        match msg.event {
            SubscriptionEvent::Unknown => {
                panic!("unknown subscription event");
            }

            SubscriptionEvent::Subscribe {
                channel_data,
                auth: _,
                channel,
            } => {
                self.insert_channel_session(
                    msg.app.as_str(),
                    channel.as_str(),
                    msg.id,
                    channel_data.clone(),
                );

                let internal_type = get_channel_type(channel.as_str());

                let data = match internal_type {
                    ChannelType::Presence => {
                        let channel_data = channel_data.unwrap();

                        self.send_message(
                            msg.app.as_str(),
                            channel.as_str(),
                            ChannelEvent::PusherInternalMemberAdded {
                                data: PresenceChannelData {
                                    user_id: msg.id.to_string(),
                                    user_info: channel_data.user_info.clone(),
                                },
                                channel: channel.to_string(),
                            },
                            Some(msg.id),
                        );

                        let channel = self
                            .apps
                            .get(&msg.app.to_string())
                            .unwrap()
                            .channels
                            .get(&channel.to_string())
                            .unwrap();

                        let mut presence = PresenceInternalData {
                            ids: vec![],
                            hash: HashMap::new(),
                            count: channel.sessions.len(),
                        };

                        for (id, info) in &channel.sessions_info {
                            presence.ids.push(id.to_string());
                            presence.hash.insert(id.to_string(), info.user_info.clone());
                        }

                        SubscriptionData {
                            presence: Some(presence),
                        }
                    }
                    _ => SubscriptionData { presence: None },
                };

                self.send_direct(
                    msg.app.as_str(),
                    serde_json::to_string(&ChannelEvent::PusherInternalSubscriptionSucceeded {
                        channel: channel.clone(),
                        data,
                    })
                    .unwrap()
                    .as_str(),
                    msg.id,
                );
            }

            SubscriptionEvent::Unsubscribe {
                channel: channel_name,
            } => {
                let found =
                    self.remove_channel_session(&msg.app, channel_name.clone().as_str(), msg.id);

                if found {
                    match get_channel_type(channel_name.as_str()) {
                        ChannelType::Presence => {
                            self.send_message(
                                msg.app.as_str(),
                                channel_name.clone().as_str(),
                                ChannelEvent::PusherInternalMemberRemoved {
                                    channel: channel_name.to_string(),
                                    data: PresenceMemberRemovedData {
                                        user_id: msg.id.to_string(),
                                    },
                                },
                                Some(msg.id),
                            );
                        }
                        _ => {}
                    };
                }
            }
        }
    }
}
