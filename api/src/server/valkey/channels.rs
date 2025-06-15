use crate::{GameId, UserId};
use std::borrow::Borrow;

#[inline]
pub fn app_channel() -> String {
    String::from("cribbage")
}

#[inline]
pub fn user_channel<U: Borrow<UserId>>(user_id: U) -> String {
    format!("cribbage::user-{}", user_id.borrow())
}

#[inline]
pub fn game_channel<G: Borrow<GameId>>(game_id: G) -> String {
    format!("cribbage::game-{}", game_id.borrow())
}

#[inline]
pub fn game_start_state_key<G: Borrow<GameId>>(game_id: G) -> String {
    format!("cribbage::game-start-{}", game_id.borrow())
}

#[inline]
pub fn game_redraw_state_key<G: Borrow<GameId>>(game_id: G) -> String {
    format!("cribbage::game-redraw-{}", game_id.borrow())
}

#[inline]
pub fn user_game_channel<U, G>(user_id: U, game_id: G) -> String
where
    U: Borrow<UserId>,
    G: Borrow<GameId>,
{
    format!(
        "cribbage::user-{}::game-{}",
        user_id.borrow(),
        game_id.borrow()
    )
}
