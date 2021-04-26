use serde::Serialize;

#[derive(Serialize)]
pub struct GetChannelsResponsePayload {
    pub channels: Vec<GetChannelsResponseChannels>
}

#[derive(Serialize)]
pub struct GetChannelsResponseChannels {
    pub name: String,
    pub user_count: Option<usize>,
}

#[derive(Serialize)]
pub struct GetChannelUsers {
    pub users: Vec<GetChannelUsersUser>,
}

#[derive(Serialize)]
pub struct GetChannelUsersUser {
    pub id: String,
}
