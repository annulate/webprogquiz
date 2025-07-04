use actix_web::{HttpResponse, dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{encode, decode, Header, Validation, Algorithm, TokenData};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use once_cell::sync::Lazy;
use std::env;

static JWT_ENCODING_KEY: Lazy<jsonwebtoken::EncodingKey> = Lazy::new(|| {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    jsonwebtoken::EncodingKey::from_secret(secret.as_bytes())
});
static JWT_DECODING_KEY: Lazy<jsonwebtoken::DecodingKey> = Lazy::new(|| {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    jsonwebtoken::DecodingKey::from_secret(secret.as_bytes())
});

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

pub fn hash_password(password: &str) -> String {
    use sha2::{Sha256, Digest};
    let salt = "bugtrack2025";
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    hasher.update(salt.as_bytes());
    hex::encode(hasher.finalize())
}

pub fn create_jwt(username: &str, role: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now() + Duration::hours(24);
    let claims = Claims {
        sub: username.to_owned(),
        role: role.to_owned(),
        exp: expiration.timestamp() as usize,
    };
    encode(&Header::default(), &claims, &JWT_ENCODING_KEY)
}

pub async fn auth_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token = credentials.token();
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;

    match decode::<Claims>(token, &JWT_DECODING_KEY, &validation) {
        Ok(TokenData { claims, .. }) => {
            req.extensions_mut().insert(claims);
            Ok(req)
        }
        Err(_) => Err((actix_web::error::ErrorUnauthorized("Invalid token"), req)),
    }
}

pub fn error_response<E: std::fmt::Display>(err: E) -> HttpResponse {
    HttpResponse::InternalServerError().body(err.to_string())
}