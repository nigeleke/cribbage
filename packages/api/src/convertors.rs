use crate::dto::{CardDTO, PlayerDTO, ScoreDTO, UserGameDTO};
use server::domain::{Card, Game, PLAYER0, PLAYER1, Pegging, Player};

#[inline]
fn face_up(card: &Card) -> CardDTO {
    CardDTO::FaceUp { cid: card.cid() }
}

#[inline]
fn face_down(_card: &Card) -> CardDTO {
    CardDTO::FaceDown
}

#[inline]
fn player_as_dto(current: Player, required: Player) -> PlayerDTO {
    if current == required {
        PlayerDTO::User
    } else {
        PlayerDTO::Opponent
    }
}

#[inline]
fn cards_as_dto(cards: &[Card], f: impl Fn(&Card) -> CardDTO) -> Vec<CardDTO> {
    Vec::from_iter(cards.iter().map(f))
}

fn pegging_as_score(pegging: &Pegging) -> ScoreDTO {
    let back_peg = pegging.back_peg().value();
    let front_peg = pegging.front_peg().value();
    ScoreDTO {
        back_peg,
        front_peg,
    }
}

#[inline]
pub fn game_to_user_game_dto(game: &Game, user_id: &server::domain::UserId) -> UserGameDTO {
    UserGameDTO::from((*user_id, game))
}

// pub fn available_game_to_dto(game: &AvailableGame) -> AvailableGameDTO {
//     let game_id = GameIdDTO::from(game.id().value());
//     let name = game.name().clone();
//     let source = game.source();
//     match source {
//         AvailableGameSource::Lobby => AvailableGameDTO::Lobby { game_id, name },
//         AvailableGameSource::Active => AvailableGameDTO::Active { game_id, name },
//     }
// }

// pub fn cut_for_deal_state_to_dto(state: &CutForDealState) -> CutForDealStateDTO {
//     match state {
//         backend::CutForDealState::Pending => CutForDealStateDTO::Pending,
//         backend::CutForDealState::RedrawRequired => CutForDealStateDTO::RedrawRequired,
//         backend::CutForDealState::DealerSelected => CutForDealStateDTO::DealerSelected,
//     }
// }
