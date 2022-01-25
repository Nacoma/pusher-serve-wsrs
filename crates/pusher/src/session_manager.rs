use crate::socket::Socket;
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct SessionManager {
    sessions: HashMap<usize, Option<String>>,
}

impl SessionManager {
    pub fn connect(&mut self, socket: Socket, data: Option<String>) {
        self.sessions.insert(socket.id, data);
    }

    pub fn disconnect(&mut self, socket: &Socket) {
        self.sessions.remove(&socket.id);
    }

    pub fn is_connected(&self, socket: &Socket) -> bool {
        self.sessions.contains_key(&socket.id)
    }

    pub fn sessions(&self) -> HashSet<usize> {
        HashSet::from_iter(self.sessions.keys().copied())
    }
}

#[cfg(test)]
mod tests {
    use crate::session_manager::SessionManager;
    use crate::socket::Socket;

    #[test]
    fn can_connect() {
        let mut session = SessionManager::default();
        session.connect(Socket { id: 1 }, None);

        assert!(session.sessions().contains(&1))
    }

    #[test]
    fn can_disconnect() {
        let mut session = SessionManager::default();
        session.connect(Socket { id: 1 }, None);
        session.disconnect(&Socket { id: 1 });

        assert!(!session.sessions().contains(&1));
    }

    #[test]
    fn can_check_if_connected() {
        let mut session = SessionManager::default();
        session.connect(Socket { id: 1 }, None);

        assert!(session.is_connected(&Socket { id: 1 }));
    }
}
