pub mod schema;

use crate::app::App;
use crate::AppRepo;
use diesel::prelude::*;
use diesel::SqliteConnection;
use std::sync::Mutex;

use schema::apps;

#[derive(Debug, Insertable)]
#[table_name = "apps"]
struct NewApp<'a> {
    pub id: i64,
    pub name: &'a str,
    pub key: &'a str,
    pub secret: &'a str,
}

#[derive(Debug, Queryable)]
struct QueryApp {
    pub id: i64,
    pub name: String,
    pub key: String,
    pub secret: String,
}

impl Into<App> for QueryApp {
    fn into(self) -> App {
        App {
            id: self.id,
            name: self.name,
            key: self.key,
            secret: self.secret,
        }
    }
}

impl Into<App> for &QueryApp {
    fn into(self) -> App {
        App {
            id: self.id,
            name: self.name.clone(),
            key: self.key.clone(),
            secret: self.secret.clone(),
        }
    }
}

pub struct SqliteRepo {
    conn: Mutex<SqliteConnection>,
}

impl SqliteRepo {
    pub fn new(conn: SqliteConnection) -> Self {
        Self {
            conn: Mutex::new(conn),
        }
    }
}

impl AppRepo for SqliteRepo {
    fn all(&self) -> Vec<App> {
        use schema::apps::dsl;

        let result: Vec<QueryApp> = dsl::apps
            .get_results::<QueryApp>(&*self.conn.lock().unwrap())
            .unwrap();

        result.iter().map(|app| app.into()).collect()
    }

    fn find_by_id(&self, id: i64) -> Option<App> {
        use schema::apps::dsl;

        let conn = self.conn.lock().unwrap();

        let result: QueryResult<QueryApp> = dsl::apps.filter(dsl::id.eq(id)).first(&*conn);

        if let Ok(query_app) = result {
            Some(query_app.into())
        } else {
            None
        }
    }

    fn find_by_key(&self, key: &String) -> Option<App> {
        use schema::apps::dsl;

        let conn = self.conn.lock().unwrap();

        let result = dsl::apps
            .filter(dsl::key.eq(key.as_str()))
            .first::<QueryApp>(&*conn);

        if let Ok(query_app) = result {
            Some(query_app.into())
        } else {
            None
        }
    }

    fn insert_app(&mut self, app: &App) -> Result<(), Box<dyn std::error::Error>> {
        let new_app = NewApp {
            id: app.id,
            name: app.name.as_str(),
            key: app.key.as_str(),
            secret: app.key.as_str(),
        };

        diesel::insert_into(apps::table)
            .values(&new_app)
            .execute(&*self.conn.lock().unwrap())?;

        Ok(())
    }
}
