use std::sync::Arc;

use session_rs::session::Session;
use sqlx::SqlitePool;

use crate::{
    types::{SessionMap, UUID},
    user::User,
};

pub async fn authenticate(
    sessions: SessionMap,
    session: Session,
    name: UUID,
    uuid: UUID,
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

    let auth: crate::types::MinecraftAuthResponse = response
        .json()
        .await
        .map_err(|_| "Unable to parse auth response".to_string())?;

    *uuid.lock().await = auth.id.clone();
    *name.lock().await = auth.name.clone();

    sessions
        .lock()
        .await
        .entry(auth.id.clone())
        .or_default()
        .insert(session);

    println!("Authenticated as {:?}", auth);

    crate::user::get_put(&auth.id, &pool).await
}
