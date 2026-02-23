use serde::{Deserialize, Serialize};
use session_rs::Method;

use crate::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth;

impl Method for Auth {
    const NAME: &'static str = "auth";
    type Request = String;
    type Response = User;
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
    const NAME: &'static str = "set_hat";
    type Request = String;
    type Response = String;
    type Error = String;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuyCloak;

impl Method for BuyCloak {
    const NAME: &'static str = "buy_cloak";
    type Request = String;
    type Response = String;
    type Error = String;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuyHat;

impl Method for BuyHat {
    const NAME: &'static str = "buy_hat";
    type Request = String;
    type Response = String;
    type Error = String;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientEmote {
    pub emote: String,
    pub targets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEmote {
    pub emote: String,
    pub from: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Emote;

impl Method for Emote {
    const NAME: &'static str = "emote";
    type Request = ClientEmote;
    type Response = String;
    type Error = String;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmoteEvent;

impl Method for EmoteEvent {
    const NAME: &'static str = "emote_event";
    type Request = EventEmote;
    type Response = String;
    type Error = String;
}
