use crate::kind::Channel;

use serde::Serialize;

#[derive(Serialize)]
pub struct PusherSubscriptionError {
    event: &'static str,
    data: PusherSubscriptionErrorData
}

#[derive(Serialize)]
pub struct PusherSubscriptionErrorData {
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
            }
        }
    }
}

#[derive(Serialize)]
pub struct PusherSystemError {
    event: &'static str,
    data: PusherSystemErrorData,
}

#[derive(Serialize)]
struct PusherSystemErrorData {
    error: String,
    status: Option<i32>,
}

impl ToString for PusherSystemError {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl PusherSystemError {
    pub fn new(kind: ErrorKind) -> Self {
        Self {
            event: kind.to_event(),
            data: PusherSystemErrorData {
                error: kind.to_message(),
                status: kind.to_code(),
            }
        }
    }
}

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
        match self {
            _ => "pusher:error",
        }
    }

    fn to_message(&self) -> String {
        todo!();

        match self {
            _ => "".to_string()
        }
    }
}
