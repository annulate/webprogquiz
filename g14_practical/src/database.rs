use sqlx::SqlitePool;
use crate::models::{Bug, NewBug, Developer, NewDeveloper, User};
use bcrypt::{hash, DEFAULT_COST};

pub async fn create_connection() -> Result<SqlitePool, sqlx::Error> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:bugs.db".to_string());
    
    let pool = SqlitePool::connect(&database_url).await?;
    
    // Create your existing tables (bugs and developers already exist)
    
    // Add users table for authentication
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            role TEXT DEFAULT 'developer'
        )
        "#,
    )
    .execute(&pool)
    .await?;

    // Insert default users if they don't exist
    let admin_hash = hash("admin123", DEFAULT_COST).unwrap();
    sqlx::query(
        "INSERT OR IGNORE INTO users (username, password_hash, role) VALUES (?, ?, ?)"
    )
    .bind("admin")
    .bind(admin_hash)
    .bind("admin")
    .execute(&pool)
    .await?;
    
    // Insert some default developers if table is empty
    let dev_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM developers")
        .fetch_one(&pool)
        .await?;
    
    if dev_count == 0 {
        sqlx::query("INSERT INTO developers (name) VALUES (?)")
            .bind("John Developer")
            .execute(&pool)
            .await?;
        sqlx::query("INSERT INTO developers (name) VALUES (?)")
            .bind("Jane Smith")
            .execute(&pool)
            .await?;
        sqlx::query("INSERT INTO developers (name) VALUES (?)")
            .bind("Bob Wilson")
            .execute(&pool)
            .await?;
    }

    println!("✅ Database connection established and tables verified");
    Ok(pool)
}

pub async fn create_bug(pool: &SqlitePool, bug: &NewBug) -> Result<Bug, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO bugs (title, description, reported_by, severity) VALUES (?, ?, ?, ?)"
    )
    .bind(&bug.title)
    .bind(&bug.description)
    .bind(&bug.reported_by)
    .bind(&bug.severity)
    .execute(pool)
    .await?;

    let bug_id = result.last_insert_rowid();
    
    let created_bug = sqlx::query_as::<_, Bug>(
        "SELECT * FROM bugs WHERE id = ?"
    )
    .bind(bug_id)
    .fetch_one(pool)
    .await?;

    Ok(created_bug)
}

pub async fn get_all_bugs(pool: &SqlitePool) -> Result<Vec<Bug>, sqlx::Error> {
    let bugs = sqlx::query_as::<_, Bug>("SELECT * FROM bugs ORDER BY id DESC")
        .fetch_all(pool)
        .await?;
    Ok(bugs)
}

pub async fn get_bug_by_id(pool: &SqlitePool, bug_id: i64) -> Result<Option<Bug>, sqlx::Error> {
    let bug = sqlx::query_as::<_, Bug>("SELECT * FROM bugs WHERE id = ?")
        .bind(bug_id)
        .fetch_optional(pool)
        .await?;
    Ok(bug)
}

pub async fn update_bug(pool: &SqlitePool, bug_id: i64, bug: &Bug) -> Result<Bug, sqlx::Error> {
    sqlx::query(
        "UPDATE bugs SET title = ?, description = ?, reported_by = ?, severity = ?, developer_id = ? WHERE id = ?"
    )
    .bind(&bug.title)
    .bind(&bug.description)
    .bind(&bug.reported_by)
    .bind(&bug.severity)
    .bind(&bug.developer_id)
    .bind(bug_id)
    .execute(pool)
    .await?;

    let updated_bug = get_bug_by_id(pool, bug_id).await?
        .ok_or_else(|| sqlx::Error::RowNotFound)?;
    
    Ok(updated_bug)
}

pub async fn delete_bug(pool: &SqlitePool, bug_id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM bugs WHERE id = ?")
        .bind(bug_id)
        .execute(pool)
        .await?;
    
    Ok(result.rows_affected() > 0)
}

pub async fn assign_bug(pool: &SqlitePool, bug_id: i64, developer_id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE bugs SET developer_id = ? WHERE id = ?"
    )
    .bind(developer_id)
    .bind(bug_id)
    .execute(pool)
    .await?;
    
    Ok(result.rows_affected() > 0)
}

pub async fn get_all_developers(pool: &SqlitePool) -> Result<Vec<Developer>, sqlx::Error> {
    let developers = sqlx::query_as::<_, Developer>("SELECT * FROM developers ORDER BY name")
        .fetch_all(pool)
        .await?;
    Ok(developers)
}

pub async fn create_developer(pool: &SqlitePool, developer: &NewDeveloper) -> Result<Developer, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO developers (name) VALUES (?)"
    )
    .bind(&developer.name)
    .execute(pool)
    .await?;

    let developer_id = result.last_insert_rowid();
    
    let created_developer = sqlx::query_as::<_, Developer>(
        "SELECT * FROM developers WHERE id = ?"
    )
    .bind(developer_id)
    .fetch_one(pool)
    .await?;

    Ok(created_developer)
}

pub async fn get_user_by_username(pool: &SqlitePool, username: &str) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
        .bind(username)
        .fetch_optional(pool)
        .await?;
    Ok(user)
}