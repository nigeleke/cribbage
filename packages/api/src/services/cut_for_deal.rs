use dioxus::prelude::*;
use dto::{CutForDealStateDTO, GameIdDTO, UserIdDTO};

#[post("/api/{user_id}/game/{game_id}/cut_for_deal")]
pub async fn cut_for_deal(
    user_id: UserIdDTO,
    game_id: GameIdDTO,
) -> Result<CutForDealStateDTO, ServerFnError> {
    use crate::services::convertors;

    let user_id = backend::UserId::from(user_id.value());
    let game_id = backend::GameId::from(game_id.value());

    let state = backend::cut_for_deal(user_id, game_id)
        .await
        .map(|state| convertors::cut_for_deal_state_to_dto(&state))
        .map_err(ServerFnError::new)?;

    Ok(state)
}
