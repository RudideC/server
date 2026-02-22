use serde::{Deserialize, Serialize};
use session_rs::Method;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub cloak: String,
    pub hat: String,
    pub cloaks: Vec<String>,
    pub hats: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth;

impl Method for Auth {
    const NAME: &'static str = "auth";
    type Request = String;
    type Response = AuthResponse;
    type Error = String;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetCloak;

impl Method for SetCloak {
    const NAME: &'static str = "set_cloak";
    type Request = String;
    type Response = String;
    type Error = String;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetHat;

impl Method for SetHat {
    const NAME: &'static str = "set_cloak";
    type Request = String;
    type Response = String;
    type Error = String;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuyCloak;

impl Method for BuyCloak {
    const NAME: &'static str = "set_cloak";
    type Request = String;
    type Response = String;
    type Error = String;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuyHat;

impl Method for BuyHat {
    const NAME: &'static str = "set_cloak";
    type Request = String;
    type Response = String;
    type Error = String;
}
