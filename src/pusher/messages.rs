#[derive(Clone)]
pub struct Broadcast {
    pub except: Option<usize>,
    pub channels: Vec<String>,
    pub data: serde_json::Value,
    pub name: String,
}
