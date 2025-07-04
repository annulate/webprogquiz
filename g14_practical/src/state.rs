use sqlx::SqlitePool;
use crate::models::Project;
use std::sync::{Arc, Mutex};

/// Shared application state
/// - `db_pool` handles database operations
/// - `projects` maintains an in-memory list of active projects
#[derive(Clone)]
pub struct AppState {
    pub db_pool: SqlitePool,
    pub projects: Arc<Mutex<Vec<Project>>>,
}