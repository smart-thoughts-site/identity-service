use axum::{
    extract::State,
    headers::UserAgent,
    http::StatusCode,
    response::IntoResponse,
    Json,
    TypedHeader
};

// go ahead and run "cargo run main.rs"
// localhost:4000 should now print out your user agent
pub async fn index(TypedHeader(user_agent): TypedHeader<UserAgent>) -> String {
    let mut ret = String::from(user_agent.as_str());
    ret.push_str(": Hello, world!");
    ret
}

use crate::authentication;
use crate::persistence;

pub async fn login(
    State(state): State<authentication::Authentication>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let maybe_user = persistence::fetch_user(&payload.username)
        .await
        .map_err(database_error)?;

    let user = maybe_user.ok_or(unautohirzed("UserNotFound"))?;

    state
        .verify_password(&user.salt, &user.password, &payload.password)
        .map_err(|msg| unautohirzed(msg.to_string()))?;

    let token = uuid::Uuid::new_v4().to_string();
    let response = LoginResponse {
        token
    };

    Ok((StatusCode::OK, Json(response)))
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String
}

#[derive(Serialize)]
pub struct BooleanResponse {
    pub result: bool,
}

pub const TRUE_RESPONSE: BooleanResponse = BooleanResponse { result: true };
pub const FALSE_RESPONSE: BooleanResponse = BooleanResponse { result: false };

impl BooleanResponse {
    pub const fn of(result: bool) -> Self {
        if result {
            TRUE_RESPONSE
        } else {
            FALSE_RESPONSE
        }
    }
}

/// Utility function for mapping any error into a `500 Internal Server Error`response.
pub fn internal_error<S>(message: S) -> (StatusCode, String)
    where
        S: Into<String>,
{
    (StatusCode::INTERNAL_SERVER_ERROR, message.into())
}

pub fn database_error(err: sqlx::error::Error) -> (StatusCode, String) {
    tracing::debug!("db error: {:?}", err);
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

pub fn bad_request<S>(message: S) -> (StatusCode, String)
    where
        S: Into<String>,
{
    (StatusCode::BAD_REQUEST, message.into())
}

pub fn unautohirzed<S>(message: S) -> (StatusCode, String)
    where
        S: Into<String>,
{
    (StatusCode::UNAUTHORIZED, message.into())
}

pub fn forbidden<S>(message: S) -> (StatusCode, String)
    where
        S: Into<String>,
{
    (StatusCode::FORBIDDEN, message.into())
}
