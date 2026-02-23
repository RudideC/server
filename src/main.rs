mod cosmetics;
mod methods;
mod user;

use std::{pin::Pin, sync::Arc};

use serde::{Deserialize, Serialize};
use session_rs::server::SessionServer;
use sqlx::SqlitePool;
use tokio::sync::Mutex;
use ureq::http::StatusCode;

use crate::user::User;

#[tokio::main(flavor = "current_thread")]
async fn main() -> session_rs::Result<()> {
    let pool = Arc::new(user::init_db().await);
    let server = SessionServer::bind("127.0.0.1:8080").await?;

    server
        .session_loop({
            let pool = Arc::clone(&pool);
            move |session, _| {
                let pool = Arc::clone(&pool);

                Box::pin(async move {
                    println!("Connected");
                    let uuid: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));

                    session
                        .on::<methods::Auth, _>({
                            let pool = Arc::clone(&pool);
                            let uuid = Arc::clone(&uuid);

                            move |_, token| authenticate(uuid.clone(), token, pool.clone())
                        })
                        .await;

                    session
                        .on::<methods::BuyCloak, _>({
                            let uuid = Arc::clone(&uuid);
                            let pool = Arc::clone(&pool);

                            move |_, item_id| {
                                let pool = Arc::clone(&pool);
                                let item_id = item_id.clone();

                                cosmetics::buy(
                                    cosmetics::CosmeticKind::Cloak,
                                    uuid.clone(),
                                    item_id,
                                    pool,
                                )
                            }
                        })
                        .await;

                    session
                        .on::<methods::BuyHat, _>({
                            let uuid = Arc::clone(&uuid);
                            let pool = Arc::clone(&pool);

                            move |_, item_id| {
                                let pool = Arc::clone(&pool);
                                let item_id = item_id.clone();

                                cosmetics::buy(
                                    cosmetics::CosmeticKind::Hat,
                                    uuid.clone(),
                                    item_id,
                                    pool,
                                )
                            }
                        })
                        .await;

                    session
                        .on::<methods::SetCloak, _>({
                            let uuid = Arc::clone(&uuid);
                            let pool = Arc::clone(&pool);

                            move |_, item_id| {
                                let pool = Arc::clone(&pool);
                                let item_id = item_id.clone();

                                cosmetics::equip(
                                    cosmetics::CosmeticKind::Cloak,
                                    uuid.clone(),
                                    item_id,
                                    pool,
                                )
                            }
                        })
                        .await;

                    session
                        .on::<methods::SetHat, _>({
                            let uuid = Arc::clone(&uuid);
                            let pool = Arc::clone(&pool);

                            move |_, item_id| {
                                let pool = Arc::clone(&pool);
                                let item_id = item_id.clone();

                                cosmetics::equip(
                                    cosmetics::CosmeticKind::Hat,
                                    uuid.clone(),
                                    item_id,
                                    pool,
                                )
                            }
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
    uuid: Arc<Mutex<String>>,
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

    *uuid.lock().await = auth.id.clone();

    user::get_put(&auth.id, &pool).await
}
