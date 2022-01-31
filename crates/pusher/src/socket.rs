use rand::{thread_rng, Rng};
use serde::de::Visitor;
use serde::{Serialize, Serializer};
use std::convert::TryFrom;
use std::fmt::Formatter;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Socket {
    pub id: usize,
}

impl Default for Socket {
    fn default() -> Self {
        Socket {
            id: thread_rng().gen(),
        }
    }
}

impl Serialize for Socket {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

struct SocketVisitor;

impl<'de> Visitor<'de> for SocketVisitor {
    type Value = Option<usize>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("expected integer or ####.####")
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if value >= usize::MAX as i64 || value <= usize::MIN as i64 {
            Err(E::custom(format!("integer out of range: {}", value)))
        } else {
            Ok(Some(value as usize))
        }
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if value >= usize::MAX as u64 || value <= usize::MIN as u64 {
            Err(E::custom(format!("integer out of range: {}", value)))
        } else {
            Ok(Some(value as usize))
        }
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Some(value.replace(".", "").parse::<usize>().unwrap()))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(None)
    }
}

impl TryFrom<String> for Socket {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.replace(".", "").parse::<usize>() {
            Ok(id) => Ok(Self { id }),
            Err(_) => Err("invalid socket id"),
        }
    }
}

impl ToString for Socket {
    fn to_string(&self) -> String {
        let mut val = self.id.to_string();
        val.insert(4, '.');
        val
    }
}
