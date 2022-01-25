use crate::adapter::Adapter;
use crate::WebSocket;
use crate::auth::{AuthPayload, AuthError, Key, validate_token};
use crate::messages::PusherMessage;

pub struct JoinResponse {
    pub ws: WebSocket,
    pub success: bool,
    pub error_code: Option<i32>,
    pub error_message: Option<&'static str>,
}

impl JoinResponse {
    pub fn todo_error(ws: WebSocket) -> JoinResponse {
        JoinResponse {
            ws,
            success: false,
            error_code: None,
            error_message: None,
        }
    }
}

pub struct LeaveResponse {
    pub left: bool,
}

pub trait ChannelManger {
    fn join(&self, adapter: &Box<dyn Adapter>, ws: WebSocket, message: PusherMessage) -> JoinResponse;

    fn leave(&self, adapter: &Box<dyn Adapter>, ws: WebSocket, message: PusherMessage) -> LeaveResponse {
        let mut namespace = adapter.namespace(&ws.app.id);

        namespace.remove_from_channel(ws.id, message.channel.unwrap().clone());

        LeaveResponse { left: true }
    }
}

pub struct PublicChannelManager {}

impl ChannelManger for PublicChannelManager {
    fn join(&self, adapter: &Box<dyn Adapter>, ws: WebSocket, message: PusherMessage) -> JoinResponse {
        let app_id = ws.app.id.clone();

        adapter
            .namespace(&app_id)
            .add_to_channel(Clone::clone(&ws), message.channel.unwrap().clone());

        JoinResponse {
            ws,
            success: true,
            error_message: None,
            error_code: None,
        }
    }
}

pub struct PrivateChannelManager {}

impl ChannelManger for PrivateChannelManager {
    fn join(&self, adapter: &Box<dyn Adapter>, ws: WebSocket, message: PusherMessage) -> JoinResponse {
        let channel = message.channel.unwrap();

        match validate_token(&ws.app, AuthPayload::new(
            message.data.auth.unwrap(),
            ws.id.to_string(),
            channel.clone(),
            None,
        )) {
            Ok(_) => {
                let app_id = ws.app.id.clone();


                adapter
                    .namespace(&app_id)
                    .add_to_channel(Clone::clone(&ws), channel.clone());

                JoinResponse {
                    ws,
                    success: true,
                    error_message: None,
                    error_code: None,
                }
            }
            Err(_) => {
                JoinResponse::todo_error(ws)
            }
        }

    }
}

pub struct PresenceChannelManager {}

impl ChannelManger for PresenceChannelManager {
    fn join(&self, adapter: &Box<dyn Adapter>, ws: WebSocket, message: PusherMessage) -> JoinResponse {
        let channel = message.channel.unwrap().clone();

        match validate_token(&ws.app, AuthPayload::new(
            message.data.auth.unwrap(),
            ws.id.to_string(),
            channel.clone(),
            message.data.channel_data,
        )) {
            Ok(_) => {
                let app_id = ws.app.id.clone();

                adapter
                    .namespace(&app_id)
                    .add_to_channel(Clone::clone(&ws), channel.clone());

                JoinResponse {
                    ws,
                    success: true,
                    error_message: None,
                    error_code: None,
                }
            }
            Err(_) => {
                JoinResponse::todo_error(ws)
            }
        }
    }
}
