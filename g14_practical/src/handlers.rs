use actix_web::{web, HttpResponse};
use actix_web::web::ServiceConfig;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::Row;
use crate::state::AppState;
use crate::models::Project;
use crate::utils::{hash_password, create_jwt, error_response};

// ----- AUTH HANDLERS -----
#[derive(Deserialize)]
pub struct AuthData { pub username: String, pub password: String }

#[derive(Serialize)]
struct AuthResponse { status: String, token: Option<String> }

/// Register a new user
pub async fn register(data: web::Data<AppState>, form: web::Json<AuthData>) -> HttpResponse {
    let pool = &data.db_pool;
    let hashed = hash_password(&form.password);
    let res = sqlx::query("INSERT INTO users (username, password_hash) VALUES (?, ?)")
        .bind(&form.username)
        .bind(&hashed)
        .execute(pool)
        .await;
    match res {
        Ok(_) => HttpResponse::Ok().json(AuthResponse { status: "success".into(), token: None }),
        Err(e) => {
            if let sqlx::Error::Database(db_err) = &e {
                if db_err.message().contains("UNIQUE constraint failed: users.username") {
                    return HttpResponse::Conflict().body("Username already exists");
                }
            }
            error_response(e)
        }
    }
}

/// Login and issue JWT on success
pub async fn login(data: web::Data<AppState>, form: web::Json<AuthData>) -> HttpResponse {
    let pool = &data.db_pool;
    let hashed = hash_password(&form.password);
    let row = sqlx::query("SELECT password_hash, role FROM users WHERE username = ?")
        .bind(&form.username)
        .fetch_optional(pool)
        .await;
    match row {
        Ok(Some(r)) if r.get::<String, _>("password_hash") == hashed => {
            let role = r.get::<String, _>("role");
            match create_jwt(&form.username, &role) {
                Ok(tok) => HttpResponse::Ok().json(AuthResponse { status: "success".into(), token: Some(tok) }),
                Err(e) => error_response(e),
            }
        }
        Ok(_) => HttpResponse::Unauthorized().body("Invalid credentials"),
        Err(e) => error_response(e),
    }
}

// ----- BUG HANDLERS -----
#[derive(Deserialize)]
pub struct Pagination { pub page: Option<u32>, pub per_page: Option<u32> }

