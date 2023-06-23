use base64::{Engine,engine::general_purpose::STANDARD};
use ring::pbkdf2;
use std::num::NonZeroU32;

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA512;

#[derive(Clone)]
pub struct Authentication {
    iterations: NonZeroU32
}

pub fn new() -> Authentication {
    let iterations = NonZeroU32::new(1000).unwrap();
    Authentication { iterations }
}

impl Authentication {

    pub fn verify_password(
        &self,
        salt: &str,
        actual_password_hash: &str,
        attempted_password: &str,
    ) -> Result<(), &'static str> {
        let decoded_salt = STANDARD.decode(salt).unwrap();
        let decoded_actual_password_hash = STANDARD.decode(actual_password_hash).unwrap();

        pbkdf2::verify(
            PBKDF2_ALG,
            self.iterations,
            decoded_salt.as_slice(),
            attempted_password.as_bytes(),
            decoded_actual_password_hash.as_slice(),
        )
            .map_err(|_| "Parola este incorecta")
    }
}
