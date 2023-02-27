use std::str::FromStr;

use actix_web::{HttpRequest, Result};
use chrono::{Duration, Local};
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;
use argonautica::{Hasher, Verifier};
use jsonwebtoken::errors::*;

use crate::common_utils::UserRole;
use crate::config_variables::TOKEN_DURATION;

lazy_static! {
    static ref JWT_SECRET_KEY: String = 
        std::env::var("JWT_SECRET_KEY").expect("Can't read JWT_SECRET_KEY");
}

lazy_static! {
    static ref PASSWORD_SECRET_KEY: String = 
        std::env::var("PASSWORD_SECRET_KEY").expect("Can't read PASSWORD_SECRET_KEY");
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub role: String,
}

pub fn create_token(user_id: String, role: UserRole) -> String {
    let exp_time = Local::now() + Duration::seconds(TOKEN_DURATION);

    let claims = Claims {
        sub: user_id,
        exp: exp_time.timestamp(),
        role: role.to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET_KEY.as_ref()),
    )
    .expect("Can't create token")
}

pub fn get_claim(http_request: HttpRequest) -> Result<(UserRole, uuid::Uuid, i64), jsonwebtoken::errors::Error> {

    println!("{:?}", &http_request.headers().get("Authorization"));

    let token_data = http_request
        .headers()
        .get("Authorization")
        .and_then(|header_value| {
            header_value.to_str().ok().map(|s| {
                let jwt_start_index = "Bearer ".len();
                let jwt = s[jwt_start_index..s.len()].to_string();
                let token_data = decode_token(&jwt);
                println!("TOKEN: {:?}", &token_data);
                token_data
            })
        });

        let token = match token_data {
            Some(td) => {
                let token = match td {
                    Ok(t) => t,
                    Err(e) => return Err(e),
                };
                token
            },
            None => return Err(jsonwebtoken::errors::Error::from(ErrorKind::InvalidToken)),
        };

        let role = UserRole::from_str(&token.claims.role).expect("Can't parse role");
        let uuid = uuid::Uuid::from_str(&token.claims.sub).expect("Can't parse CBSA_ID");
        let exp_time = &token.claims.exp;

        Ok((role, uuid.to_owned(), *exp_time))
}

pub fn decode_token(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    Ok(decode::<Claims>(
        &token,
        &DecodingKey::from_secret(JWT_SECRET_KEY.as_ref()),
        &Validation::default(),
    )?)
}

pub fn hash_password(password: &str) -> Result<String, argonautica::Error> {
    Hasher::default()
        .with_password(password)
        .with_secret_key(PASSWORD_SECRET_KEY.as_str())
        .hash()
}

pub fn verify_password(hash: &str, password: &str) -> Result<bool, argonautica::Error> {
    Verifier::default()
        .with_hash(hash)
        .with_password(password)
        .with_secret_key(PASSWORD_SECRET_KEY.as_str())
        .verify()
}




