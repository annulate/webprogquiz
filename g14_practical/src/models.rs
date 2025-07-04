use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

/// Represents a user in the system
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub user_id: i64,
    pub username: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
}

/// Represents a developer profile that bugs can be assigned to
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Developer {
    pub developer_id: i64,
    pub name: String,
}

/// Represents a project under which bugs are tracked
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Project {
    pub project_id: i64,
    pub name: String,
}

/// Represents a bug report
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Bug {
    pub bug_id: i64,
    pub title: String,
    pub description: String,
    pub reported_by: String,
    pub severity: String,
    pub status: String,
    pub project_id: Option<i64>,
    pub assigned_to: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}