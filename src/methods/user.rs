use std::sync::Arc;

use sqlx::SqlitePool;

use crate::{
    methods,
    types::{PlayerStream, SessionMap, UUID},
    user::User,
};

pub async fn get_user(
    sessions: SessionMap,
    uuid: String,
    pool: Arc<SqlitePool>,
) -> Result<Option<User>, String> {
    if !sessions.lock().await.contains_key(&uuid) {
        return Ok(None);
    }

    Ok(Some(crate::user::get(&uuid, pool.as_ref()).await?))
}

pub async fn send_user(
    sessions: SessionMap,
    name: UUID,
    uuid: UUID,
    targets: Vec<String>,
    pool: Arc<SqlitePool>,
) -> Result<(), String> {
    println!("send player {targets:?}");
    let uuid = uuid.lock().await.to_string();

    let user = PlayerStream {
        player: crate::user::get(&uuid, pool.as_ref()).await?,
        uuid: uuid.clone(),
        name: name.lock().await.clone(),
    };

    println!("ready {targets:?}");

    for target in targets {
        if let Some(sessions) = sessions.lock().await.get_mut(&target) {
            let mut bad_sessions = Vec::new();

            for s in sessions.iter() {
                if s.notify::<methods::Player>(user.clone()).await.is_err() {
                    bad_sessions.push(s.clone());
                }
            }

            for s in bad_sessions {
                sessions.remove(&s);
            }
        }
    }

    println!("sent");

    Ok(())
}
