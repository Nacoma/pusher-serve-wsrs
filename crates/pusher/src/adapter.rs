use crate::app::App;
use crate::namespace::Namespace;
use parking_lot::{MappedMutexGuard, Mutex, MutexGuard};

use std::collections::HashMap;

use crate::AppRepo;
use std::sync::{Arc, RwLock};

pub trait Adapter: Send + Sync {
    fn namespace(&self, app_id: i64) -> MappedMutexGuard<Namespace>;
}

#[derive(Default)]
pub struct InMemoryAdapter {
    apps: RwLock<HashMap<i64, App>>,
    namespaces: Mutex<HashMap<i64, Namespace>>,
}

impl Adapter for InMemoryAdapter {
    fn namespace(&self, app_id: i64) -> MappedMutexGuard<Namespace> {
        MutexGuard::map(self.namespaces.lock(), |d| d.entry(app_id).or_default())
    }
}
