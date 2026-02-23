use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub cloak: String,
    pub hat: String,
    pub cloaks: Vec<String>,
    pub hats: Vec<String>,
}

/// Get a user
pub async fn get(uuid: String) -> Result<User, String> {}

/// Create a user if it doesn't exist
pub async fn get_put(uuid: String) -> Result<User, String> {}
