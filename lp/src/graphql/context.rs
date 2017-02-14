use diesel::pg::PgConnection;
use juniper;

use db::{self, PooledPgConnection};

pub struct Context {
    connection: PooledPgConnection,
}

impl Context {
    pub fn new() -> Context {
        Context { connection: db::connection().get().unwrap() }
    }

    pub fn connection(&self) -> &PgConnection {
        &self.connection
    }
}

impl juniper::Context for Context {}
