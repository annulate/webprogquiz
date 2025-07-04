use actix_web::{web, HttpResponse, Result, HttpRequest};
use crate::models::*;
use sqlx::Row;  
use crate::state::AppState;
use crate::database;
use crate::auth;

// Homepage
pub async fn homepage() -> Result<HttpResponse> {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Bug Tracker System</title>
    <style>
        body { 
            font-family: Arial, sans-serif; 
            max-width: 800px; 
            margin: 50px auto; 
            padding: 20px; 
            background-color: #f8f9fa;
        }
        .header { text-align: center; margin-bottom: 40px; }
        .section { background: white; padding: 20px; margin: 20px 0; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }
        .endpoint { margin: 10px 0; padding: 8px; background: #e9ecef; border-radius: 4px; }
        .method { font-weight: bold; color: #007bff; }
        a { color: #007bff; text-decoration: none; }
        a:hover { text-decoration: underline; }
        .nav-links { text-align: center; margin: 20px 0; }
        .nav-links a { 
            display: inline-block; 
            margin: 10px; 
            padding: 10px 20px; 
            background: #007bff; 
            color: white; 
            border-radius: 4px; 
            text-decoration: none;
        }
        .nav-links a:hover { background: #0056b3; }
    </style>
</head>
<body>
    <div class="header">
        <h1>🐛 Bug Tracker System</h1>
        <p>CSC1106 Web Programming - Group 14</p>
    </div>

    <div class="nav-links">
        <a href="/bugs/assign">🔧 Assign Bugs</a>
        <a href="/health">❤️ Health Check</a>
    </div>

    <div class="section">
        <h2>📚 API Endpoints</h2>
        
        <h3>🐛 Bug Management</h3>
        <div class="endpoint"><span class="method">POST</span> /bugs/new - Create new bug</div>
        <div class="endpoint"><span class="method">GET</span> /bugs - List all bugs</div>
        <div class="endpoint"><span class="method">GET</span> /bugs/:id - Get specific bug</div>
        <div class="endpoint"><span class="method">PATCH</span> /bugs/:id - Update bug</div>
        <div class="endpoint"><span class="method">DELETE</span> /bugs/:id - Delete bug</div>
        
        <h3>👥 Developer Management</h3>
        <div class="endpoint"><span class="method">GET</span> /developers - List developers</div>
        <div class="endpoint"><span class="method">POST</span> /developers - Add developer</div>
        
        <h3>📁 Project Management</h3>
        <div class="endpoint"><span class="method">GET</span> /projects - List projects</div>
        <div class="endpoint"><span class="method">POST</span> /projects - Add project</div>
        
        <h3>🔐 Authentication</h3>
        <div class="endpoint"><span class="method">POST</span> /login - User login</div>
    </div>

    <div class="section">
        <h2>🧪 Quick Test Commands</h2>
        <h3>Create a Bug:</h3>
        <pre style="background: #f1f3f4; padding: 10px; border-radius: 4px; overflow-x: auto;">
curl -X POST http://localhost:8080/bugs/new \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Sample Bug",
    "description": "This is a test bug",
    "reported_by": "test@example.com",
    "severity": "Medium"
  }'</pre>

        <h3>Login:</h3>
        <pre style="background: #f1f3f4; padding: 10px; border-radius: 4px; overflow-x: auto;">
curl -X POST http://localhost:8080/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "admin123"}'</pre>

        <h3>Get All Bugs:</h3>
        <pre style="background: #f1f3f4; padding: 10px; border-radius: 4px; overflow-x: auto;">
curl http://localhost:8080/bugs</pre>
    </div>

    <div class="section">
        <h2>👤 Default Users</h2>
        <ul>
            <li><strong>Admin:</strong> username: admin, password: admin123</li>
            <li><strong>Developer 1:</strong> ID: 1 - John Developer</li>
            <li><strong>Developer 2:</strong> ID: 2 - Jane Smith</li>
            <li><strong>Developer 3:</strong> ID: 3 - Bob Wilson</li>
        </ul>
    </div>

    <div class="section">
        <h2>🚀 Features Implemented</h2>
        <ul>
            <li>✅ Full CRUD operations for bugs</li>
            <li>✅ Bug assignment with HTML forms</li>
            <li>✅ Project state management</li>
            <li>✅ User authentication with bcrypt + JWT</li>
            <li>✅ Developer management system</li>
            <li>✅ Dynamic HTML templates</li>
            <li>✅ Comprehensive error handling</li>
            <li>✅ SQLite database integration</li>
        </ul>
    </div>
</body>
</html>
    "#;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

// Health check
pub async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "Bug Tracker API",
        "version": "1.0.0"
    })))
}

