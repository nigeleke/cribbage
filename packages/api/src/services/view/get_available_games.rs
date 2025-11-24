use crate::dto::{AvailableGameDTO, UserIdDTO};
use chrono::{DateTime, Utc};
use dioxus::fullstack::extract::State;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

pub type Since = Option<DateTime<Utc>>;

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AvailableGamesResponse {
    games: Vec<AvailableGameDTO>,
    has_more: bool,
    since: Since,
}

impl AvailableGamesResponse {
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

#[get("/api/{user_id}/available_games?filter&since", State(server_state): State<server::ServerState>)]
pub async fn get_available_games(
    user_id: UserIdDTO,
    filter: Option<String>,
    since: Since,
) -> Result<AvailableGamesResponse> {
    use crate::dto::{AvailabilityDTO, GameIdDTO};
    use server::domain::{Availability, UserId};
    use server::view::get_available_games;

    let user_id = UserId::from(user_id.value());
    let filter = filter.unwrap_or_default();

    let (games, has_more, since) =
        get_available_games(server_state, user_id, filter, since).await?;

    let games = games
        .into_iter()
        .map(|game| {
            let game_id = GameIdDTO::from(game.id().value());
            let name = game.name().clone();
            let availability = match game.availability() {
                Availability::Private => AvailabilityDTO::Private,
                Availability::Public => AvailabilityDTO::Public,
            };
            let created_at = game.created_at().clone();
            AvailableGameDTO {
                game_id,
                name,
                availability,
                created_at,
            }
        })
        .collect::<Vec<_>>();

    let response = AvailableGamesResponse {
        games,
        has_more,
        since,
    };

    Ok(response)
}
