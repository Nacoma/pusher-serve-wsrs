use std::collections::{HashMap, HashSet};

use app::App;

use crate::models::{ConnectionEstablishedPayload, SystemEvent, NewApp, AppModel};
use crate::models::responses::{GetChannelsResponseChannels, GetChannelsResponsePayload, GetChannelUsers, GetChannelUsersUser};
use crate::pusher::channel::{Channel, ChannelType};
use crate::server::{Sendable};
use crate::server::messages::{BroadcastMessage, ClientEventMessage, ClientEvent};
use crate::pusher::messages::Broadcast;
use crate::server::errors::WsrsError;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use crate::repository::Repository;

mod app;
mod channel;
pub mod messages;

pub struct Pusher {
    apps: HashMap<String, App>,
    repository: Repository,
}

impl Pusher {
    pub fn new(repository: Repository) -> Pusher {
        let apps: HashMap<String, App> = repository.apps()
            .iter()
            .map(|app| (app.key.clone(), App::new()))
            .collect();

        Pusher {
            apps,
            repository,
        }
    }
}

impl Pusher {
    pub fn remove_connection(&mut self, id: usize) -> Vec<Sendable> {
        self.apps.iter_mut().map(|(_, app)| {
            app.sessions.remove(&id);

            app.channels.iter_mut().filter_map(|(_, channel)| {
                channel.unsubscribe(id).ok()
            })
                .collect::<Vec<Option<Sendable>>>()

        })
            .flatten()
            .filter_map(|s| s)
            .collect()
    }

    pub fn broadcast(&mut self, msg: BroadcastMessage) -> Vec<Sendable> {
        let app = self.repository.find_app(msg.app.parse::<i32>().unwrap()).unwrap().key;

        let mut channels: Vec<String> = vec![];

        if let Some(ref channel) = msg.channel {
            channels.push(channel.to_string());
        }

        if let Some(ref _channels) = msg.channels {
            for channel in _channels {
                channels.push(channel.to_string());
            }
        }

        let broadcast = Broadcast {
            channels,
            except: msg.socket_id,
            data: msg.data,
            name: msg.name,
        };

        self.apps.get(app.as_str()).unwrap().broadcast(broadcast)
    }

    pub fn process_client_event(&mut self, msg: ClientEventMessage) -> Option<Vec<Sendable>> {
        let app = self.apps.get_mut(&msg.app).unwrap();

        match msg.message {
            ClientEvent::Subscribe {channel_data, auth: _, channel: channel_name} => {
                let channel = app
                    .channels
                    .entry(channel_name.to_owned())
                    .or_insert_with(|| Channel::new(channel_name.to_owned()));

                channel.subscribe(msg.id, channel_data).unwrap()

            },
            ClientEvent::Unsubscribe {channel: channel_name} => {
                let channel = app
                    .channels
                    .entry(channel_name.to_owned())
                    .or_insert_with(|| Channel::new(channel_name.to_owned()));

                match channel.unsubscribe(msg.id).unwrap() {
                    Some(s) => Some(vec![s]),
                    None => None,
                }

            },
            ClientEvent::Unknown => panic!("unknown subscription event"),
        }
    }

    pub fn get_channels(&self, app: &str) -> Result<GetChannelsResponsePayload, WsrsError> {
        let app = self.repository.find_app(app.parse::<i32>().unwrap()).unwrap().key;

        if let Some(app) = self.apps.get(&app) {
            Ok(GetChannelsResponsePayload {
                channels: app.channels.iter()
                    .map(|(name, channel)| {
                        match ChannelType::which(name.as_str()) {
                            ChannelType::Presence => {
                                GetChannelsResponseChannels {
                                    name: name.to_string(),
                                    user_count: Some(channel.sessions_info.len()),
                                }
                            },
                            _ => {
                                GetChannelsResponseChannels {
                                    name: name.to_string(),
                                    user_count: Some(channel.sessions.len()),
                                }
                            }
                        }
                    })
                    .collect()
            })
        } else {
            Err(WsrsError::app_not_found())
        }
    }

    pub fn get_channel_users(&self, app: &str, channel_name: &str) -> Result<GetChannelUsers, WsrsError> {
        let app = self.repository.find_app(app.parse::<i32>().unwrap()).unwrap().key;

        if let Some(app) = self.apps.get(&app) {
            if let Some(channel) = app.channels.get(channel_name) {
                Ok(GetChannelUsers {
                    users: channel.sessions_info.iter().map(|(_, i)| GetChannelUsersUser {
                        id: i.user_id.clone(),
                    }).collect()
                })
            } else {
                Err(WsrsError::channel_not_found())
            }
        } else {
            Err(WsrsError::app_not_found())
        }
    }

    pub fn add_connection(&mut self, app: String, id: usize) -> Sendable
    {
        self.apps
            .entry(app)
            .or_insert(App::new());

        let mut recipients = HashSet::new();
        recipients.insert(id);

        Sendable {
            recipients,
            message: Box::new(SystemEvent::PusherConnectionEstablished {
                data: ConnectionEstablishedPayload {
                    socket_id: id,
                    activity_timeout: 9000,
                },
            })
        }
    }

    pub fn find_app(&self, id: i32) -> Option<AppModel> {
        self.repository.find_app(id)
    }

    pub fn create_app(&self, app_name: String) -> Result<AppModel, &'static str> {
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(24)
            .map(char::from)
            .collect();


        Ok(self.repository.insert_app(&NewApp {
            key: rand_string.as_str(),
            name: app_name.as_str(),
        }))
    }

    pub fn list_apps(&self) -> Result<Vec<AppModel>, &'static str> {
        Ok(self.repository.apps())
    }

    pub fn delete_app(&self, app_id: i32) -> Result<(), &'static str> {
        Ok(self.repository.delete_app(app_id))
    }
}
