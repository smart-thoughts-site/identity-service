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

const PRIVATE_KEY: &'static str = "LS0tLS1CRUdJTiBSU0EgUFJJVkFURSBLRVktLS0tLQpNSUlKS0FJQkFBS0NBZ0VBME05dm5IWExKZFgzbk1hWXM4OHVvd1dRS21NSWRNMXVzbGN1MUhZdW01NWs1RE1yCm9pclBXcjcyQW5uVUhVZDczSmo2b3kzSGtYMmZrU3NGSkpVSitZdlQrL3RSRHpGdHlMWXJrbUxFVnJNbmVjVSsKeis0RHJVYitDdmkwUitXWmorMDRLdU1JdTNjSU5ONjh5ZWtQSjB4VVRQSm04bWNtT1ZGN1NJUVBxRXJKR3NtRgp2dTJZOEZGdmo5VkluK2Z3ZmRBeHJhRTEyem05WlhkWnloL2QvU05wZUgxWkVXYmVnSmhPTUJzWWlLcVhMS3V5Clc5bm5uRld2QUNTbGtoYjFLVlY0UW1TV0FVVnNnMEdTMGo5QlFrVkQ1TEZBVWpndDlzSzVDRWtxRGhpS1pNQXIKVFpWVU12eDcwTHRoZmpRNng0ZXljVEVNeG10dXRqam1kYXRQcmJFWGZleHNqNTRIVHlwSzNwSU1ncW1OVTFjNQpHYVFlcW5CSElNa0o2MTk1cUZ4WE5HejE5c2liQlkzTlpleE5HWmc0bkdGTjdrUW9QR2FWdHMyYXdiVU4xL0JZCnBjN0FpSnh5RFg5SkFweUFSUWgxcmxDVkdXb3daQ05WRkJ4OWNMTjBDeGpyYi9td0sxSkRmMHFmSms3QmpyVHcKTnVzL1k5NUp5TE1JSHNvTlpRYk1uL095N2pmMXVjV3dNUkRnYjhqSDdxa2tCQ2F3OW1md2djZVE0cVBtZzFsMgovMjVmQzh1eGlJdWRZWCtQZjBaSVVkQ09zTDllT2xYYWJGcTA4UG5jUmFuRzBFcHRsNnV6eTVuNi9waHdEK0R0Cmh1RE5ycURoNjVTUy9uU1JEVWRHbGtITms0RlByZGNRK0kraWtBZDM1RnJVb0l3ajRjT0VLa0JyT1Q4Q0F3RUEKQVFLQ0FnQnpCUkN4MnFEZ1lwQldwMzZON1YzL0pwMVcrOTQ0bU1DVk5EanpoM1g4K3E4UWxLOUFVTnlQWEFrZgpMQVNQYkVUcUtzcEZBSDZod2RVWG5kN2pXOFYyMUhNY3BqN3NZNG5adVo4ZXI1RC9RUWhKcDBFR1FGRitMVkRhCnNreDhIaGtNa3RzUnBLVzJ2Y2FqZU4zOVNvZXlXZlZGdlhDL3JkbjhVTW5jRkFLYjdUWUJyMmdnMTdnYkNJQ3YKZGdqZkxGL29yYm52cnBHQUJMb3pIaDh6bTRJb1lrMUN0YWxPVUovWHJnM0RxZWxGdnRJdkpSVEdTNjJ0Qy9XdAoyb0hwaXdQWWxOLzlrbktlbUtOQldlbUtMcFcvNzIrS2xhaWNvWjJRQTRydzZYeGs3MWVzVDc2S3Flc0xldENwCkZjNktPakwybmVUSlBQK1FmTFVyWXdSdlpNSXFKOVBVQjZUR1BIRVpsSmROQml5VlNya2d2S2R1NjllemJZZmgKQkRJeXh2Mnh4Q0pSTFU1VUJXb2I0YWp6RWlQZkhmSkIvUnNrOGdVNGNrc0Z2U0ZhZHpPU1hlNlZEYjNRR3NZNgozdFFlK2xsem5lOFVFWTg1NGg2L0JiRENWbHVEa2UxNTk5Ny8yam9MUnl0U0EySGxXc1N4MW41SFp5ZDZ1a1NpCkd4bXgvNHN6b2NGZ1FYVnhhMTljdVlIZXFSK2haa3FGaC9EYTh5UVNsOWRHYXh4WkF5RWplMzBWdjdIeEcxQ0MKQjM4RjZSUmh5Qm9LSnpRbnRNVlY2YXc2Q2FZMk43YS9hRFBLWjRONU5YY0dDKzZSRHh3b0M5bFNleXRrbkRCago1UWVIZmJMai9mRzhQWUU1NnRSWnNEZGNNVmg4SllDdk1acG1uUW9Qb0lUYU9PenNRUUtDQVFFQTl0bzZFOXhnCmZTa1NJMHpDYUdLNkQzMnBmdFJXbkR5QWJDVXpOYk5rcEJLOHJuSGlXanFJVDQzbVd6empGc0RvNlZwVXpscFQKYVVHWkNHMXc5THpHaWlaNllBd0F4TXBOZERzMFFOemhJNjMzS0tseHd0NGlUaGQ0aG9oYmZqdndGWHkyQ0paWgovUkkyZ1AwUEdvSENURXFMMTgzQklpYnJJR0g4dzY2K0F5cFc2L3cvdENEQ2NReFA1RE45YlNPSmFlQVI0a1NzCjg3REM1bmdNMVhJeVFpSCtvL21zaEpUS3ZhZUVpeTVmM1BaaExJNWZNQlZwN0tWTUNZY3V2NWZ4Y3pHVHZFM1YKcHcxamJmSzRDdG9xemFmK3hrdUk5ZWNjakp4TU5KRGc0QW5CNEpxWm11Y2dQWGJPdEpRR2VHaHZqZlBqTVZHZworTHhzSUFWZE8vRjFtUUtDQVFFQTJJeFNNK1VZOTFoem5vUURSbzV4WWVGS0dUeDhVZ00rWDdycURzTXp6NUVSCkRWKzh5WlNsY29NVjNlcGVSdjFHYlRodEUvTlZ4c1k2SW5yUkVJNHB2WFJqYkxqZDZPVkJYWENsYVl1YWsyV20KV2QxTVo4dDZRMUtVWXBFS0piZVRMN09SUmtibnIzTHhmWGJ2WTRPV1BaQjZyNktoaXljbTFubUNJU0hiMFh5Mwp1WHY1VVZEYVZWdklnS0RkNGhrRGZSWmEzNEZZUDYvcUFzMzkyWkJnclpvbVk0SkFMN2F0RnpmWVVZMUtlamV3CmpJWCtpQmRkdkd0cXQ0ZzYwQkgzQUxCZjJFb0Q4bkluaHRuUWtSd0d5QnRFN1pRVGdCYzRJbm5mR2tMZTRpWDkKQlZaSFgxb0VHWUp3RkVUNk1zUHFwcU8yWDhPT21YRDFFVFhUTUVjOGx3S0NBUUFmMWQwUG1xaEcrL2orM0hObQpDdlY3OGZUZUNueHhBY3grSmY0SXV1NEx5dTdTZ0pWMGxYL200cUlHdWo5L083bk4vbnhaY0lTNVdtQm1HZGNyCmVQMFI3QXgwUHBnS3lSeGNGUmFVRnVoaU5abGVnUnZPeWQ4YXV5UXNGWUhYTWR1d3FiakFPc080UTVVTDVaY0IKRUNNQ3U4cDFObS9sKzZidk1qUHErS3BBdGtFbmhneWhLbWhwTS9GSnVPcEFIUWtud21JTUVGZE54a29jZHZjUQp2LzJEVWVjSk5yWHRFMU5pU2l4cDFyMCtQZmdpU3VvenhVODMyY21Jb1FxQ1l4SWNqUlJFZ0xWQktoVGNwU1RmCklXdkx3aEsxZUNCZHRrU1VUY1AyTTRrTTI3VkpSaWJ4TjBXTko3bFl5STVkRVByeUQ3WUpNa0hVVWxpUGVLR2gKalc1aEFvSUJBQWdWQktSbk1vMVl3Y2Z5eVdTQ3dIeVV1ZjFESXFpMDhra0VZdVAyS1NMZ0dURFVsK2sySVE2cgpFYy9jaFhSRTA3SVQzdzVWa0tnQWtmN2pjcFlabURrMzlOWUQrRlJPNmllZ29xdlR5QXNrU2hja2lVdCticXZBCmswVXlnSnh6dzR5T09TZlVVYVZjdHVLbDQ3MWxGZUJxV2duZ0dnTmxqSytJalhETElMY3EzbmlQeGZoZytpVWgKYmRSUExMalpraVhEQmRVOXNKdC81MDMvZmkvMmtZVXBNYkdaRk9neSt6YllvTHc2ZDhNai9QVGhzMlJFNnZ5egpUYUpYOVVuNndhdEc2ZXphcGxjUUo2V0N6NlA2MWMzMkpwWnZabUxyZXU3ZWVaTXpWN285RExwOFErR3RMR1gvClZrdUxYNE14aUxwN2RiMFJRV3M4cWdqZ1oyZHY0VFVDZ2dFQkFMRjRiNnhRNjJJaCtMaTdsVk9lSWQ5VFVub08KUU1LUVNRN0xlWjJ4TmhCYWRPUEt0ZmJ5U0dGMGZieXZiVWk2czAyVnJpWC93S1V6T2o1WEFUbUZYQVdzYnU1dwo2M1JVR09ua2Z6cjIwWDZJWTVzOS9kdnJWZXFLNkpLdlQyZ0F0dWMwNXNCZzJPaG5CdHh2c0JDekhYVy9YRWJsCktWamVIMUxQTnZMaFNSc3BvT2FFVUhlaHpNN2c1V3FGSXhSQmRlb2J1SWNxQ1J2WjRFZGl6b05ybzVRZXFub3oKMTlyU0VVcTNBMEdIdE5Pb0xuV2Q3ZkZta2NOMEw5S3R0MTdsK2wxV0c3Y2kxVTVuSXBlOXBxZThlUUU2YmNYaApkNnlkdWd3UUpXbUxKSlpMQUs3eFpZdzd1ODhoa3ppZ2pSR2ltWHZ4VTJCMTU5OW5OT2NrNWQ0YXJTRT0KLS0tLS1FTkQgUlNBIFBSSVZBVEUgS0VZLS0tLS0=";

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

    let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::RS256);

    let bytes_private_key = STANDARD.decode(PRIVATE_KEY).unwrap();
    let decoded_private_key = String::from_utf8(bytes_private_key).unwrap();

    let access_token = jsonwebtoken::encode(
        &header,
        &claims,
        &jsonwebtoken::EncodingKey::from_rsa_pem(decoded_private_key.as_bytes())?,
    )?;

    let token_details = TokenDetails {
        username: user.username.to_owned(),
        access_token,
        token_type: "Bearer",
        expires_in: exp
    };

    Ok(token_details)
}