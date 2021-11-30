use diesel::{SqliteConnection, Connection, insert_into, delete};
use diesel::prelude::*;
use crate::models::{NewApp, AppModel};
use super::schema::apps::dsl::*;
use diesel::result::Error;

pub struct Repository {
    conn: SqliteConnection,
}

impl Repository {
    pub fn new(conn: SqliteConnection) -> Repository {
        Repository {
            conn,
        }
    }
}

impl Repository {
    pub fn find_app(&self, _id: i32) -> Option<AppModel> {
        match apps.find(_id).first(&self.conn) {
            Ok(r) => Some(r),
            Err(_) => None
        }
    }

    pub fn find_app_by_key(&self, k: String) -> Option<AppModel> {
        match apps.filter(key.eq(k)).first(&self.conn) {
            Ok(r) => Some(r),
            Err(_) => None,
        }
    }

    pub fn delete_app(&self, app_id: i32) -> () {
        delete(apps.filter(id.eq(app_id))).execute(&self.conn).unwrap();
    }

    pub fn apps(&self) -> Vec<AppModel> {
        apps.load::<AppModel>(&self.conn).expect("error loading apps")
    }

    pub fn insert_app(&self, app: &NewApp) -> AppModel {
        let mut results: Vec<AppModel> = self.conn.transaction::<_, Error, _>(|| {
            insert_into(apps).values(app).execute(&self.conn)?;

            apps.order(id.desc()).limit(1).load::<AppModel>(&self.conn)
        })
            .unwrap();

        results.pop().unwrap()
    }
}
