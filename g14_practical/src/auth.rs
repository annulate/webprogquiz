use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use chrono::{Utc, Duration};
use crate::models::{User, Claims};

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    let salt = "bugtrack2025";
    let salted_password = format!("{}{}", salt, password);
    hash(salted_password, DEFAULT_COST)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    let salt = "bugtrack2025";
    let salted_password = format!("{}{}", salt, password);
    verify(salted_password, hash)
}

pub fn generate_jwt(user: &User) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "your-super-secret-jwt-key-bugtrack2025".to_string());
    
    let exp = (Utc::now() + Duration::hours(24)).timestamp() as usize;
    
    let claims = Claims {
        sub: user.username.clone(),
        role: user.role.clone(),
        exp,
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

pub fn verify_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "your-super-secret-jwt-key-bugtrack2025".to_string());
    
    let validation = Validation::default();
    
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )?;
    
    Ok(token_data.claims)
}

pub async fn authenticate_user(
    pool: &sqlx::SqlitePool,
    username: &str,
    password: &str,
) -> Result<Option<User>, Box<dyn std::error::Error>> {
    if let Some(user) = crate::database::get_user_by_username(pool, username).await? {
        if verify_password(password, &user.password_hash)? {
            return Ok(Some(user));
        }
    }
    Ok(None)
}