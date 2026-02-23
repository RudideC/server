use sqlx::SqlitePool;

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

enum CosmeticKind {
    Hat,
    Cloak,
}

// Buy a cosmetic
pub async fn buy(
    kind: CosmeticKind,
    uuid: &str,
    id: &str,
    pool: &SqlitePool,
) -> Result<String, String> {
    let mut user = get_put(uuid, pool).await?;

    match kind {
        CosmeticKind::Hat => {
            if !HATS.contains(&id) {
                return Err("Hat does not exist".into());
            }
            if !user.hats.contains(&id.to_string()) {
                user.hats.push(id.to_string());
            }
        }
        CosmeticKind::Cloak => {
            if !CLOAKS.contains(&id) {
                return Err("Cloak does not exist".into());
            }
            if !user.cloaks.contains(&id.to_string()) {
                user.cloaks.push(id.to_string());
            }
        }
    }

    sqlx::query("UPDATE users SET cloaks = ?, hats = ? WHERE uuid = ?")
        .bind(serde_json::to_string(&user.cloaks).unwrap())
        .bind(serde_json::to_string(&user.hats).unwrap())
        .bind(uuid)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(format!("Item {} bought successfully!", id))
}

// Equip a cosmetic
pub async fn equip(
    kind: CosmeticKind,
    uuid: &str,
    id: &str,
    pool: &SqlitePool,
) -> Result<String, String> {
    let mut user = get_put(uuid, pool).await?;

    match kind {
        CosmeticKind::Hat => {
            if !user.hats.contains(&id.to_string()) {
                return Err("You don't own this hat".into());
            }
            user.hat = id.to_string();
        }
        CosmeticKind::Cloak => {
            if !user.cloaks.contains(&id.to_string()) {
                return Err("You don't own this cloak".into());
            }
            user.cloak = id.to_string();
        }
    }

    sqlx::query("UPDATE users SET cloak = ?, hat = ? WHERE uuid = ?")
        .bind(&user.cloak)
        .bind(&user.hat)
        .bind(uuid)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(format!("Item {} equipped successfully!", id))
}
