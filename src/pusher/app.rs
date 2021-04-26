use std::collections::{HashMap, HashSet};

use super::channel::Channel;
use crate::server::Sendable;
use crate::pusher::messages::Broadcast;

#[derive(Debug)]
pub struct App {
    pub sessions: HashSet<usize>,
    pub channels: HashMap<String, Channel>,
}

impl App {
    pub fn new() -> App {
        App {
            channels: HashMap::new(),
            sessions: HashSet::new(),
        }
    }
}

impl App {
    pub fn broadcast(&self, b: Broadcast) -> Vec<Sendable> {
        let channels = b.channels.clone();

        channels.iter().filter_map(|channel_name| {
            match self.channels.get(channel_name) {
                Some(channel) => Some(channel.broadcast(b.clone())),
                None => None,
            }
        })
            .collect()
    }
}
