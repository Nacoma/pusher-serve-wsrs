use std::convert::TryFrom;

#[derive(Copy, Clone, Debug)]
pub struct SocketId {
    id: usize,
}

impl SocketId {
    pub fn val(&self) -> usize {
        self.id
    }
}

impl ToString for SocketId {
    fn to_string(&self) -> String {
        (*self).into()
    }
}

impl TryFrom<String> for SocketId {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.replace(".", "").parse::<usize>() {
            Ok(id) => Ok(SocketId { id }),
            Err(_) => Err("invalid socket id"),
        }
    }
}

impl Into<String> for SocketId {
    fn into(self) -> String {
        let mut val = self.id.to_string();
        val.insert(4, '.');
        val
    }
}

impl Into<usize> for SocketId {
    fn into(self) -> usize {
        self.id
    }
}

impl From<usize> for SocketId {
    fn from(id: usize) -> Self {
        SocketId { id }
    }
}
