use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

use hmac::{Hmac, Mac, NewMac};
use serde::Serialize;
use sha2::Sha256;

use crate::models::{AppModel, ChannelEvent, PresenceInternalData, PresenceMemberRemovedData, SendClientEvent, SubscriptionData};
use crate::pusher::messages::Broadcast;
use crate::pusher::types::SocketId;
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

    pub fn subscribe(&mut self, id: SocketId, data: SubscribePayload, app: AppModel) -> Result<Option<Vec<Sendable>>, &'static str> {
        if self.sessions.contains(&id.val()) {
            return Ok(None);
        }

        Ok(Some(match self.internal_type {
            ChannelType::Presence => {
                let signature = data.auth.ok_or("missing authorization signature")?;

                validate_auth_signature(
                    signature,
                    app.key,
                    app.secret,
                    id,
                    data.channel,
                    Some(serde_json::to_string(&data.channel_data).unwrap()),
                )?;

                let data = data.channel_data.ok_or_else(|| "invalid user info for presence channel")?;

                self.sessions.insert(id.val());
                self.sessions_info.insert(id.val(), data.clone());

                vec![
                    Sendable {
                        recipients: self.get_recipients(Some(id.val())),
                        message: Box::new(ChannelEvent::PusherInternalMemberAdded {
                            channel: self.name.clone(),
                            data,
                        }),
                    },
                    Sendable {
                        recipients: vec![id.val()].into_iter().collect(),
                        message: Box::new(ChannelEvent::PusherInternalSubscriptionSucceeded {
                            channel: self.name.clone(),
                            data: SubscriptionData {
                                presence: self.get_presence_data(),
                            },
                        }),
                    },
                ]
            }
            ChannelType::Private => {
                let signature = data.auth.ok_or("missing authorization signature")?;

                validate_auth_signature(
                    signature,
                    app.key,
                    app.secret,
                    id,
                    data.channel,
                    Some(serde_json::to_string(&data.channel_data).unwrap()),
                )?;

                self.sessions.insert(id.val());

                vec![Sendable {
                    recipients: vec![id.val()].into_iter().collect(),
                    message: Box::new(ChannelEvent::PusherInternalSubscriptionSucceeded {
                        channel: self.name.clone(),
                        data: SubscriptionData {
                            presence: None
                        },
                    }),
                }]
            }
            ChannelType::Public => {
                self.sessions.insert(id.val());

                vec![Sendable {
                    recipients: vec![id.val()].into_iter().collect(),
                    message: Box::new(ChannelEvent::PusherInternalSubscriptionSucceeded {
                        channel: self.name.clone(),
                        data: SubscriptionData {
                            presence: None
                        },
                    }),
                }]
            }
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

fn validate_auth_signature(
    signature: String,
    key: String,
    secret: String,
    socket_id: SocketId,
    channel: String,
    channel_data: Option<String>,
) -> Result<(), &'static str> {
    let components: Vec<&str> = signature.split(":").collect();

    if components.len() != 2 || components[0] != key {
        return Err("invalid auth signature");
    }

    let id_str: String = socket_id.into();

    let message = if channel_data.is_some() && channel.starts_with("presence-") {
        format!("{}:{}:{}", id_str, channel, channel_data.unwrap())
    } else {
        format!("{}:{}", id_str, channel)
    };

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();

    mac.update(message.as_bytes());

    let decoded_signature = hex::decode(components[1].as_bytes())
        .or(Err("invalid auth signature"))?;

    mac.verify(decoded_signature.as_slice())
        .or(Err("invalid auth signature"))
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::pusher::channel::validate_auth_signature;
    use crate::pusher::types::SocketId;

    #[test]
    fn validates_auth_payload_with_channel_data() {
        let channel_data = json!({
            "user_id": 10,
            "user_info": {
                "name": "Mr. Channels"
            }
        })
            .to_string();

        let r1 = validate_auth_signature(
            "278d425bdf160c739803:31935e7d86dba64c2a90aed31fdc61869f9b22ba9d8863bba239c03ca481bc80".to_string(),
            "278d425bdf160c739803".to_string(),
            "7ad3773142a6692b25b8".to_string(),
            SocketId::from(12341234),
            "presence-foobar".to_string(),
            Some(channel_data),
        );

        assert!(r1.is_ok());
    }

    #[test]
    fn can_validate_signature() {
        let res = validate_auth_signature(
            "278d425bdf160c739803:58df8b0c36d6982b82c3ecf6b4662e34fe8c25bba48f5369f135bf843651c3a4".to_string(),
            "278d425bdf160c739803".to_string(),
            "7ad3773142a6692b25b8".to_string(),
            SocketId::from(12341234),
            "private-foobar".to_string(),
            None,
        );

        assert!(res.is_ok());
    }
}
