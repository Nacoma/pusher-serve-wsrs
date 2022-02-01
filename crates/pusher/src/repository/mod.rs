pub mod sqlite;
use crate::app::App;
use std::collections::HashMap;

pub trait AppRepo: Send + Sync {
    fn all(&self) -> Vec<App>;
    fn find_by_id(&self, id: i64) -> Option<App>;
    fn find_by_key(&self, key: &String) -> Option<App>;
    fn insert_app(&mut self, app: &App) -> Result<(), Box<dyn std::error::Error>>;
}

#[derive(Default, Debug)]
pub struct InMemoryAppRepo {
    apps: HashMap<i64, App>,
    key_to_id: HashMap<String, i64>,
}

impl AppRepo for InMemoryAppRepo {
    fn all(&self) -> Vec<App> {
        todo!();
    }

    fn find_by_id(&self, id: i64) -> Option<App> {
        self.apps.get(&id).cloned()
    }

    fn find_by_key(&self, key: &String) -> Option<App> {
        if let Some(id) = self.key_to_id.get(key) {
            self.find_by_id(*id)
        } else {
            None
        }
    }

    fn insert_app(&mut self, app: &App) -> Result<(), Box<dyn std::error::Error>> {
        self.apps.insert(app.id, app.clone());
        self.key_to_id.insert(app.key.clone(), app.id);

        Ok(())
    }
}
