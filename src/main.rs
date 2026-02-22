use session_rs::server::SessionServer;

use crate::methods::AuthResponse;

pub mod methods;

#[tokio::main(flavor = "current_thread")]
async fn main() -> session_rs::Result<()> {
    let server = SessionServer::bind("127.0.0.1:8080").await?;

    server
        .session_loop(async |session, _| {
            println!("Connected");
            session
                .on::<methods::Auth, _>(|_, req| async {
                    Ok(AuthResponse {
                        cloak: String::new(),
                        hat: String::new(),
                        cloaks: Vec::new(),
                        hats: Vec::new(),
                    })
                })
                .await;
            Ok(())
        })
        .await
}
