mod cosmetics;
mod methods;
mod user;

use std::{pin::Pin, sync::Arc};

use serde::{Deserialize, Serialize};
use session_rs::server::SessionServer;
use sqlx::SqlitePool;
use ureq::http::StatusCode;

use crate::user::User;

#[tokio::main(flavor = "current_thread")]
async fn main() -> session_rs::Result<()> {
    let pool = Arc::new(init_db().await);
    let server = SessionServer::bind("127.0.0.1:8080").await?;

    server
        .session_loop({
            let pool = Arc::clone(&pool);
            move |session, _| {
                let pool = Arc::clone(&pool);

                Box::pin(async move {
                    println!("Connected");

                    session
                        .on::<methods::Auth, _>(move |req_id, token| {
                            let req_id = req_id.clone();
                            let token = token.clone();

                            authenticate(req_id.clone(), token, Arc::clone(&pool))
                        })
                        .await;

                    Ok::<(), session_rs::Error>(())
                }) as Pin<Box<dyn Future<Output = _> + Send>>
            }
        })
        .await
}

#[derive(Debug, Deserialize, Serialize)]
struct MinecraftAuthResponse {
    pub id: String,
    pub name: String,
}

async fn authenticate(
    _req_id: u32,
    session_token: String,
    pool: Arc<SqlitePool>,
) -> Result<User, String> {
    let mut response = ureq::get("https://api.minecraftservices.com/minecraft/profile")
        .header("Authorization", &format!("Bearer {session_token}"))
        .call()
        .map_err(|_| "Failed to validate session".to_string())?;

    if response.status() != StatusCode::OK {
        return Err(format!(
            "Authentication failed with code {}",
            response.status()
        ));
    }

    let auth: MinecraftAuthResponse = response
        .body_mut()
        .read_json()
        .map_err(|_| "Unable to parse auth response".to_string())?;

    user::get_put(&auth.id, &pool).await
}

async fn init_db() -> SqlitePool {
    let pool = SqlitePool::connect("sqlite:cosmetics.db").await.unwrap();
    sqlx::query(include_str!("schema.sql"))
        .execute(&pool)
        .await
        .unwrap();
    pool
}
