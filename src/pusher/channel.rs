use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

use serde::Serialize;

use crate::server::messages::UserInfo;
use crate::models::{ChannelEvent, PresenceInternalData, PresenceMemberRemovedData, SendClientEvent, SubscriptionData};
use crate::pusher::messages::Broadcast;
use crate::server::Sendable;

#[derive(Serialize, Debug, Clone)]
pub struct PresenceChannel {}

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

    pub fn subscribe(&mut self, id: usize, data: Option<UserInfo>) -> Result<Option<Vec<Sendable>>, &'static str> {
        if self.sessions.contains(&id) {
            return Ok(None);
        }

        Ok(Some(if let ChannelType::Presence = self.internal_type {
            let data = data.ok_or_else(|| "invalid user info for presence channel")?;

            self.sessions.insert(id);
            self.sessions_info.insert(id, data.clone());

            vec![
                Sendable {
                    recipients: self.get_recipients(Some(id)),
                    message: Box::new(ChannelEvent::PusherInternalMemberAdded {
                        channel: self.name.clone(),
                        data,
                    }),
                },
                Sendable {
                    recipients: vec![id].into_iter().collect(),
                    message: Box::new(ChannelEvent::PusherInternalSubscriptionSucceeded {
                        channel: self.name.clone(),
                        data: SubscriptionData {
                            presence: self.get_presence_data(),
                        },
                    }),
                }
            ]
        } else {
            self.sessions.insert(id);

            vec![Sendable {
                recipients: vec![id].into_iter().collect(),
                message: Box::new(ChannelEvent::PusherInternalSubscriptionSucceeded {
                    channel: self.name.clone(),
                    data: SubscriptionData {
                        presence: None
                    },
                }),
            }]
        }))
    }

    pub fn unsubscribe(&mut self, id: usize) -> Result<Option<Sendable>, &'static str> {
        if !self.sessions.contains(&id) {
            return Ok(None);
        }

        self.sessions.remove(&id);

        let info = self.sessions_info.remove(&id);

        if let ChannelType::Presence = self.internal_type {
            Ok(Some(Sendable {
                recipients: self.sessions.clone(),
                message: Box::new(ChannelEvent::PusherInternalMemberRemoved {
                    channel: self.name.clone(),
                    data: PresenceMemberRemovedData {
                        user_id: info.ok_or_else(|| "user info is absent")?.user_id
                    },
                }),
            }))
        } else {
            Ok(None)
        }
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
