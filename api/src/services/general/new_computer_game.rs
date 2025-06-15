use crate::dto::{GameId, UserId};
use dioxus::prelude::*;

#[server]
pub async fn new_computer_game(_user_id: UserId) -> Result<GameId, ServerFnError> {
    unimplemented!()
}
