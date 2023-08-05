use crate::runtime::db::{SqlxResult, pool};


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