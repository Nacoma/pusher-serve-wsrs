use std::fmt::{Display, Formatter, Result};

pub struct WsrsError {
    pub message: String,
    pub kind: WsrsErrorKind,
}

impl Display for WsrsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.message)
    }
}

pub enum WsrsErrorKind {
    AppNotFound,
    ChannelNotFound,
    Other,
}

impl WsrsError {
    pub fn app_not_found() -> WsrsError {
        WsrsError {
            message: "application not found".to_string(),
            kind: WsrsErrorKind::AppNotFound,
        }
    }

    pub fn channel_not_found() -> WsrsError {
        WsrsError {
            message: "channel not found".to_string(),
            kind: WsrsErrorKind::ChannelNotFound,
        }
    }
}
