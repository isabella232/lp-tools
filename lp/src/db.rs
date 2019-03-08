use std::env;

use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use dotenv::dotenv;
use lazy_static::lazy_static;

pub type PooledPgConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

lazy_static! {
    static ref CONNECTION: r2d2::Pool<ConnectionManager<PgConnection>> = {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);

        r2d2::Pool::builder()
            .build(manager)
            .expect("failed to create connection pool")
    };
}

pub fn connection() -> r2d2::Pool<ConnectionManager<PgConnection>> {
    CONNECTION.clone()
}
