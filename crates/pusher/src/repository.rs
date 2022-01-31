use std::collections::HashMap;
use crate::app::App;

pub trait AppRepo : Send {
    fn find_by_id(&self, id: &String) -> Option<App>;
    fn find_by_key(&self, key: &String) -> Option<App>;
    fn insert_app(&mut self, app: App);
}

#[derive(Default, Debug)]
pub struct InMemoryAppRepo {
    apps: HashMap<String, App>,
    key_to_id: HashMap<String, String>,
}

impl AppRepo for InMemoryAppRepo {
    fn find_by_id(&self, id: &String) -> Option<App> {
        match self.apps.get(id) {
            Some(app) => Some(app.clone()),
            None => None,
        }
    }

    fn find_by_key(&self, key: &String) -> Option<App> {
        match self.key_to_id.get(key) {
            Some(id) => self.find_by_id(id),
            None => None
        }
    }

    fn insert_app(&mut self, app: App) {
        self.apps.insert(app.id.clone(), app.clone());
        self.key_to_id.insert(app.key.clone(), app.id.clone());

        println!("{:?}", self);
    }
}
