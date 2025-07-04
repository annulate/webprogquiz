mod models;
mod handlers;
mod database;
mod auth;
mod state;

use actix_web::{web, App, HttpServer, middleware::Logger};
use state::AppState;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    
    println!("🚀 Starting Bug Tracker Server...");
    
    let db_pool = database::create_connection().await
        .expect("❌ Failed to create database connection");
    
    println!("✅ Database connection established");
    
    let app_state = AppState::new(db_pool);
    
    let server_url = "127.0.0.1:8080";
    println!("🌐 Server starting at http://{}", server_url);
    println!("🏠 Homepage available at: http://{}/", server_url);
    println!("📚 Available endpoints:");
    println!("   GET  /                   - Homepage");
    println!("   POST /bugs/new           - Create new bug");
    println!("   GET  /bugs               - List all bugs");
    println!("   GET  /bugs/:id           - Get specific bug");
    println!("   PATCH /bugs/:id          - Update bug");
    println!("   DELETE /bugs/:id         - Delete bug");
    println!("   GET  /bugs/assign        - Bug assignment form");
    println!("   POST /bugs/assign        - Submit bug assignment");
    println!("   GET  /developers         - List developers");
    println!("   POST /developers         - Add new developer");
    println!("   GET  /projects           - List projects");
    println!("   POST /projects           - Add new project (admin)");
    println!("   POST /login              - User authentication");
    println!("   GET  /health             - Health check");
    println!("📖 Default admin credentials: admin/admin123");
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(Logger::default())
            // Homepage route
            .route("/", web::get().to(handlers::homepage))
            // Bug management routes
            // Bug management routes
            .service(
                web::scope("/bugs")
                    .route("/new", web::post().to(handlers::create_bug))
                    .route("/assign", web::get().to(handlers::bug_assign_form))     // Move this UP
                    .route("/assign", web::post().to(handlers::assign_bug))        // Move this UP
                    .route("", web::get().to(handlers::get_bugs))
                    .route("/{id}", web::get().to(handlers::get_bug))              // Keep this AFTER assign
                    .route("/{id}", web::patch().to(handlers::update_bug))
                    .route("/{id}", web::delete().to(handlers::delete_bug))
            )
            // Developer management routes
            .service(
                web::scope("/developers")
                    .route("", web::get().to(handlers::get_developers))
                    .route("", web::post().to(handlers::create_developer))
            )
            // Project management routes
            .service(
                web::scope("/projects")
                    .route("", web::get().to(handlers::get_projects))
                    .route("", web::post().to(handlers::add_project))
            )
            // Authentication routes (MOVED OUTSIDE PROJECTS SCOPE)
            .route("/login", web::post().to(handlers::login))
            // Health check endpoint
            .route("/health", web::get().to(handlers::health_check))
            // Debug routes
            .route("/fix-admin", web::post().to(handlers::fix_admin))
            .route("/protected", web::get().to(handlers::protected_endpoint))
    })
    .bind(server_url)?
    .run()
    .await
}