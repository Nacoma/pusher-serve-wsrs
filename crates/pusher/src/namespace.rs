use actix::Recipient;
use std::collections::{HashMap, HashSet};

use crate::kind::Channel;
use crate::messages::PusherMessageChannelData;
use crate::OutgoingMessage;

#[derive(Default, Debug)]
pub struct Namespace {
    sockets: HashMap<usize, Recipient<OutgoingMessage>>,
    channels: HashMap<Channel, HashSet<usize>>,
    channel_presence_data: HashMap<Channel, HashMap<usize, Option<PusherMessageChannelData>>>,
}

impl Namespace {
    pub fn add_socket(&mut self, id: usize, ws: Recipient<OutgoingMessage>) {
        self.sockets.insert(id, ws);
    }

    pub fn remove_socket(&mut self, id: usize) {
        self.sockets.remove(&id);
    }

    pub fn channels_for_member(&self, id: usize) -> Vec<Channel> {
        self.channels.iter()
            .filter(|(_, members)| members.contains(&id))
            .map(|(channel, _)| channel.clone())
            .collect()
    }

    pub fn add_to_channel(
        &mut self,
        id: usize,
        channel: &Channel,
        presence_data: Option<PusherMessageChannelData>,
    ) -> usize {
        self.channels.entry(channel.clone()).or_default();

        self.channels.get_mut(&channel).unwrap().insert(id);

        self.channel_presence_data
            .entry(channel.clone())
            .or_default()
            .insert(id, presence_data);

        self.channels.get(&channel).unwrap().len()
    }

    pub fn remove_from_channel(&mut self, id: usize, channel: &Channel) {
        if let Some(hs) = self.channels.get_mut(channel) {
            hs.remove(&id);
        }

        if let Some(presence_data) = self.channel_presence_data.get_mut(&channel) {
            presence_data.remove(&id);
        }
    }

    pub fn is_in_channel(&self, id: usize, channel: &Channel) -> bool {
        return if let Some(hs) = self.channels.get(channel) {
            hs.contains(&id)
        } else {
            false
        };
    }

    pub fn channels(&self) -> HashMap<Channel, HashSet<usize>> {
        self.channels.clone()
    }

    pub fn channel_sockets(
        &self,
        channel: &Channel,
    ) -> HashMap<usize, &Recipient<OutgoingMessage>> {
        if let Some(hs) = self.channels.get(channel) {
            hs.iter()
                .map(|socket| (socket.clone(), self.sockets.get(&socket).unwrap()))
                .collect()
        } else {
            HashMap::default()
        }
    }

    pub fn get_presence_data(&self, id: usize, channel: &Channel) -> Option<PusherMessageChannelData> {
        if let Some(map) = self.channel_presence_data.get(channel) {
            map.get(&id).unwrap().clone()
        } else {
            None
        }
    }

    pub fn channel_members(&self, channel: &Channel) -> HashMap<usize, PusherMessageChannelData> {
        if let Some(presence_data) = self.channel_presence_data.get(channel) {
            let hs: HashMap<usize, Option<PusherMessageChannelData>> = self
                .channel_sockets(channel)
                .iter()
                .map(|(id, _)| (id.clone(), presence_data.get(id).unwrap().clone()))
                .collect();

            hs.iter()
                .filter(|(_, ws)| ws.is_some())
                .map(|(s, ws)| (s.clone(), ws.clone().unwrap()))
                .collect()
        } else {
            HashMap::default()
        }
    }
}
