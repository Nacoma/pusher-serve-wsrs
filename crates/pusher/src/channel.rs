use std::collections::{HashSet, HashMap};
use crate::session_manager::SessionManager;
use crate::messages::SubscribePayload;

pub struct Channel {
    pub name: String,
    pub channel_type: ChannelType,
    session: SessionManager
}

impl Channel {
    pub (crate) fn new(name: String) -> Self {
        Channel {
            channel_type: ChannelType::from(&name),
            session: SessionManager::new(),
            name,
        }
    }
}

pub enum ChannelType {
    Presence,
    Private,
    Public,
}

impl From<&String> for ChannelType {
    fn from(s: &String) -> Self {
        if s.starts_with("presence-") {
            Self::Presence
        } else if s.starts_with("private-") {
            Self::Private
        } else {
            Self::Public
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::channel::{Channel, ChannelType};

    #[test]
    fn get_channel_type_from_string() {
        let channel = Channel::new("private-something".to_string());

        assert!(matches!(channel.channel_type, ChannelType::Private));

        let channel = Channel::new("presence-something".to_string());

        assert!(matches!(channel.channel_type, ChannelType::Presence));
    }
}
