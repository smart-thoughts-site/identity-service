use axum::{
    extract::State,
    headers::UserAgent,
    http::StatusCode,
    response::IntoResponse,
    Json,
    TypedHeader
};

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

// go ahead and run "cargo run main.rs"
// localhost:4000 should now print out your user agent
pub async fn index(TypedHeader(user_agent): TypedHeader<UserAgent>) -> String {
    let mut ret = String::from(user_agent.as_str());
    ret.push_str(": Hello, world!");
    ret
}

/*

{
       "access_token":"2YotnFZFEjr1zCsicMWpAA",
       "token_type":"example",
       "expires_in":3600,
       "refresh_token":"tGzv3JOkF0XG5Qx2TlKWIA",
       "example_parameter":"example_value"
     }

     + usename + roles
*/
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

    let claims = Claims {
        sub: user.email.to_owned(),
        company: "ACME".to_owned(),
        // Mandatory expiry time as UTC timestamp
        exp: 2000000000, // May 2033
    };

    // Create the authorization token
    let access_token = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref()))
        .map_err(|_| unautohirzed("JWTTokenCreationError"))?;

    // let token = uuid::Uuid::new_v4().to_string();
    let response = LoginResponse {
        access_token,
        token_type: "Bearer"
    };

    Ok((StatusCode::OK, Json(response)))
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String
}

#[derive(Serialize)]
pub struct LoginResponse {
    access_token: String,
    token_type: &'static str
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
