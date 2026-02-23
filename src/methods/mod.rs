pub mod auth;
pub mod emote;
pub mod user;

use serde::{Deserialize, Serialize};
use session_rs::Method;

use crate::{
    types::{EmoteRequest, EventEmote, PlayerStream},
    user::User,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Auth;

impl Method for Auth {
    const NAME: &'static str = "auth";
    type Request = String;
    type Response = User;
    type Error = String;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCloak;

impl Method for SetCloak {
    const NAME: &'static str = "set_cloak";
    type Request = String;
    type Response = String;
    type Error = String;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetHat;

impl Method for SetHat {
    const NAME: &'static str = "set_hat";
    type Request = String;
    type Response = String;
    type Error = String;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuyCloak;

impl Method for BuyCloak {
    const NAME: &'static str = "buy_cloak";
    type Request = String;
    type Response = String;
    type Error = String;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuyHat;

impl Method for BuyHat {
    const NAME: &'static str = "buy_hat";
    type Request = String;
    type Response = String;
    type Error = String;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Emote;

impl Method for Emote {
    const NAME: &'static str = "emote";
    type Request = EmoteRequest;
    type Response = ();
    type Error = String;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmoteEvent;

impl Method for EmoteEvent {
    const NAME: &'static str = "emote_event";
    type Request = EventEmote;
    type Response = ();
    type Error = ();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPlayer;

impl Method for GetPlayer {
    const NAME: &'static str = "get_player";
    type Request = String;
    type Response = Option<User>;
    type Error = String;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player;

impl Method for Player {
    const NAME: &'static str = "player";
    type Request = PlayerStream;
    type Response = ();
    type Error = ();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendPlayer;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendPlayerRequest {
    pub targets: Vec<String>,
}

impl Method for SendPlayer {
    const NAME: &'static str = "send_player";
    type Request = SendPlayerRequest;
    type Response = ();
    type Error = String;
}
