mod cosmetics;
mod methods;
mod user;

use std::{
    collections::{HashMap, HashSet},
    pin::Pin,
    sync::Arc,
};

use serde::{Deserialize, Serialize};
use session_rs::{server::SessionServer, session::Session};
use sqlx::SqlitePool;
use tokio::sync::Mutex;

use crate::user::User;

type SessionMap = Arc<Mutex<HashMap<String, HashSet<Session>>>>;

#[tokio::main(flavor = "current_thread")]
async fn main() -> session_rs::Result<()> {
    let pool = Arc::new(user::init_db().await);
    let server = SessionServer::bind("127.0.0.1:8080").await?;

    let sessions: SessionMap = Arc::new(Mutex::new(HashMap::new()));

    server
        .session_loop({
            let pool = Arc::clone(&pool);
            move |session, _| {
                let pool = Arc::clone(&pool);
                let sessions = Arc::clone(&sessions);

                Box::pin(async move {
                    println!("Connected");
                    let uuid: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));

                    session
                        .on_close({
                            let session = session.clone();
                            let uuid = uuid.clone();
                            let sessions = Arc::clone(&sessions);

                            move || {
                                let uuid = uuid.clone();
                                let sessions = Arc::clone(&sessions);
                                let session = session.clone();

                                Box::pin(async move {
                                    let uuid_lock = uuid.lock().await;
                                    if !uuid_lock.is_empty() {
                                        if let Some(sessions) =
                                            sessions.lock().await.get_mut(uuid_lock.as_str())
                                        {
                                            sessions.remove(&session);
                                        }
                                    }
                                    Ok(())
                                })
                            }
                        })
                        .await;

                    session.start_ping(
                        tokio::time::Duration::from_secs(30),
                        tokio::time::Duration::from_secs(5),
                    );

                    session
                        .on_request::<methods::Auth, _>({
                            let pool = Arc::clone(&pool);
                            let uuid = Arc::clone(&uuid);
                            let session = session.clone();
                            let sessions = sessions.clone();

                            move |_, token| {
                                authenticate(
                                    sessions.clone(),
                                    session.clone(),
                                    uuid.clone(),
                                    token,
                                    pool.clone(),
                                )
                            }
                        })
                        .await;

                    session
                        .on_request::<methods::BuyCloak, _>({
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
                        .on_request::<methods::BuyHat, _>({
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
                        .on_request::<methods::SetCloak, _>({
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
                        .on_request::<methods::SetHat, _>({
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

                    session
                        .on_request::<methods::SetHat, _>({
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

                    session
                        .on_request::<methods::Emote, _>({
                            let sessions = Arc::clone(&sessions);

                            move |_, emote| {
                                let sessions = Arc::clone(&sessions);
                                let uuid = Arc::clone(&uuid);

                                send_emote(sessions, uuid, emote)
                            }
                        })
                        .await;

                    Ok::<(), session_rs::Error>(())
                }) as Pin<Box<dyn Future<Output = _> + Send>>
            }
        })
        .await
}

async fn send_emote(
    sessions: SessionMap,
    uuid: Arc<Mutex<String>>,
    emote: methods::ClientEmote,
) -> Result<String, String> {
    let emote_event = methods::EventEmote {
        from: uuid.lock().await.clone(),
        emote: emote.emote,
    };

    for i in emote.targets {
        if let Some(sessions) = sessions.lock().await.get_mut(&i) {
            let emote_event = emote_event.clone();
            sessions.retain(|s| {
                let notify_result = tokio::runtime::Handle::current()
                    .block_on(s.notify::<methods::EmoteEvent>(emote_event.clone()));

                notify_result.is_ok()
            });
        }
    }

    Ok(format!("Cool"))
}

#[derive(Debug, Deserialize, Serialize)]
struct MinecraftAuthResponse {
    pub id: String,
    pub name: String,
}

async fn authenticate(
    sessions: SessionMap,
    session: Session,
    uuid: Arc<Mutex<String>>,
    session_token: String,
    pool: Arc<SqlitePool>,
) -> Result<User, String> {
    if !uuid.lock().await.is_empty() {
        return Err(format!("Already authenticated"));
    }

    let client = reqwest::Client::new();

    let response = client
        .get("https://api.minecraftservices.com/minecraft/profile")
        .bearer_auth(&session_token)
        .send()
        .await
        .map_err(|_| "Failed to validate session".to_string())?;

    if !response.status().is_success() {
        return Err(format!(
            "Authentication failed with code {}",
            response.status()
        ));
    }

    let auth: MinecraftAuthResponse = response
        .json()
        .await
        .map_err(|_| "Unable to parse auth response".to_string())?;

    *uuid.lock().await = auth.id.clone();

    sessions
        .lock()
        .await
        .entry(auth.id.clone())
        .or_default()
        .insert(session);

    println!("Authenticated as {:?}", auth);

    user::get_put(&auth.id, &pool).await
}
