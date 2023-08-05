use base64::{Engine,engine::general_purpose::STANDARD};

use jsonwebtoken::{
    decode, 
    encode, 
    errors::Result, 
    DecodingKey, 
    EncodingKey, 
    Header, 
    Validation
};

use serde::{Deserialize, Serialize};

use super::domain::User;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenDetails {
    pub username: String,
    pub access_token: String,
    pub token_type: &'static str,
    pub expires_in: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenClaims {
    sub: String,
    company: String,
    iat: i64,
    exp: i64,
    nbf: i64,
}

/*
    "iss" (Issuer): this issuer of the token
    "sub" (Subject): the subject of the token
    "aud" (Audience): the audience for the token
    "exp" (Expiration Time): the expiration time of the token
    "iat" (Issued At): the time the token was issued
    "nbf" (Not Before): the time before which the token is not valid
 */

const SIGNATURE_SECRET: &'static str = "my_ultra_secure_jwt_secret";

pub fn generate_token(user: &User) -> Result<TokenDetails> {
    let now = chrono::Utc::now();
    let exp = (now + chrono::Duration::minutes(60)).timestamp();

    let claims = TokenClaims {
        sub: user.email.to_owned(),
        company: "ACME".to_owned(),
        // Mandatory expiry time as UTC timestamp
        exp,
        iat: now.timestamp(),
        nbf: now.timestamp(),
    };

    let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256);

    let access_token = jsonwebtoken::encode(
        &header,
        &claims,
        &EncodingKey::from_secret(SIGNATURE_SECRET.as_ref()),
    )?;

    let token_details = TokenDetails {
        username: user.username.to_owned(),
        access_token,
        token_type: "Bearer",
        expires_in: exp
    };

    Ok(token_details)
}