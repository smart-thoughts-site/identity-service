use std::sync::OnceLock;
use std::time::Duration;
use sqlx::{
    self,
    postgres::{
        PgConnectOptions,
        PgPoolOptions
    },
    Pool,
    Postgres
};

static PG_POOL: OnceLock<Pool<Postgres>> = OnceLock::new();

pub async fn init() -> () {
    let options = PgConnectOptions::new()
        .ssl_mode(sqlx::postgres::PgSslMode::Require);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .idle_timeout(Duration::from_secs(60))
        .connect_with(options)
        // use your own credentials
        .await
        .expect("couldn't connect to the database");

    PG_POOL.set(pool).expect("couldn't initialize PG_POOL");
}

pub fn pool() -> &'static Pool<Postgres> {
    PG_POOL.get().unwrap()
}

pub type SqlxResult<T> = Result<T, sqlx::error::Error>;
