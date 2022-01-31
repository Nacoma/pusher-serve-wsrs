use rand::{distributions::Alphanumeric, thread_rng, Rng};

#[derive(Clone, Debug)]
pub struct App {
    pub id: String,
    pub key: String,
    pub secret: String,
}

impl Default for App {
    fn default() -> Self {
        App {
            id: "test".to_string(),
            key: "ybEhzuYXaWWcAQ6reiJQRAfw".to_string(),
            secret: "0f150d83f7ceebc8289286a020f418f6".to_string(),
        }
    }
}

/// Generate a hex encoded secret key
fn generate_secret_key() -> String {
    hex::encode(thread_rng().gen::<[u8; 16]>().to_vec())
}

/// Generate a human readable public key
fn generate_public_key() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(24)
        .map(char::from)
        .collect()
}
