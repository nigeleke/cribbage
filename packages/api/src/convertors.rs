use crate::dto::{CardDTO, PlayerDTO, ScoreDTO, UserGameDTO};
use server::{Card, Game, PLAYER0, PLAYER1, Pegging, Player, State};

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

pub fn game_to_user_game_dto(game: &Game, user_id: &server::UserId) -> UserGameDTO {
    let user_is_host = || game.host() == user_id;
    let user_is_guest = || game.guest() == Some(user_id);

    let view_for_user_opponent = |user: Player, opponent: Player| match game.state() {
        State::Starting(_) if game.guest().is_none() => UserGameDTO::new(game.name()),
        State::Starting(state) => {
            let user_cut = state.cut(user).map(face_up);
            let opponent_cut = state.cut(opponent).map(face_up);
            let dealer = state
                .roles()
                .map(|r| player_as_dto(user, r.dealer().player()));
            UserGameDTO::new(game.name())
                .with_user_cut(user_cut, dealer)
                .with_opponent_cut(opponent_cut)
        }
        State::Discarding(state) => {
            let dealer = player_as_dto(user, state.dealer().player());
            let crib = cards_as_dto(state.crib().as_ref(), face_down);
            let user_score = pegging_as_score(state.scoreboard().pegging(user));
            let opponent_score = pegging_as_score(state.scoreboard().pegging(opponent));
            let user_hand = cards_as_dto(state.hand(user).as_ref(), face_up);
            let opponent_hand = cards_as_dto(state.hand(opponent).as_ref(), face_down);

            UserGameDTO::new(game.name())
                .with_dealer_and_crib(dealer, &crib)
                .with_user_state(user_score, &user_hand)
                .with_opponent_state(opponent_score, &opponent_hand)
        }
        State::Playing(_state) => UserGameDTO::new("TODO: Playing UserGameDTO"),
        State::ScoringPone(_state) => UserGameDTO::new("TODO: ScoringPone UserGameDTO"),
        State::ScoringDealer(_state) => UserGameDTO::new("TODO: ScoringDealer UserGameDTO"),
        State::ScoringCrib(_state) => UserGameDTO::new("TODO: ScoringCrib UserGameDTO"),
        State::Finished(_state) => UserGameDTO::new("TODO: Finished UserGameDTO"),
    };

    if user_is_host() {
        view_for_user_opponent(PLAYER0, PLAYER1)
    } else if user_is_guest() {
        view_for_user_opponent(PLAYER1, PLAYER0)
    } else {
        UserGameDTO::new(game.name())
    }
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
