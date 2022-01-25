use std::collections::{HashMap, HashSet};

use crate::kind::WebSocket;
use crate::Socket;

#[derive(Default)]
pub struct Namespace {
    sockets: HashMap<Socket, WebSocket>,
    channels: HashMap<String, HashSet<Socket>>,
}

impl Namespace {
    pub fn add_socket(&mut self, ws: WebSocket) {
        self.sockets.insert(ws.id, ws);
    }

    pub fn remove_socket(&mut self, socket: Socket) {
        self.sockets.remove(&socket);
    }

    pub fn add_to_channel(&mut self, ws: WebSocket, channel: String) {
        self.channels
            .entry(channel.clone())
            .or_insert(HashSet::new());

        self.channels.get_mut(&channel).unwrap().insert(ws.id);
    }

    pub fn remove_from_channel(&mut self, socket: Socket, channel: String) {
        if let Some(hs) = self.channels.get_mut(&channel) {
            hs.remove(&socket);
        }
    }

    pub fn is_in_channel(&self, socket: Socket, channel: String) -> bool {
        return if let Some(hs) = self.channels.get(&channel) {
            hs.contains(&socket)
        } else {
            false
        };
    }

    pub fn channels(&self) -> HashMap<String, HashSet<Socket>> {
        self.channels.clone()
    }

    pub fn channel_sockets(&self, channel: String) -> HashMap<Socket, &WebSocket> {
        if let Some(hs) = self.channels.get(&channel) {
            hs.iter()
                .map(|socket| (socket.clone(), self.sockets.get(&socket).unwrap()))
                .collect()
        } else {
            HashMap::default()
        }
    }

    pub fn channel_members(&self, channel: String) -> HashMap<Socket, String> {
        let hs: HashMap<Socket, Option<String>> = self
            .channel_sockets(channel.clone())
            .iter()
            .map(|(socket, ws)| (socket.clone(), ws.presence_data.clone()))
            .collect();

        hs.iter()
            .filter(|(_, ws)| ws.is_some())
            .map(|(s, ws)| (s.clone(), ws.clone().unwrap()))
            .collect()
    }
}
