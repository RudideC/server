use sqlx::SqlitePool;
use std::sync::Arc;

use crate::{types::UUID, user};

pub const CLOAKS: &[&str] = &[
    "glitch",
    "mercedes_flow",
    "crimson_mark",
    "bmw",
    "amg",
    "amg_petronas",
    "ferrari",
    "redbull",
    "black_hole_amethyst",
    "black_hole_flame",
    "black_hole_white",
    "albania_mark",
    "end",
    "spanish_empire",
    "spain_flag"
];

pub const HATS: &[&str] = &[
    "horns_black",
    "horns_white",
    "halo_white",
    "halo_black",
    "horns_end",
    "halo_end",
    "bucket_black",
    "bucket_end"
];

pub enum CosmeticKind {
    Hat,
    Cloak,
}

// Buy a cosmetic
pub async fn buy(
    kind: CosmeticKind,
    uuid: UUID,
    item_id: String,
    pool: Arc<SqlitePool>,
) -> Result<String, String> {
    // Lock once
    let uuid = uuid.lock().await.clone();

    let mut user = user::get(&uuid, &pool).await?;

    match kind {
        CosmeticKind::Hat => {
            if !HATS.contains(&item_id.as_str()) {
                return Err("Hat does not exist".into());
            }

            if !user.hats.contains(&item_id) {
                user.hats.push(item_id.clone());
            }
        }
        CosmeticKind::Cloak => {
            if !CLOAKS.contains(&item_id.as_str()) {
                return Err("Cloak does not exist".into());
            }

            if !user.cloaks.contains(&item_id) {
                user.cloaks.push(item_id.clone());
            }
        }
    }

    sqlx::query("UPDATE users SET cloaks = ?, hats = ? WHERE uuid = ?")
        .bind(serde_json::to_string(&user.cloaks).map_err(|e| e.to_string())?)
        .bind(serde_json::to_string(&user.hats).map_err(|e| e.to_string())?)
        .bind(&uuid)
        .execute(pool.as_ref())
        .await
        .map_err(|e| e.to_string())?;

    Ok(format!("Item {} bought successfully!", item_id))
}

// Equip a cosmetic
pub async fn equip(
    kind: CosmeticKind,
    uuid: UUID,
    item_id: String,
    pool: Arc<SqlitePool>,
) -> Result<String, String> {
    // Lock once
    let uuid = uuid.lock().await.clone();

    let mut user = user::get(&uuid, &pool).await?;

    match kind {
        CosmeticKind::Hat => {
            if !user.hats.contains(&item_id) {
                return Err("You don't own this hat".into());
            }

            user.hat = item_id.clone();
        }
        CosmeticKind::Cloak => {
            if !user.cloaks.contains(&item_id) {
                return Err("You don't own this cloak".into());
            }

            user.cloak = item_id.clone();
        }
    }

    sqlx::query("UPDATE users SET cloak = ?, hat = ? WHERE uuid = ?")
        .bind(&user.cloak)
        .bind(&user.hat)
        .bind(&uuid)
        .execute(pool.as_ref())
        .await
        .map_err(|e| e.to_string())?;

    Ok(format!("Item {} equipped successfully!", item_id))
}
