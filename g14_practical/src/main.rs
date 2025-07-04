use dotenv::dotenv;
use std::env;
use env_logger;
use actix_web::{App, HttpServer, middleware::Logger, web};
use actix_web_httpauth::middleware::HttpAuthentication;
use sqlx::SqlitePool;
use std::sync::{Arc, Mutex};

mod models;
mod state;
mod handlers;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from `.env`
    dotenv().ok();
    // Initialize logger
    env_logger::init();

    // Connect to the database
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://./bugtrack.db".into());
    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    // Shared application state: database pool + in-memory projects list
    let projects = Arc::new(Mutex::new(Vec::new()));
    let app_state = state::AppState { db_pool: pool.clone(), projects };

    // JWT authentication middleware
    let auth_middleware = HttpAuthentication::bearer(utils::auth_validator);

    // Build and run server
    HttpServer::new(move || {

        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(Logger::default())
            // Public endpoints
            .route("/login", web::post().to(handlers::login))
            .route("/register", web::post().to(handlers::register))
            // Protected endpoints under JWT
            .service(
                web::scope("")
                    .wrap(auth_middleware.clone())
                    .configure(handlers::init),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}