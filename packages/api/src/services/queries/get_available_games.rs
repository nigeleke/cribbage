use chrono::{DateTime, Utc};
#[cfg(feature = "server")]
use dioxus::fullstack::extract::State;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use crate::ServerStateExtractor;
use crate::dto::{AvailableGameDTO, UserIdDTO};

/// Optional timestamp used for fetching only records created after a certain point in time.
///
/// This is used for pagination or incremental updates.
/// If `None`, the endpoint returns results from the beginning (or all available games).
pub type Since = Option<DateTime<Utc>>;

/// Response type for the `get_available_games` API endpoint.
///
/// Contains the list of available games for a user, along with pagination information.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AvailableGamesResponse {
    games: Vec<AvailableGameDTO>,
    has_more: bool,
    since: Since,
}

impl AvailableGamesResponse {
    /// Returns a slice of available games in this response.
    pub fn games(&self) -> &[AvailableGameDTO] {
        &self.games
    }

    /// Returns `true` if there are more available games beyond this response.
    pub fn has_more(&self) -> bool {
        self.has_more
    }

    /// Returns the `since` timestamp used for pagination, if any.
    pub fn since(&self) -> &Since {
        &self.since
    }
}

/// Retrieves a list of available games for a given user.
///
/// # Parameters
/// - `user_id`: The ID of the user requesting available games.
/// - `filter`: Optional string to filter games by name or criteria.
/// - `since`: Optional timestamp; only games created after this time are returned.
///
/// # Returns
/// On success, returns an [`AvailableGamesResponse`] containing:
/// - The list of available games (`games`) matching the filter and since parameters.
/// - Whether there are more games (`has_more`) beyond this response.
/// - The timestamp used for this query (`since`).
///
/// # Errors
/// Returns a [`ServerError`] if there is an issue fetching games from the server.
#[get("/api/{user_id}/available_games?filter&since", State(server_state): State<ServerStateExtractor>)]
pub async fn get_available_games(
    user_id: UserIdDTO,
    filter: Option<String>,
    since: Since,
) -> Result<AvailableGamesResponse> {
    use server::{
        domain::{Availability, UserId},
        queries::get_available_games,
    };

    use crate::dto::{AvailabilityDTO, GameIdDTO};

    let user_id = UserId::from(user_id.value());
    let filter = filter.unwrap_or_default();

    let (games, has_more, since) =
        get_available_games(server_state.0, user_id, filter, since).await?;

    let games = games
        .into_iter()
        .map(|game| {
            let game_id = GameIdDTO::from(game.id().value());
            let name = String::from(game.name());
            let availability = match game.availability() {
                Availability::Private => AvailabilityDTO::Private,
                Availability::Public => AvailabilityDTO::Public,
            };
            AvailableGameDTO {
                game_id,
                name,
                availability,
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
