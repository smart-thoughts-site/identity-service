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

fn pool() -> &'static Pool<Postgres> {
    PG_POOL.get().unwrap()
}

pub type SqlxResult<T> = Result<T, sqlx::error::Error>;

#[derive(sqlx::FromRow)]
pub struct User {
    pub username: String,
    pub salt: String,
    pub password: String,
    pub email: String,
}

pub async fn fetch_user(username: &str) -> SqlxResult<Option<User>> {
    let pool = pool();
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(username)
        .fetch_optional(pool)
        .await
}