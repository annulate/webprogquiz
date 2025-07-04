use crate::models::Project;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub projects: Arc<RwLock<Vec<Project>>>,
    pub db: sqlx::SqlitePool,
}

impl AppState {
    pub fn new(db: sqlx::SqlitePool) -> Self {
        let initial_projects = vec![
            Project {
                id: 1,
                name: "Frontend Development".to_string(),
                description: "User interface and UX improvements".to_string(),
                active: true,
            },
            Project {
                id: 2,
                name: "Backend API".to_string(),
                description: "Server-side development and database optimization".to_string(),
                active: true,
            },
            Project {
                id: 3,
                name: "Mobile App".to_string(),
                description: "iOS and Android application development".to_string(),
                active: true,
            },
        ];

        Self {
            projects: Arc::new(RwLock::new(initial_projects)),
            db,
        }
    }
    
    pub async fn get_next_project_id(&self) -> i64 {
        let projects = self.projects.read().await;
        projects.iter().map(|p| p.id).max().unwrap_or(0) + 1
    }
}