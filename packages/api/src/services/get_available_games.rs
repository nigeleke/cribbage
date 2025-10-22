use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use dto::{AvailableGameDTO, UserIdDTO};
use serde::{Deserialize, Serialize};

pub type Since = Option<DateTime<Utc>>;

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Response {
    games: Vec<AvailableGameDTO>,
    has_more: bool,
    since: Since,
}

impl Response {
    pub fn games(&self) -> &[AvailableGameDTO] {
        &self.games
    }

    pub fn has_more(&self) -> bool {
        self.has_more
    }

    pub fn since(&self) -> &Since {
        &self.since
    }
}

#[get("/api/{user_id}/available_games?filter&since")]
pub async fn get_available_games(
    user_id: UserIdDTO,
    filter: Option<String>,
    since: Since,
) -> Result<Response, ServerFnError> {
    use dto::GameIdDTO;

    let user_id = backend::UserId::from(user_id.value());
    let filter = filter.unwrap_or_default();

    let (games, has_more, since) = backend::get_available_games(user_id, filter, since)
        .await
        .map_err(ServerFnError::new)?;

    let games = games
        .into_iter()
        .map(|game| {
            let game_id = GameIdDTO::from(game.0.value());
            let source = game.1;
            let name = game.2;
            match source {
                backend::AvailableGameSource::Lobby => AvailableGameDTO::Lobby { game_id, name },
                backend::AvailableGameSource::Active => AvailableGameDTO::Active { game_id, name },
            }
        })
        .collect::<Vec<_>>();

    let response = Response {
        games,
        has_more,
        since,
    };

    Ok(response)
}
