use crate::messages::{JsonMessage, OutgoingMessage};
use serde::Serialize;
use std::error::Error;
use std::fmt::{Display, Formatter};

pub trait WsError: Error + Sync + Send {
    fn is_fatal(&self) -> bool {
        false
    }

    fn to_msg(&self) -> Option<String> {
        None
    }

    fn msg(&self) -> OutgoingMessage;
}

#[derive(Serialize, JsonMessage)]
struct PusherSubscriptionError {
    event: &'static str,
    data: PusherSubscriptionErrorData,
}

#[derive(Debug, Serialize)]
struct PusherSubscriptionErrorData {
    kind: &'static str,
    error: &'static str,
    status: i32,
}

impl Default for PusherSubscriptionError {
    fn default() -> Self {
        Self {
            event: "pusher:subscription_error",
            data: PusherSubscriptionErrorData {
                kind: "AuthError",
                error: "Not authorized",
                status: 403,
            },
        }
    }
}

#[derive(Debug, Serialize, JsonMessage)]
pub struct PusherSystemError {
    event: &'static str,
    data: PusherSystemErrorData,
}

impl Display for PusherSystemError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.data.status.unwrap_or(0), self.data.error)
    }
}

#[derive(Debug, Serialize)]
struct PusherSystemErrorData {
    error: String,
    status: Option<i32>,
}

#[allow(unused)]
#[derive(Debug)]
pub enum ErrorKind {
    AppRequiresSsl,
    AppNotFound,
    AppDisabled,
    AppOverConnectionQuota,
    PathNotFound,
    InvalidVersionStringFormat,
    UnsupportedProtocolVersion,
    NoProtocolVersionSupplied,
    ConnectionUnauthorized,

    OverCapacity,

    GenericReconnectImmediately,
    PongNotReceived,
    ClosedAfterInactivity,

    ExceededRateLimit,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_message())
    }
}

impl Error for ErrorKind {}

impl WsError for ErrorKind {
    fn is_fatal(&self) -> bool {
        matches!(self, ErrorKind::AppNotFound)
    }

    fn to_msg(&self) -> Option<String> {
        Some(
            serde_json::to_string(&PusherSystemError {
                event: self.to_event(),
                data: PusherSystemErrorData {
                    error: self.to_message(),
                    status: self.to_code(),
                },
            })
            .unwrap(),
        )
    }

    fn msg(&self) -> OutgoingMessage {
        OutgoingMessage(Box::new(PusherSystemError {
            event: self.to_event(),
            data: PusherSystemErrorData {
                error: self.to_message(),
                status: self.to_code(),
            },
        }))
    }
}

impl ErrorKind {
    fn to_code(&self) -> Option<i32> {
        match self {
            ErrorKind::AppRequiresSsl => Some(4000),
            ErrorKind::AppNotFound => Some(4001),
            ErrorKind::AppDisabled => Some(4003),
            ErrorKind::AppOverConnectionQuota => Some(4004),
            ErrorKind::PathNotFound => Some(4005),
            ErrorKind::InvalidVersionStringFormat => Some(4006),
            ErrorKind::UnsupportedProtocolVersion => Some(4007),
            ErrorKind::NoProtocolVersionSupplied => Some(4008),
            ErrorKind::ConnectionUnauthorized => Some(4009),
            ErrorKind::OverCapacity => Some(4100),
            ErrorKind::GenericReconnectImmediately => Some(4200),
            ErrorKind::PongNotReceived => Some(4201),
            ErrorKind::ClosedAfterInactivity => Some(4202),
            ErrorKind::ExceededRateLimit => Some(4301),
        }
    }

    fn to_event(&self) -> &'static str {
        "pusher:error"
    }

    fn to_message(&self) -> String {
        match self {
            ErrorKind::AppNotFound => "App key does not exist".to_string(),
            _ => todo!(),
        }
    }
}
