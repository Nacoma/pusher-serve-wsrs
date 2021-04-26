use crate::server::messages::DataType;

#[derive(Clone)]
pub struct Broadcast {
    pub except: Option<usize>,
    pub channels: Vec<String>,
    pub data: DataType,
    pub name: String,
}
