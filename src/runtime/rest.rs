use axum::{http::StatusCode, TypedHeader, headers::UserAgent};
use serde::Serialize;

pub async fn index(TypedHeader(user_agent): TypedHeader<UserAgent>) -> String {
    let mut ret = String::from(user_agent.as_str());
    ret.push_str(": Hello, world!");
    ret
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
