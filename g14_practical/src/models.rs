use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Bug {
    pub id: Option<i64>,
    pub title: String,
    pub description: Option<String>,
    pub reported_by: Option<String>,
    pub severity: Option<String>,
    pub developer_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewBug {
    pub title: String,
    pub description: String,
    pub reported_by: String,
    pub severity: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Developer {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewDeveloper {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewProject {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub status: String,
    pub token: Option<String>,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BugAssignment {
    pub bug_id: i64,
    pub developer_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
    pub bug_id: Option<i64>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T, message: &str) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            data: Some(data),
            bug_id: None,
        }
    }
    
    pub fn success_with_id(data: T, message: &str, bug_id: i64) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            data: Some(data),
            bug_id: Some(bug_id),
        }
    }
    
    pub fn error(message: &str) -> Self {
        Self {
            success: false,
            message: message.to_string(),
            data: None,
            bug_id: None,
        }
    }
}

// User model for authentication
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}