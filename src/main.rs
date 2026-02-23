mod cosmetics;
mod methods;
mod user;

use serde::{Deserialize, Serialize};
use session_rs::server::SessionServer;
use ureq::http::StatusCode;

use crate::user::User;

#[tokio::main(flavor = "current_thread")]
async fn main() -> session_rs::Result<()> {
    let server = SessionServer::bind("127.0.0.1:8080").await?;

    server
        .session_loop(async |session, _| {
            println!("Connected");
            session.on::<methods::Auth, _>(authenticate).await;

            Ok(())
        })
        .await
}

#[derive(Debug, Deserialize, Serialize)]
struct MinecraftAuthResponse {
    pub id: String,
    pub name: String,
}

async fn authenticate(_req_id: u32, session_token: String) -> Result<User, String> {
    let mut response = ureq::get("https://api.minecraftservices.com/minecraft/profile")
        .header("Authorization", &format!("Bearer {session_token}"))
        .call()
        .map_err(|_| format!("Failed to validate session"))?;

    if response.status() != StatusCode::OK {
        return Err(format!(
            "Authentication failed with code {}",
            response.status()
        ));
    }

    let auth: MinecraftAuthResponse = response
        .body_mut()
        .read_json()
        .map_err(|_| format!("Unable to parse auth response"))?;

    user::get_put(auth.id).await
}
