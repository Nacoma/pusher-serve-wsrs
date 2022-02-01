use actix::Recipient;
use std::collections::{HashMap, HashSet};
use std::sync::RwLock;

use crate::kind::Channel;
use crate::messages::PusherMessageChannelData;
use crate::OutgoingMessage;

#[derive(Default, Debug)]
pub struct Namespace {
    sockets: RwLock<HashMap<usize, Recipient<OutgoingMessage>>>,
    channels: RwLock<HashMap<Channel, HashSet<usize>>>,
    channel_presence_data:
        RwLock<HashMap<Channel, HashMap<usize, Option<PusherMessageChannelData>>>>,
}

impl Namespace {
    pub fn add_socket(&self, id: usize, ws: Recipient<OutgoingMessage>) {
        self.sockets.write().unwrap().insert(id, ws);
    }

    pub fn remove_socket(&self, id: usize) {
        self.sockets.write().unwrap().remove(&id);
    }

    pub fn channels_for_member(&self, id: usize) -> Vec<Channel> {
        self.channels
            .read()
            .unwrap()
            .iter()
            .filter(|(_, members)| members.contains(&id))
            .map(|(channel, _)| channel.clone())
            .collect()
    }

    pub fn add_to_channel(
        &self,
        id: usize,
        channel: &Channel,
        presence_data: Option<PusherMessageChannelData>,
    ) -> usize {
        let mut channels = self.channels.write().unwrap();

        channels.entry(channel.clone()).or_default();

        let members = channels.get_mut(channel).unwrap();

        members.insert(id);

        let count = members.len();

        drop(channels);

        self.channel_presence_data
            .write()
            .unwrap()
            .entry(channel.clone())
            .or_default()
            .insert(id, presence_data);

        count
    }

    pub fn remove_from_channel(&self, id: usize, channel: &Channel) {
        if let Some(hs) = self.channels.write().unwrap().get_mut(channel) {
            hs.remove(&id);
        }

        if let Some(presence_data) = self.channel_presence_data.write().unwrap().get_mut(channel) {
            presence_data.remove(&id);
        }
    }

    pub fn channel_sockets(&self, channel: &Channel) -> HashMap<usize, Recipient<OutgoingMessage>> {
        return if let Some(hs) = self.channels.read().unwrap().get(channel) {
            let sockets = self.sockets.read().unwrap();

            hs.iter()
                .map(|id| (*id, Clone::clone(sockets.get(id).unwrap())))
                .collect()
        } else {
            HashMap::default()
        };
    }

    pub fn get_presence_data(
        &self,
        id: usize,
        channel: &Channel,
    ) -> Option<PusherMessageChannelData> {
        if let Some(map) = self.channel_presence_data.read().unwrap().get(channel) {
            map.get(&id).unwrap().clone()
        } else {
            None
        }
    }

    pub fn channel_members(&self, channel: &Channel) -> HashMap<usize, PusherMessageChannelData> {
        if let Some(presence_data) = self.channel_presence_data.read().unwrap().get(channel) {
            let hs: HashMap<usize, Option<PusherMessageChannelData>> = self
                .channel_sockets(channel)
                .iter()
                .map(|(id, _)| (*id, presence_data.get(id).unwrap().clone()))
                .collect();

            hs.iter()
                .filter(|(_, ws)| ws.is_some())
                .map(|(s, ws)| (*s, ws.clone().unwrap()))
                .collect()
        } else {
            HashMap::default()
        }
    }
}
