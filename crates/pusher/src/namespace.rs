

use std::collections::{HashMap, HashSet};

use std::sync::RwLock;


use crate::kind::Channel;
use crate::messages::PusherMessageChannelData;


#[derive(Debug)]
pub struct Namespace<R>
where R: Clone
{
    sockets: RwLock<HashMap<usize, R>>,
    channels: RwLock<HashMap<Channel, HashSet<usize>>>,
    channel_presence_data:
    RwLock<HashMap<Channel, HashMap<usize, Option<PusherMessageChannelData>>>>,
}

impl<R> Default for Namespace<R> where R: Clone {
    fn default() -> Self {
        Namespace {
            sockets: RwLock::default(),
            channels: RwLock::default(),
            channel_presence_data: RwLock::default(),
        }
    }
}

impl<R> Namespace<R>
where R: Clone
{
    pub fn add_socket(&self, id: usize, ws: R) {
        self.sockets.write().unwrap().insert(id, ws);
    }

    pub fn channels(&self) -> Vec<Channel> {
        self.channels
            .read()
            .unwrap()
            .keys()
            .map(|c| c.clone())
            .collect()
    }

    pub fn users_per_channel(&self, channel: &Channel) -> usize {
        self.channel_presence_data
            .read()
            .unwrap()
            .get(channel)
            .unwrap()
            .keys()
            .len()
    }

    fn member_count_by_channel(&self, ch: &Channel) -> usize {
        if let Some(members) = self.channels.read().unwrap().get(&ch) {
            members.len()
        } else {
            0
        }
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
        let mut channels = self.channels.write().unwrap();

        let mut len = 0;

        if let Some(hs) = channels.get_mut(&channel) {
            hs.remove(&id);

            len = hs.len();
        }

        if len == 0 {
            channels.remove(&channel);
        }

        self.remove_presence_data(id, channel);
    }

    pub fn channel_sockets(&self, channel: &Channel) -> HashMap<usize, R> {
        return if let Some(hs) = self.channels.read().unwrap().get(channel) {
            let sockets = self.sockets.read().unwrap();

            hs.iter()
                .map(|id| (*id, Clone::clone(sockets.get(id).unwrap())))
                .collect()
        } else {
            HashMap::default()
        };
    }

    fn remove_presence_data(&self, id: usize, ch: &Channel) {
        let mut cpd = self.channel_presence_data.write().unwrap();

        let should_remove = if let Some(members) = cpd.get_mut(&ch) {
            members.remove(&id);

            members.is_empty()
        } else {
            false
        };

        if should_remove {
            cpd.remove(&ch);
        }
    }

    pub fn get_presence_data(
        &self,
        id: usize,
        channel: &Channel,
    ) -> Option<PusherMessageChannelData> {
        let presence_data = self.channel_presence_data.read().unwrap();

        if let Some(members) = presence_data.get(&channel) {
            if let Some(member) = members.get(&id) {
                member.clone()
            } else {
                None
            }
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

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use crate::kind::Channel;
    use crate::messages::PusherMessageChannelData;
    use crate::namespace::Namespace;

    #[test]
    fn socket_actions() {
        let ns: Namespace<&'static str> = Namespace::default();

        ns.add_socket(1, "t1");

        assert!(ns.sockets.read().unwrap().contains_key(&1));

        ns.remove_socket(1);

        assert!(!ns.sockets.read().unwrap().contains_key(&1));
    }

    #[test]
    fn can_get_channels_for_a_member() {
        let ns: Namespace<&'static str> = Namespace::default();

        ns.add_to_channel(1, &Channel::Public("test".to_string()), None);
        ns.add_to_channel(1, &Channel::Public("test2".to_string()), None);

        let channels = ns.channels_for_member(1);

        assert!(channels.contains(&Channel::Public("test".to_string())));
        assert!(channels.contains(&Channel::Public("test2".to_string())));
    }


    #[test]
    fn can_get_sockets_for_a_channel() {
        let ns: Namespace<&'static str> = Namespace::default();

        ns.add_socket(1, "s1");
        ns.add_socket(2, "s2");

        let ch = Channel::Public("test".to_string());

        ns.add_to_channel(1, &ch, None);
        ns.add_to_channel(2, &ch, None);

        let members = ns.channel_sockets(&ch);

        assert!(members.contains_key(&1));
        assert!(members.contains_key(&2));
    }

    #[test]
    fn channel_is_removed_when_last_member_leaves() {
        let ns: Namespace<&'static str> = Namespace::default();

        let ch = Channel::Public("test".to_string());

        ns.add_to_channel(1, &ch, None);

        assert_eq!(1, ns.member_count_by_channel(&ch));
        ns.remove_from_channel(1, &ch);
        assert_eq!(0, ns.member_count_by_channel(&ch));
    }

    #[test]
    fn presence_data_is_removed_when_member_leaves() {
        let ns: Namespace<&'static str> = Namespace::default();

        let ch = Channel::Public("test".to_string());

        ns.add_to_channel(1, &ch, Some(PusherMessageChannelData {
            user_info: Value::default(),
            user_id: "s1".to_string()
        }));
        ns.add_to_channel(2, &ch, Some(PusherMessageChannelData {
            user_info: Value::default(),
            user_id: "s2".to_string()
        }));

        assert!(ns.get_presence_data(1, &ch).is_some());
        assert!(ns.get_presence_data(2, &ch).is_some());

        ns.remove_from_channel(1, &ch);

        assert!(ns.get_presence_data(1, &ch).is_none());
        assert!(ns.get_presence_data(2, &ch).is_some());

        ns.remove_from_channel(2, &ch);
        println!("{:?}", ns.channel_presence_data);
        assert!(ns.get_presence_data(2, &ch).is_none());
        assert!(ns.channel_presence_data.read().unwrap().is_empty())
    }
}