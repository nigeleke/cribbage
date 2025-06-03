use crate::{ActiveGameId, UserId};
use std::borrow::Borrow;
use std:fmt::Display;

#[inline]
pub fn app_channel() -> String {
    String::from("cribbage")
}

#[inline]
pub fn user_channel<U: Borrow<UserId>>(user_id: U) -> String
{
    format!("cribbage::user-{}", user_id.borrow())
}

#[inline]
pub fn game_channel<G: Borrow<ActiveGameId>>(game_id: G) -> String {
    format!("cribbage::game-{}", game_id.borrow())
}

#[inline]
pub fn user_game_channel<U, G>(user_id: U, game_id: G) -> String
where
    U: Borrow<UserId>,
    G: Borrow<ActiveGameId>,
{
    format!("cribbage::user-{}::game-{}", user_id.borrow(), game_id.borrow())
}
