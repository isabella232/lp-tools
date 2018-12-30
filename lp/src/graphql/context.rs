use diesel::pg::PgConnection;
use juniper;

use crate::db::{self, PooledPgConnection};

pub struct Context {
    connection: PooledPgConnection,
}

impl Context {
    pub fn new() -> Context {
        Context::default()
    }

    pub fn connection(&self) -> &PgConnection {
        &self.connection
    }
}

impl Default for Context {
    fn default() -> Context {
        Context { connection: db::connection().get().unwrap() }
    }
}

impl juniper::Context for Context {}
