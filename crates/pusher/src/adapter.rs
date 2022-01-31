use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use crate::app::App;
use crate::namespace::Namespace;

pub trait Adapter : Send {
    fn namespace(&mut self, app_id: &String) -> &mut Namespace;
}

#[derive(Default)]
pub struct InMemoryAdapter {
    apps: HashMap<String, App>,
    namespaces: HashMap<String, Namespace>
}

impl InMemoryAdapter {
    pub fn add_app(&mut self, app: App) {
        if !self.apps.contains_key(&app.id) {
            self.apps.insert(app.id.clone(), app.clone());
            self.namespaces.insert(app.id.clone(), Namespace::default());
        }
    }
}

impl Adapter for InMemoryAdapter {
    fn namespace(&mut self, app_id: &String) -> &mut Namespace {
        self.namespaces.get_mut(app_id).unwrap()
    }
}