// Bug CRUD Operations
pub async fn create_bug(
    app_state: web::Data<AppState>,
    bug: web::Json<NewBug>,
) -> Result<HttpResponse> {
    if bug.title.trim().is_empty() {
        return Ok(HttpResponse::BadRequest().json(
            ApiResponse::<()>::error("Title is required")
        ));
    }

    match database::create_bug(&app_state.db, &bug).await {
        Ok(created_bug) => {
            println!("✅ Created bug #{:?}: {}", created_bug.id, created_bug.title);
            Ok(HttpResponse::Created().json(
                ApiResponse::success_with_id(created_bug.clone(), "Bug created successfully", created_bug.id.unwrap_or(0))
            ))
        }
        Err(e) => {
            eprintln!("❌ Failed to create bug: {}", e);
            Ok(HttpResponse::InternalServerError().json(
                ApiResponse::<()>::error("Failed to create bug")
            ))
        }
    }
}

pub async fn get_bugs(app_state: web::Data<AppState>) -> Result<HttpResponse> {
    match database::get_all_bugs(&app_state.db).await {
        Ok(bugs) => {
            println!("📋 Retrieved {} bugs", bugs.len());
            Ok(HttpResponse::Ok().json(bugs))
        }
        Err(e) => {
            eprintln!("❌ Failed to fetch bugs: {}", e);
            Ok(HttpResponse::InternalServerError().json(
                ApiResponse::<()>::error("Failed to fetch bugs")
            ))
        }
    }
}

pub async fn get_bug(
    app_state: web::Data<AppState>,
    path: web::Path<i64>,
) -> Result<HttpResponse> {
    let bug_id = path.into_inner();
    
    match database::get_bug_by_id(&app_state.db, bug_id).await {
        Ok(Some(bug)) => {
            println!("🔍 Retrieved bug #{}", bug_id);
            Ok(HttpResponse::Ok().json(bug))
        }
        Ok(None) => {
            println!("⚠️  Bug #{} not found", bug_id);
            Ok(HttpResponse::NotFound().json("Bug not found"))
        }
        Err(e) => {
            eprintln!("❌ Database error retrieving bug #{}: {}", bug_id, e);
            Ok(HttpResponse::InternalServerError().json("Database error"))
        }
    }
}

pub async fn update_bug(
    app_state: web::Data<AppState>,
    path: web::Path<i64>,
    bug: web::Json<Bug>,
) -> Result<HttpResponse> {
    let bug_id = path.into_inner();
    
    if bug.title.trim().is_empty() {
        return Ok(HttpResponse::BadRequest().json("Title is required"));
    }
    
    match database::update_bug(&app_state.db, bug_id, &bug).await {
        Ok(updated_bug) => {
            println!("✏️  Updated bug #{}", bug_id);
            Ok(HttpResponse::Ok().json(updated_bug))
        }
        Err(sqlx::Error::RowNotFound) => {
            Ok(HttpResponse::NotFound().json("Bug not found"))
        }
        Err(e) => {
            eprintln!("❌ Failed to update bug #{}: {}", bug_id, e);
            Ok(HttpResponse::InternalServerError().json("Failed to update bug"))
        }
    }
}

pub async fn delete_bug(
    app_state: web::Data<AppState>,
    path: web::Path<i64>,
) -> Result<HttpResponse> {
    let bug_id = path.into_inner();
    
    match database::delete_bug(&app_state.db, bug_id).await {
        Ok(true) => {
            println!("🗑️  Deleted bug #{}", bug_id);
            Ok(HttpResponse::Ok().json("Bug deleted successfully"))
        }
        Ok(false) => {
            Ok(HttpResponse::NotFound().json("Bug not found"))
        }
        Err(e) => {
            eprintln!("❌ Failed to delete bug #{}: {}", bug_id, e);
            Ok(HttpResponse::InternalServerError().json("Failed to delete bug"))
        }
    }
}

