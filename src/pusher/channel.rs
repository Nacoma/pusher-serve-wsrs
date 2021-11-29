use std::collections::{HashMap, HashSet};
use std::fmt::{Debug};

use hmac::{Hmac, Mac, NewMac};
use hmac::crypto_mac::MacError;
use serde::Serialize;
use sha2::Sha256;
use hex;

use crate::models::{AppModel, ChannelEvent, PresenceInternalData, PresenceMemberRemovedData, SendClientEvent, SubscriptionData};
use crate::pusher::messages::Broadcast;
use crate::server::messages::{SubscribePayload, UserInfo};
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

    pub fn subscribe(&mut self, id: usize, data: SubscribePayload, app: AppModel) -> Result<Option<Vec<Sendable>>, &'static str> {
        if self.sessions.contains(&id) {
            return Ok(None);
        }

        Ok(Some(if let ChannelType::Presence = self.internal_type {
            let signature = data.auth.ok_or("missing authorization signature")?;

            validate_signature(signature, app.secret, id.to_string(), data.channel)
                .or(Err("invalid authorization signature"))?;

            let data = data.channel_data.ok_or_else(|| "invalid user info for presence channel")?;

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
                },
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

type HmacSha256 = Hmac<Sha256>;

fn validate_signature(signature: String, secret: String, socket_id: String, channel: String) -> Result<(), MacError> {
    let mut id = socket_id.to_string();
    id.insert(4, '.');
    let message = format!("{}:{}", id, channel);

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .unwrap();

    mac.update(message.as_bytes());

    // println!("{:?}", std::str::from_utf8(&mac.finalize().into_bytes()));

    mac.verify(signature.as_bytes())
}


#[cfg(test)]
mod tests {
    use hmac::{Mac, Hmac, NewMac};
    use sha2::Sha256;
    use base64;
    use crate::pusher::channel::validate_signature;


    type HmacSha256 = Hmac<Sha256>;

    #[test]
    fn can_validate_signature() {
        let secret = "c5tCyBjiHMWapmjRJ5QUPmWQ".to_string();
        let socket_id = "1527.0736728473932610".to_string();
        let signature = "4af4cd3e3d6a4ae147253ed702a9dbe0b732372b3b89ebd2053b94dad1c65361".to_string();
        let channel = "presence-test".to_string();

        let message = format!("{}:{}", socket_id, channel);

        println!("{}", message);

        let mut mac = HmacSha256::new_from_slice(hex::encode(secret.as_bytes()).as_bytes()).unwrap();

        mac.update(hex::encode(message.as_bytes()).as_bytes());

        let res = mac.finalize();

        println!("{:?}", signature.as_bytes());
        println!("{:?}", res.into_bytes());

        assert!(
            validate_signature(signature.to_string(), secret, socket_id.replace(".", ""), channel).is_ok()
        );
    }
}
