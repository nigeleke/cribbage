use dioxus::prelude::*;

use crate::dto::{ActiveGameId, UserId};

#[server]
pub async fn new_computer_game(_user_id: UserId) -> Result<ActiveGameId, ServerFnError> {
    unimplemented!()
}
