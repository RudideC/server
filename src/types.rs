use serde::{Deserialize, Serialize};
use session_rs::session::Session;
use tokio::sync::Mutex;

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use crate::user::User;

pub type SessionMap = Arc<Mutex<HashMap<String, HashSet<Session>>>>;
pub type UUID = Arc<Mutex<String>>;

#[derive(Debug, Deserialize, Serialize)]
pub struct MinecraftAuthResponse {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
/// Received from the client to send to the targets
pub struct EmoteRequest {
    pub emote: String,
    pub targets: Vec<String>,
}

/// To send to the targets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEmote {
    pub emote: String,
    pub from: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStream {
    pub player: User,
    pub uuid: String,
    pub name: String,
}

pub fn format_uuid(input: &str) -> Result<String, &'static str> {
    if input.len() != 32 {
        return Err("Input must be exactly 32 hex characters");
    }

    Ok(format!(
        "{}-{}-{}-{}-{}",
        &input[0..8],
        &input[8..12],
        &input[12..16],
        &input[16..20],
        &input[20..32],
    ))
}