// Developer Management
pub async fn get_developers(app_state: web::Data<AppState>) -> Result<HttpResponse> {
    match database::get_all_developers(&app_state.db).await {
        Ok(developers) => {
            println!("👥 Retrieved {} developers", developers.len());
            Ok(HttpResponse::Ok().json(developers))
        }
        Err(e) => {
            eprintln!("❌ Failed to fetch developers: {}", e);
            Ok(HttpResponse::InternalServerError().json("Failed to fetch developers"))
        }
    }
}

pub async fn create_developer(
    app_state: web::Data<AppState>,
    developer: web::Json<NewDeveloper>,
) -> Result<HttpResponse> {
    if developer.name.trim().is_empty() {
        return Ok(HttpResponse::BadRequest().json("Developer name is required"));
    }

    match database::create_developer(&app_state.db, &developer).await {
        Ok(created_developer) => {
            println!("✅ Created developer #{}: {}", created_developer.id, created_developer.name);
            Ok(HttpResponse::Created().json(created_developer))
        }
        Err(e) => {
            eprintln!("❌ Failed to create developer: {}", e);
            Ok(HttpResponse::InternalServerError().json("Failed to create developer"))
        }
    }
}

// Project Management
pub async fn get_projects(app_state: web::Data<AppState>) -> Result<HttpResponse> {
    let projects = app_state.projects.read().await;
    println!("📁 Retrieved {} projects", projects.len());
    Ok(HttpResponse::Ok().json(projects.clone()))
}

pub async fn add_project(
    app_state: web::Data<AppState>,
    project: web::Json<NewProject>,
) -> Result<HttpResponse> {
    if project.name.trim().is_empty() {
        return Ok(HttpResponse::BadRequest().json("Project name is required"));
    }
    
    let mut projects = app_state.projects.write().await;
    let new_id = app_state.get_next_project_id().await;
    
    let new_project = Project {
        id: new_id,
        name: project.name.clone(),
        description: project.description.clone(),
        active: true,
    };
    
    projects.push(new_project.clone());
    println!("📁 Created project #{}: {}", new_id, project.name);
    
    Ok(HttpResponse::Created().json(new_project))
}

// Authentication
pub async fn login(
    app_state: web::Data<AppState>,
    credentials: web::Json<LoginRequest>,
) -> Result<HttpResponse> {
    if credentials.username.trim().is_empty() || credentials.password.trim().is_empty() {
        return Ok(HttpResponse::BadRequest().json(LoginResponse {
            status: "failure".to_string(),
            token: None,
            message: "Username and password are required".to_string(),
        }));
    }

    match auth::authenticate_user(&app_state.db, &credentials.username, &credentials.password).await {
        Ok(Some(user)) => {
            match auth::generate_jwt(&user) {
                Ok(token) => {
                    println!("🔐 User '{}' logged in successfully", credentials.username);
                    Ok(HttpResponse::Ok().json(LoginResponse {
                        status: "success".to_string(),
                        token: Some(token),
                        message: "Login successful".to_string(),
                    }))
                }
                Err(e) => {
                    eprintln!("❌ JWT generation error: {}", e);
                    Ok(HttpResponse::InternalServerError().json(LoginResponse {
                        status: "failure".to_string(),
                        token: None,
                        message: "Token generation failed".to_string(),
                    }))
                }
            }
        }
        Ok(None) => {
            println!("🚫 Failed login attempt for '{}'", credentials.username);
            Ok(HttpResponse::Unauthorized().json(LoginResponse {
                status: "failure".to_string(),
                token: None,
                message: "Invalid username or password".to_string(),
            }))
        }
        Err(e) => {
            eprintln!("❌ Authentication error: {}", e);
            Ok(HttpResponse::InternalServerError().json(LoginResponse {
                status: "failure".to_string(),
                token: None,
                message: "Authentication system error".to_string(),
            }))
        }
    }
}

