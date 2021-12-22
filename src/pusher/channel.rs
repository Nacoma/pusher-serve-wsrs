use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

use crate::models::{AppModel, ChannelEvent, PresenceInternalData, PresenceMemberRemovedData, SendClientEvent, SubscriptionData};
use crate::pusher::messages::Broadcast;
use crate::pusher::socket_id::SocketId;
use crate::server::{Sendable};
use crate::server::messages::{SubscribePayload, UserInfo};
use pusher_credentials::Key;

#[derive(Debug)]
pub struct Channel {
    pub name: String,
    pub internal_type: ChannelType,
    pub sessions_info: HashMap<usize, UserInfo>,
    pub sessions: HashSet<usize>,
}

impl Channel {
    pub fn new(name: String) -> Channel {
        Channel {
            name: name.clone(),
            internal_type: ChannelType::which(&name),
            sessions_info: HashMap::new(),
            sessions: HashSet::new(),
        }
    }
}

impl Channel {
    fn get_presence_data(&self) -> Option<PresenceInternalData> {
        if let ChannelType::Presence = self.internal_type {
            let mut presence = PresenceInternalData {
                ids: vec![],
                hash: HashMap::new(),
                count: self.sessions.len(),
            };

            for (id, info) in &self.sessions_info {
                presence.ids.push(id.to_string());
                presence.hash.insert(id.to_string(), info.user_info.clone());
            }

            Some(presence)
        } else {
            None
        }
    }

    fn get_recipients(&self, except: Option<usize>) -> HashSet<usize> {
        match except {
            Some(id) => self.sessions.clone().into_iter().filter(|v| v != &id).collect(),
            None => self.sessions.clone(),
        }
    }

    pub fn broadcast(&self, b: Broadcast) -> Sendable {
        Sendable {
            recipients: self.get_recipients(b.except),
            message: Box::new(SendClientEvent {
                channel: self.name.to_string(),
                event: b.name.clone(),
                data: b.data.clone(),
            }),
        }
    }

    pub fn subscribe(&mut self, id: SocketId, data: SubscribePayload, app: AppModel) -> Result<Option<Vec<Sendable>>, &'static str> {
        if self.sessions.contains(&id.val()) {
            return Ok(None);
        }

        if self.name.starts_with("private-") || self.name.starts_with("presence-") {
            let key = Key {
                public: app.key,
                private: app.secret,
            };

            let signature = data.auth.ok_or("missing authorization signature")?;

            if self.name.starts_with("private-") {
                if !key.is_valid_signature(signature, vec![id.to_string(), data.channel]) {
                    return Err("invalid signature");
                }
            } else if !key.is_valid_signature(
                signature,
                vec![id.to_string(), data.channel, serde_json::to_string(&data.channel_data).unwrap()],
            ) {
                return Err("invalid signature");
            }
        }

        let res = match self.internal_type {
            ChannelType::Presence => {
                // validation


                vec![
                    Sendable {
                        recipients: self.get_recipients(Some(id.val())),
                        message: Box::new(ChannelEvent::PusherInternalMemberAdded {
                            channel: self.name.clone(),
                            // validation
                            data: data.channel_data.as_ref().ok_or_else(|| "invalid user info for presence channel")?.clone(),
                        }),
                    },
                    self.on_subscription_succeeded(id),
                ]
            }
            _ => vec![self.on_subscription_succeeded(id)]
        };

        self.sessions.insert(id.val());

        match data.channel_data.clone() {
            Some(v) => {
                self.sessions_info.insert(id.val(), v.clone());
            },
            None => {}
        };

        Ok(Some(res))
    }

    fn on_subscription_succeeded(&mut self, id: SocketId) -> Sendable {
        let presence = match self.name.starts_with("presence-") {
            true => self.get_presence_data(),
            false => None,
        };

        create_sendable(
            HashSet::from([id.val()]),
            ChannelEvent::PusherInternalSubscriptionSucceeded {
                channel: self.name.clone(),
                data: SubscriptionData {
                    presence
                },
            },
        )
    }

    pub fn unsubscribe(&mut self, id: usize) -> Result<Option<Sendable>, &'static str> {
        if !self.sessions.contains(&id) {
            return Ok(None);
        }

        self.sessions.remove(&id);

        let info = self.sessions_info.remove(&id);

        let res = match self.internal_type {
            ChannelType::Presence => Some(create_sendable(
                self.sessions.clone(),
                ChannelEvent::PusherInternalMemberRemoved {
                    channel: self.name.clone(),
                    data: PresenceMemberRemovedData {
                        user_id: info.ok_or_else(|| "user info is absent")?.user_id
                    },
                },
            )),
            _ => None
        };

        Ok(res)
    }
}

fn create_sendable(recipients: HashSet<usize>, e: ChannelEvent) -> Sendable {
    Sendable {
        recipients,
        message: Box::new(e),
    }
}


#[derive(Debug, Copy, Clone)]
pub enum ChannelType {
    Presence,
    Private,
    Public,
}

impl ChannelType {
    pub fn which(name: &str) -> ChannelType {
        if name.starts_with("presence-") {
            Self::Presence
        } else if name.starts_with("private-") {
            Self::Private
        } else {
            Self::Public
        }
    }
}
