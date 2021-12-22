use rand::{thread_rng, Rng, distributions::Alphanumeric};
use hex;
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;

#[derive(Clone)]
pub struct Key {
    pub public: String,
    pub private: String,
}

type HmacSha256 = Hmac<Sha256>;

impl Key {
    pub fn generate() -> Key {
        Key {
            public: generate_public_key(),
            private: generate_secret_key(),
        }
    }

    pub fn is_valid_signature(
        &self,
        signature: String,
        message_parts: Vec<String>,
    ) -> bool {
        let sig_components = signature.split(":").collect::<Vec<&str>>();

        if sig_components.len() != 2 || sig_components[0] != self.public {
            return false;
        };

        let decoded_signature = hex::decode(sig_components[1].as_bytes()).unwrap_or(vec![]);

        let message = message_parts.iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join(":");

        match HmacSha256::new_from_slice(self.private.as_bytes()) {
            Ok(mut mac) => {
                mac.update(message.as_bytes());

                mac.verify(decoded_signature.as_slice()).is_ok()
            },
            Err(_) => false,
        }
    }
}

/// Generate a hex encoded secret key
fn generate_secret_key() -> String {
    hex::encode(
        thread_rng()
            .gen::<[u8; 16]>()
            .to_vec()
    )
        .to_string()
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
    use crate::Key;

    #[test]
    fn create_new_key() {
        let key = Key::generate();

        assert_eq!(24, key.public.len());
        assert!(key.private.len() > 0);
    }

    #[test]
    fn validates_signatures() {
        let key = Key {
            public: "278d425bdf160c739803".to_string(),
            private: "7ad3773142a6692b25b8".to_string(),
        };

        let signature = "278d425bdf160c739803:58df8b0c36d6982b82c3ecf6b4662e34fe8c25bba48f5369f135bf843651c3a4".to_string();

        assert!(key.is_valid_signature(signature, vec!["1234.1234".to_string(), "private-foobar".to_string()]));
    }
}
