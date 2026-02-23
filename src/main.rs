mod cosmetics;
mod methods;
mod types;
mod user;

use std::{collections::HashMap, pin::Pin, sync::Arc};

use session_rs::server::SessionServer;
use tokio::sync::Mutex;

use crate::types::SessionMap;

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
                    let uuid = Arc::new(Mutex::new(String::new()));
                    let name = Arc::new(Mutex::new(String::new()));

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
                            let sessions = Arc::clone(&sessions);
                            let name = Arc::clone(&name);
                            let session = session.clone();

                            move |_, token| {
                                methods::auth::authenticate(
                                    sessions.clone(),
                                    session.clone(),
                                    name.clone(),
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
                        .on_request::<methods::Emote, _>({
                            let sessions = Arc::clone(&sessions);
                            let uuid = Arc::clone(&uuid);

                            move |_, emote| {
                                let sessions = Arc::clone(&sessions);
                                let uuid = Arc::clone(&uuid);

                                methods::emote::send_emote(sessions, uuid, emote)
                            }
                        })
                        .await;

                    session
                        .on_request::<methods::GetPlayer, _>({
                            let sessions = Arc::clone(&sessions);
                            let pool = Arc::clone(&pool);

                            move |_, uuid| {
                                methods::user::get_user(
                                    sessions.clone(),
                                    uuid.clone(),
                                    pool.clone(),
                                )
                            }
                        })
                        .await;

                    session
                        .on_request::<methods::SendPlayer, _>({
                            let sessions = Arc::clone(&sessions);
                            let pool = Arc::clone(&pool);
                            let uuid = Arc::clone(&uuid);
                            let name = Arc::clone(&name);

                            move |_, targets| {
                                methods::user::send_user(
                                    sessions.clone(),
                                    name.clone(),
                                    uuid.clone(),
                                    targets.targets.clone(),
                                    pool.clone(),
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
