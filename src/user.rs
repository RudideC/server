use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub cloak: String,
    pub hat: String,
    pub cloaks: Vec<String>,
    pub hats: Vec<String>,
}

// Initialize SQLite connection pool
pub async fn init_db() -> SqlitePool {
    let pool = SqlitePool::connect("sqlite://users.db?mode=rwc")
        .await
        .expect("Failed to open database");

    sqlx::query(include_str!("schema.sql"))
        .execute(&pool)
        .await
        .expect("Failed to run schema");

    pool
}

// Get user by UUID
pub async fn get(uuid: &str, pool: &SqlitePool) -> Result<User, String> {
    let row = sqlx::query("SELECT cloak, hat, cloaks, hats FROM users WHERE uuid = ?")
        .bind(uuid)
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;

    if let Some(r) = row {
        let user = User {
            cloak: r.get::<String, _>("cloak"),
            hat: r.get::<String, _>("hat"),
            cloaks: serde_json::from_str(&r.get::<String, _>("cloaks")).unwrap_or_default(),
            hats: serde_json::from_str(&r.get::<String, _>("hats")).unwrap_or_default(),
        };
        Ok(user)
    } else {
        Err("User not found".into())
    }
}

// Create user if not exists
pub async fn get_put(uuid: &str, pool: &SqlitePool) -> Result<User, String> {
    match get(uuid, pool).await {
        Ok(user) => Ok(user),
        Err(_) => {
            let user = User {
                cloak: "".into(),
                hat: "".into(),
                cloaks: vec![],
                hats: vec![],
            };
            sqlx::query(
                "INSERT INTO users (uuid, cloak, hat, cloaks, hats) VALUES (?, ?, ?, ?, ?)",
            )
            .bind(uuid)
            .bind(&user.cloak)
            .bind(&user.hat)
            .bind(serde_json::to_string(&user.cloaks).unwrap())
            .bind(serde_json::to_string(&user.hats).unwrap())
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
            Ok(user)
        }
    }
}
