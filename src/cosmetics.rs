use std::sync::Arc;

use sqlx::SqlitePool;
use tokio::sync::Mutex;

use crate::user::get_put;

pub const CLOAKS: &[&str] = &[
    "mercedes_flow",
    "glitch",
    "crimson_mark",
    "bmw",
    "amg",
    "amg_petronas",
    "ferrari",
    "redbull",
];

pub const HATS: &[&str] = &["horns_black", "horns_white", "halo", "halo_black"];

pub enum CosmeticKind {
    Hat,
    Cloak,
}

// Buy a cosmetic
pub async fn buy(
    kind: CosmeticKind,
    uuid: Arc<Mutex<String>>,
    item_id: String,
    pool: Arc<SqlitePool>,
) -> Result<String, String> {
    let mut user = get_put(uuid.lock().await.as_str(), &pool).await?;

    match kind {
        CosmeticKind::Hat => {
            if !HATS.contains(&item_id.as_str()) {
                return Err("Hat does not exist".into());
            }
            if !user.hats.contains(&item_id.to_string()) {
                user.hats.push(item_id.to_string());
            }
        }
        CosmeticKind::Cloak => {
            if !CLOAKS.contains(&item_id.as_str()) {
                return Err("Cloak does not exist".into());
            }
            if !user.cloaks.contains(&item_id.to_string()) {
                user.cloaks.push(item_id.to_string());
            }
        }
    }

    sqlx::query("UPDATE users SET cloaks = ?, hats = ? WHERE uuid = ?")
        .bind(serde_json::to_string(&user.cloaks).unwrap())
        .bind(serde_json::to_string(&user.hats).unwrap())
        .bind(uuid.lock().await.as_str())
        .execute(pool.as_ref())
        .await
        .map_err(|e| e.to_string())?;

    Ok(format!("Item {} bought successfully!", item_id))
}

// Equip a cosmetic
pub async fn equip(
    kind: CosmeticKind,
    uuid: Arc<Mutex<String>>,
    item_id: String,
    pool: Arc<SqlitePool>,
) -> Result<String, String> {
    let mut user = get_put(uuid.lock().await.as_str(), &pool).await?;

    match kind {
        CosmeticKind::Hat => {
            if !user.hats.contains(&item_id) {
                return Err("You don't own this hat".into());
            }
            user.hat = item_id.to_string();
        }
        CosmeticKind::Cloak => {
            if !user.cloaks.contains(&item_id) {
                return Err("You don't own this cloak".into());
            }
            user.cloak = item_id.to_string();
        }
    }

    sqlx::query("UPDATE users SET cloak = ?, hat = ? WHERE uuid = ?")
        .bind(&user.cloak)
        .bind(&user.hat)
        .bind(uuid.lock().await.as_str())
        .execute(pool.as_ref())
        .await
        .map_err(|e| e.to_string())?;

    Ok(format!("Item {} equipped successfully!", item_id))
}
