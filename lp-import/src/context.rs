use diesel::pg::PgConnection;
use lp::db::{self, PooledPgConnection};
use lp::models::{Artist, Medium, Song};
use std::collections::HashMap;

pub struct Context {
    connection: PooledPgConnection,
    pub artists: HashMap<String, Artist>,
    pub media: HashMap<String, Medium>,
    pub songs: HashMap<String, Song>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            connection: db::connection().get().unwrap(),
            artists: HashMap::new(),
            media: HashMap::new(),
            songs: HashMap::new(),
        }
    }

    pub fn connection(&self) -> &PgConnection {
        &self.connection
    }
}