// Bug Assignment (HTML)
pub async fn bug_assign_form(app_state: web::Data<AppState>) -> Result<HttpResponse> {
    let bugs = database::get_all_bugs(&app_state.db).await.unwrap_or_default();
    let developers = database::get_all_developers(&app_state.db).await.unwrap_or_default();
    
    let html = format!(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Bug Assignment</title>
    <style>
        body {{ font-family: Arial, sans-serif; max-width: 800px; margin: 50px auto; padding: 20px; }}
        .form-group {{ margin-bottom: 15px; }}
        label {{ display: block; margin-bottom: 5px; font-weight: bold; }}
        select, input {{ width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px; }}
        button {{ background-color: #007bff; color: white; padding: 10px 20px; border: none; border-radius: 4px; cursor: pointer; }}
        button:hover {{ background-color: #0056b3; }}
        .info {{ background-color: #f8f9fa; padding: 15px; border-radius: 4px; margin-bottom: 20px; }}
    </style>
</head>
<body>
    <h1>🐛 Bug Assignment</h1>
    
    <div class="info">
        <h3>Available Bugs:</h3>
        <ul>
            {}
        </ul>
        
        <h3>Available Developers:</h3>
        <ul>
            {}
        </ul>
    </div>
    
    <form method="post" action="/bugs/assign">
        <div class="form-group">
            <label for="bug_id">Bug ID:</label>
            <select id="bug_id" name="bug_id" required>
                <option value="">Select a bug...</option>
                {}
            </select>
        </div>
        
        <div class="form-group">
            <label for="developer_id">Developer:</label>
            <select id="developer_id" name="developer_id" required>
                <option value="">Select a developer...</option>
                {}
            </select>
        </div>
        
        <button type="submit">Assign Bug</button>
    </form>
    
    <div style="margin-top: 20px;">
        <a href="/" style="color: #007bff;">← Back to Homepage</a>
    </div>
</body>
</html>
    "#,
        bugs.iter().map(|b| format!("<li>#{} - {}</li>", b.id.unwrap_or(0), b.title)).collect::<Vec<_>>().join(""),
        developers.iter().map(|d| format!("<li>#{} - {}</li>", d.id, d.name)).collect::<Vec<_>>().join(""),
        bugs.iter().map(|b| format!("<option value=\"{}\">{} - {}</option>", b.id.unwrap_or(0), b.id.unwrap_or(0), b.title)).collect::<Vec<_>>().join(""),
        developers.iter().map(|d| format!("<option value=\"{}\">{} - {}</option>", d.id, d.id, d.name)).collect::<Vec<_>>().join("")
    );
    
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

// Add this simple function to src/handlers.rs
pub async fn fix_admin(app_state: web::Data<AppState>) -> Result<HttpResponse> {
    // Get the correct hash using your auth function
    let correct_hash = crate::auth::hash_password("admin123").unwrap();
    
    // Update the admin user's password
    let result = sqlx::query("UPDATE users SET password_hash = ? WHERE username = 'admin'")
        .bind(&correct_hash)
        .execute(&app_state.db)
        .await;
    
    match result {
        Ok(_) => Ok(HttpResponse::Ok().json("Admin password fixed")),
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e)))
    }
}

pub async fn protected_endpoint(req: HttpRequest) -> Result<HttpResponse> {
    let auth_header = match req.headers().get("Authorization") {
        Some(header) => header.to_str().unwrap_or(""),
        None => return Ok(HttpResponse::Unauthorized().json("Missing Authorization header")),
    };
    
    if !auth_header.starts_with("Bearer ") {
        return Ok(HttpResponse::Unauthorized().json("Invalid Authorization format"));
    }
    
    let token = &auth_header[7..];
    
    match crate::auth::verify_jwt(token) {
        Ok(claims) => {
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "message": "Access granted!",
                "user": claims.sub,
                "role": claims.role,
                "expires": claims.exp
            })))
        }
        Err(_) => {
            Ok(HttpResponse::Unauthorized().json("Invalid or expired token"))
        }
    }
}

// Add these to src/handlers.rs (after the import fix above)

pub async fn debug_users(app_state: web::Data<AppState>) -> Result<HttpResponse> {
    let users = sqlx::query("SELECT id, username, role, password_hash FROM users")
        .fetch_all(&app_state.db)
        .await
        .unwrap_or_default();
    
    let user_info: Vec<serde_json::Value> = users.iter().map(|row| {
        serde_json::json!({
            "id": row.get::<i64, _>("id"),
            "username": row.get::<String, _>("username"),
            "role": row.get::<String, _>("role"),
            "password_hash_length": row.get::<String, _>("password_hash").len()
        })
    }).collect();
    
    Ok(HttpResponse::Ok().json(user_info))
}

