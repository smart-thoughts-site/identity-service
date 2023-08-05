use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::runtime::rest::unautohirzed;
use crate::runtime::rest::database_error;

use super::domain::fetch_user;
use super::jwt::generate_token;
use super::passwords::Passwords;

pub async fn login(
    State(state): State<Passwords>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let maybe_user = fetch_user(&payload.username)
        .await
        .map_err(database_error)?;

    let user = maybe_user.ok_or(unautohirzed("UserNotFound"))?;

    state
        .verify_password(&user.salt, &user.password, &payload.password)
        .map_err(|msg| unautohirzed(msg.to_string()))?;

    // Create the authorization token
    let token_details = generate_token(&user)
        .map_err(|_| unautohirzed("JWTTokenCreationError"))?;

    Ok((StatusCode::OK, Json(token_details)))
}

use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String
}
