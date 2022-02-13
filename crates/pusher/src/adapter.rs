use crate::app::App;
use crate::namespace::Namespace;
use parking_lot::{MappedMutexGuard, Mutex, MutexGuard};

use std::collections::HashMap;

use std::sync::RwLock;
use actix::Recipient;
use crate::OutgoingMessage;

pub trait Adapter: Send + Sync {
    fn namespace(&self, app_id: i64) -> MappedMutexGuard<Namespace<Recipient<OutgoingMessage>>>;
}

#[derive(Default)]
pub struct InMemoryAdapter {
    namespaces: Mutex<HashMap<i64, Namespace<Recipient<OutgoingMessage>>>>,
}

impl Adapter for InMemoryAdapter {
    fn namespace(&self, app_id: i64) -> MappedMutexGuard<Namespace<Recipient<OutgoingMessage>>> {
        MutexGuard::map(self.namespaces.lock(), |d| d.entry(app_id).or_default())
    }
}