pub async fn debug_password() -> Result<HttpResponse> {
    let test_password = "admin123";
    let salt = "bugtrack2025";
    let salted = format!("{}{}", salt, test_password);
    
    let hash1 = bcrypt::hash(&salted, bcrypt::DEFAULT_COST).unwrap();
    let hash2 = bcrypt::hash(&salted, bcrypt::DEFAULT_COST).unwrap();
    
    let verify1 = bcrypt::verify(&salted, &hash1).unwrap();
    let verify2 = bcrypt::verify(&salted, &hash2).unwrap();
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "original_password": test_password,
        "salt": salt,
        "salted_password": salted,
        "hash1": hash1,
        "hash2": hash2,
        "verify1": verify1,
        "verify2": verify2,
        "hashes_different": hash1 != hash2
    })))
}

pub async fn reset_admin(app_state: web::Data<AppState>) -> Result<HttpResponse> {
    // Delete existing admin
    sqlx::query("DELETE FROM users WHERE username = 'admin'")
        .execute(&app_state.db)
        .await
        .unwrap();
    
    // Create new admin with correct hash
    let password_hash = crate::auth::hash_password("admin123").unwrap();
    
    sqlx::query("INSERT INTO users (username, password_hash, role) VALUES (?, ?, ?)")
        .bind("admin")
        .bind(&password_hash)
        .bind("admin")
        .execute(&app_state.db)
        .await
        .unwrap();
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Admin user reset successfully",
        "username": "admin",
        "password": "admin123",
        "hash": password_hash
    })))
}

pub async fn assign_bug(
    app_state: web::Data<AppState>,
    form: web::Form<BugAssignment>,
) -> Result<HttpResponse> {
    let assignment = form.into_inner();
    
    match database::assign_bug(&app_state.db, assignment.bug_id, assignment.developer_id).await {
        Ok(true) => {
            println!("✅ Bug #{} assigned to developer #{}", assignment.bug_id, assignment.developer_id);
            
            let html = format!(r#"
<!DOCTYPE html>
<html>
<head><title>Assignment Successful</title></head>
<body style="font-family: Arial, sans-serif; max-width: 600px; margin: 50px auto; padding: 20px;">
    <h1 style="color: green;">✅ Assignment Successful</h1>
    <p>Bug #{} has been successfully assigned to developer #{}</p>
    <a href="/bugs/assign" style="display: inline-block; padding: 10px 20px; background-color: #007bff; color: white; text-decoration: none; border-radius: 4px;">Back to Assignment Form</a>
    <a href="/" style="display: inline-block; padding: 10px 20px; background-color: #6c757d; color: white; text-decoration: none; border-radius: 4px; margin-left: 10px;">Back to Homepage</a>
</body>
</html>
            "#, assignment.bug_id, assignment.developer_id);
            
            Ok(HttpResponse::Ok().content_type("text/html").body(html))
        }
        Ok(false) => {
            let html = r#"
<!DOCTYPE html>
<html>
<head><title>Assignment Failed</title></head>
<body style="font-family: Arial, sans-serif; max-width: 600px; margin: 50px auto; padding: 20px;">
    <h1 style="color: red;">❌ Assignment Failed</h1>
    <p>Bug or developer not found</p>
    <a href="/bugs/assign" style="display: inline-block; padding: 10px 20px; background-color: #007bff; color: white; text-decoration: none; border-radius: 4px;">Back to Assignment Form</a>
    <a href="/" style="display: inline-block; padding: 10px 20px; background-color: #6c757d; color: white; text-decoration: none; border-radius: 4px; margin-left: 10px;">Back to Homepage</a>
</body>
</html>
            "#;
            Ok(HttpResponse::BadRequest().content_type("text/html").body(html))
        }
        Err(e) => {
            eprintln!("❌ Assignment failed: {}", e);
            Ok(HttpResponse::InternalServerError().json("Assignment failed"))
        }
    }
}