#[derive(Serialize)]
pub struct BugResponse {
    pub bug_id: i64,
    pub title: String,
    pub description: String,
    pub reported_by: String,
    pub severity: String,
    pub status: String,
    pub project_id: Option<i64>,
    pub assigned_to: Option<i64>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
pub struct NewBug { pub title: String, pub description: String, pub reported_by: String, pub severity: String, pub project_id: Option<i64> }
#[derive(Deserialize)]
pub struct UpdateBug { pub title: Option<String>, pub description: Option<String>, pub severity: Option<String>, pub status: Option<String>, pub assigned_to: Option<i64> }

/// List bugs with pagination
pub async fn list_bugs(data: web::Data<AppState>, query: web::Query<Pagination>) -> HttpResponse {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(10).min(100);
    let limit = per_page as i64;
    let offset = ((page - 1) * per_page) as i64;

    let rows = sqlx::query(
        "SELECT bug_id, title, description, reported_by, severity, status, project_id, assigned_to, created_at, updated_at FROM bugs ORDER BY created_at DESC LIMIT ? OFFSET ?"
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&data.db_pool)
    .await;

    match rows {
        Ok(rows) => {
            let bugs: Vec<BugResponse> = rows.into_iter().map(|r| BugResponse {
                bug_id: r.get("bug_id"),
                title: r.get("title"),
                description: r.get("description"),
                reported_by: r.get("reported_by"),
                severity: r.get("severity"),
                status: r.get("status"),
                project_id: r.get("project_id"),
                assigned_to: r.get("assigned_to"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
            }).collect();
            HttpResponse::Ok().json(bugs)
        }
        Err(e) => error_response(e),
    }
}

/// Create a new bug
pub async fn create_bug(data: web::Data<AppState>, json: web::Json<NewBug>) -> HttpResponse {
    let b = json.into_inner();
    let res = sqlx::query("INSERT INTO bugs (title, description, reported_by, severity, project_id) VALUES (?, ?, ?, ?, ?)")
        .bind(&b.title)
        .bind(&b.description)
        .bind(&b.reported_by)
        .bind(&b.severity)
        .bind(b.project_id)
        .execute(&data.db_pool)
        .await;
    match res {
        Ok(r) => HttpResponse::Created().json(json!({ "bug_id": r.last_insert_rowid() })),
        Err(e) => error_response(e),
    }
}

/// Get a single bug by ID
pub async fn get_bug(data: web::Data<AppState>, path: web::Path<(i64,)>) -> HttpResponse {
    let id = path.into_inner().0;
    let rec = sqlx::query("SELECT bug_id, title, description, reported_by, severity, status, project_id, assigned_to, created_at, updated_at FROM bugs WHERE bug_id = ?")
        .bind(id)
        .fetch_optional(&data.db_pool)
        .await;
    match rec {
        Ok(Some(r)) => HttpResponse::Ok().json(BugResponse {
            bug_id: r.get("bug_id"),
            title: r.get("title"),
            description: r.get("description"),
            reported_by: r.get("reported_by"),
            severity: r.get("severity"),
            status: r.get("status"),
            project_id: r.get("project_id"),
            assigned_to: r.get("assigned_to"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }),
        Ok(None) => HttpResponse::NotFound().body("Bug not found"),
        Err(e) => error_response(e),
    }
}

/// Update an existing bug
pub async fn update_bug(data: web::Data<AppState>, path: web::Path<(i64,)>, json: web::Json<UpdateBug>) -> HttpResponse {
    let id = path.into_inner().0;
    let u = json.into_inner();
    let res = sqlx::query(
        "UPDATE bugs SET title = COALESCE(?, title), description = COALESCE(?, description), severity = COALESCE(?, severity), status = COALESCE(?, status), assigned_to = COALESCE(?, assigned_to), updated_at = CURRENT_TIMESTAMP WHERE bug_id = ?"
    )
    .bind(u.title)
    .bind(u.description)
    .bind(u.severity)
    .bind(u.status)
    .bind(u.assigned_to)
    .bind(id)
    .execute(&data.db_pool)
    .await;
    match res {
        Ok(info) if info.rows_affected() > 0 => HttpResponse::Ok().body("Updated"),
        Ok(_) => HttpResponse::NotFound().body("Bug not found"),
        Err(e) => error_response(e),
    }
}

/// Delete a bug
pub async fn delete_bug(data: web::Data<AppState>, path: web::Path<(i64,)>) -> HttpResponse {
    let id = path.into_inner().0;
    let res = sqlx::query("DELETE FROM bugs WHERE bug_id = ?")
        .bind(id)
        .execute(&data.db_pool)
        .await;
    match res {
        Ok(info) if info.rows_affected() > 0 => HttpResponse::NoContent().finish(),
        Ok(_) => HttpResponse::NotFound().body("Bug not found"),
        Err(e) => error_response(e),
    }
}

// ----- PROJECT HANDLERS -----
#[derive(Deserialize)]
pub struct NewProject { pub name: String }

/// List projects
pub async fn list_projects(data: web::Data<AppState>) -> HttpResponse {
    let rows = sqlx::query("SELECT project_id, name FROM projects")
        .fetch_all(&data.db_pool)
        .await;
    match rows {
        Ok(rows) => {
            let projects: Vec<Project> = rows.into_iter().map(|r| Project {
                project_id: r.get("project_id"),
                name: r.get("name"),
            }).collect();
            HttpResponse::Ok().json(projects)
        }
        Err(e) => error_response(e),
    }
}

/// Create a new project
pub async fn create_project(data: web::Data<AppState>, json: web::Json<NewProject>) -> HttpResponse {
    let name = json.name.clone();
    let res = sqlx::query("INSERT INTO projects (name) VALUES (?)")
        .bind(&name)
        .execute(&data.db_pool)
        .await;
    match res {
        Ok(r) => {
            let project = Project { project_id: r.last_insert_rowid(), name };
            if let Ok(mut vec) = data.projects.lock() {
                vec.push(project.clone());
            }
            HttpResponse::Created().json(project)
        }
        Err(e) => error_response(e),
    }
}

// ----- ASSIGN HANDLER -----
#[derive(Deserialize)]
pub struct AssignData { pub developer_id: i64 }

/// Assign a bug to a developer
pub async fn assign_bug(data: web::Data<AppState>, path: web::Path<(i64,)>, json: web::Json<AssignData>) -> HttpResponse {
    let bug_id = path.into_inner().0;
    let dev_id = json.developer_id;
    let res = sqlx::query("UPDATE bugs SET assigned_to = ?, updated_at = CURRENT_TIMESTAMP WHERE bug_id = ?")
        .bind(dev_id)
        .bind(bug_id)
        .execute(&data.db_pool)
        .await;
    match res {
        Ok(r) if r.rows_affected() > 0 => HttpResponse::Ok().body("Bug assigned"),
        Ok(_) => HttpResponse::NotFound().body("Bug not found"),
        Err(e) => error_response(e),
    }
}

/// Mount all protected and public routes
pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api")
            // BUG routes
            .service(
                web::scope("/bugs")
                    .route("", web::get().to(list_bugs))
                    .route("", web::post().to(create_bug))
                    .service(
                        web::scope("/{id}/assign")
                            .route("", web::post().to(assign_bug))
                    )
                    .route("/{id}", web::get().to(get_bug))
                    .route("/{id}", web::patch().to(update_bug))
                    .route("/{id}", web::delete().to(delete_bug)),
            )
            // PROJECT routes
            .service(
                web::scope("/projects")
                    .route("", web::get().to(list_projects))
                    .route("", web::post().to(create_project)),
            ),
    );
}