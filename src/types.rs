use serde::{Deserialize, Serialize};
use session_rs::session::Session;
use tokio::sync::Mutex;

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

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
