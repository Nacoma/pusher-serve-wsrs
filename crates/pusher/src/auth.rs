use hmac::{Hmac, Mac, NewMac};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use sha2::Sha256;
use std::fmt::Debug;
use crate::app::App;
use crate::Socket;

#[derive(Clone)]
pub struct Key {
    pub public: String,
    pub private: String,
}

#[derive(Debug)]
pub struct AuthError(pub &'static str);

type HmacSha256 = Hmac<Sha256>;

pub struct AuthPayload {
    signature: String,
    id: String,
    channel: String,
    channel_data: Option<String>,
}

impl AuthPayload {
    pub fn new(signature: String, id: String, channel: String, channel_data: Option<String>) -> AuthPayload {
        AuthPayload {
            id,
            signature: signature,
            channel,
            channel_data,
        }
    }

    pub fn parts(&self) -> Vec<String> {
        if let Some(channel_data) = self.channel_data.clone() {
            vec![self.id.clone(), self.channel.clone(), channel_data.clone()]
        } else {
            vec![self.id.clone(), self.channel.clone()]
        }
    }
}

pub fn validate_token(app: &App, auth_payload: AuthPayload) -> Result<(), AuthError> {
    let sig_components = auth_payload.signature.split(':').collect::<Vec<&str>>();

    if sig_components.len() != 2 || sig_components[0] != app.key {
        return Err(AuthError("invalid signature"));
    };

    let decoded_signature = hex::decode(sig_components[1].as_bytes()).unwrap_or_default();

    let message = auth_payload.parts()
        .iter()
        .map(|p| p.to_string())
        .collect::<Vec<String>>()
        .join(":");

    match HmacSha256::new_from_slice(app.secret.as_bytes()) {
        Ok(mut mac) => {
            mac.update(message.as_bytes());

            if mac.verify(decoded_signature.as_slice()).is_ok() {
                Ok(())
            } else {
                Err(AuthError("failed to verify mac"))
            }
        }
        Err(_e) => Err(AuthError("failed to create hash")),
    }
}

impl Key {
    pub fn generate() -> Key {
        Key {
            public: generate_public_key(),
            private: generate_secret_key(),
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

#[cfg(test)]
mod tests {
    use crate::app::App;
    use crate::auth::{AuthPayload, validate_token};

    #[test]
    fn create_new_key() {
        todo!();
        // let key = Key::generate();
        //
        // assert_eq!(24, key.public.len());
        // assert!(!key.private.is_empty());
    }

    #[test]
    fn validates_signatures() {
        let app = App {
            id: "".to_string(),
            key: "278d425bdf160c739803".to_string(),
            secret: "7ad3773142a6692b25b8".to_string(),
        };

        let signature =
            "278d425bdf160c739803:58df8b0c36d6982b82c3ecf6b4662e34fe8c25bba48f5369f135bf843651c3a4"
                .to_string();

        let auth_payload = AuthPayload::new(signature, "1234.1234".to_string(), "private-foobar".to_string(), None);

        let res = validate_token(&app, auth_payload);

        assert!(res.is_ok());
    }
}
