#[derive(Clone)]
pub struct App {
    pub id: String,
    pub key: String,
    pub secret: String,
}

impl Default for App {
    fn default() -> Self {
        App {
            id: "".to_string(),
            key: "".to_string(),
            secret: "".to_string(),
        }
    }
}
