use super::{Card as CardDto, DtoError, UserId};
#[cfg(feature = "server")]
use crate::database::UserGameRow;
#[cfg(feature = "server")]
use domain::{
    Card as DomainCard, HasCrib, HasCut, HasHands, HasScores, Play, PlayState, Player, State,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardState {
    Hidden,
    Revealed(CardDto),
    Placeholder,
}

#[cfg(feature = "server")]
fn map_cards(cards: &[DomainCard], card_as: fn(c: CardDto) -> CardState) -> Vec<CardState> {
    cards.iter().map(CardDto::from).map(card_as).collect()
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pegging {
    front: usize,
    back: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerState {
    hand: Vec<CardState>,
    score: Pegging,
}

impl PlayerState {
    #[cfg(feature = "server")]
    fn from<T: HasHands + HasScores>(
        value: &T,
        for_player: Player,
        card_as: fn(c: CardDto) -> CardState,
    ) -> Self {
        Self {
            hand: map_cards(value.hand(for_player).as_ref(), card_as),
            score: {
                let pegging = value.pegging(for_player);
                Pegging {
                    front: *pegging.front_peg().points(),
                    back: *pegging.back_peg().points(),
                }
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    User,
    Opponent,
}

impl Role {
    #[cfg(feature = "server")]
    pub fn from(user: &Player, player: &Player) -> Self {
        if player == user {
            Role::User
        } else {
            Role::Opponent
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Plays {
    current: Vec<(Role, CardDto)>,
    previous: Vec<(Role, CardDto)>,
}

#[cfg(feature = "server")]
fn map_plays(user: Player, plays: &[Play]) -> Vec<(Role, CardDto)> {
    plays
        .iter()
        .map(|play| {
            (
                Role::from(&user, &play.player()),
                CardDto::from(&play.card()),
            )
        })
        .collect()
}

impl Plays {
    #[cfg(feature = "server")]
    pub fn from(user: Player, play_state: &PlayState) -> Self {
        Self {
            current: map_plays(user, &play_state.current_plays()),
            previous: map_plays(user, &play_state.previous_plays()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameState {
    Starting {
        user_cut: CardDto,
        opponent_cut: CardDto,
        dealer: Option<Role>,
    },
    InProgress {
        user_state: PlayerState,
        opponent_state: PlayerState,
        crib: Vec<CardState>,
        cut: Option<CardDto>,
        plays: Option<Plays>,
        winner: Option<Role>,
    },
}

impl GameState {
    #[cfg(feature = "server")]
    pub fn try_from(game: UserGameRow, for_user: UserId) -> Result<Self, DtoError> {
        let user = domain::Player::from(*for_user.value());

        let state = serde_json::from_value::<State>(game.state.0)?;

        match state {
            State::Starting(game) => {
                let opponent = game.opponent(user)?;
                let user_cut = CardDto::from(&game.cut(user)?);
                let opponent_cut = CardDto::from(&game.cut(opponent)?);
                let dealer = game.draw()?.map(|p| Role::from(&user, &p.dealer()));

                Ok(GameState::Starting {
                    user_cut,
                    opponent_cut,
                    dealer,
                })
            }
            State::Discarding(game) => {
                let opponent = game.opponent(user)?;
                let user_state = PlayerState::from(&game, user, |c| CardState::Revealed(c));
                let opponent_state = PlayerState::from(&game, opponent, |_| CardState::Hidden);
                let crib = vec![if game.crib().is_empty() {
                    CardState::Placeholder
                } else {
                    CardState::Hidden
                }];
                let cut = None;
                let plays = None;
                let winner = None;
                Ok(GameState::InProgress {
                    user_state,
                    opponent_state,
                    crib,
                    cut,
                    plays,
                    winner,
                })
            }
            State::Playing(game) => {
                let opponent = game.opponent(user)?;
                let user_state = PlayerState::from(&game, user, |c| CardState::Revealed(c));
                let opponent_state = PlayerState::from(&game, opponent, |_| CardState::Hidden);
                let crib = vec![CardState::Hidden];
                let cut = Some(CardDto::from(&game.cut()));
                let plays = Some(Plays::from(user, game.play_state()));
                let winner = None;
                Ok(GameState::InProgress {
                    user_state,
                    opponent_state,
                    crib,
                    cut,
                    plays,
                    winner,
                })
            }
            State::ScoringPone(game) => {
                let opponent = game.opponent(user)?;
                let user_state = PlayerState::from(&game, user, |c| CardState::Revealed(c));
                let opponent_state = PlayerState::from(&game, opponent, |_| CardState::Hidden);
                let crib = vec![CardState::Hidden];
                let cut = Some(CardDto::from(&game.cut()));
                let plays = None;
                let winner = None;
                Ok(GameState::InProgress {
                    user_state,
                    opponent_state,
                    crib,
                    cut,
                    plays,
                    winner,
                })
            }
            State::ScoringDealer(game) => {
                let opponent = game.opponent(user)?;
                let user_state = PlayerState::from(&game, user, |c| CardState::Revealed(c));
                let opponent_state = PlayerState::from(&game, opponent, |c| CardState::Revealed(c));
                let crib = vec![CardState::Hidden];
                let cut = Some(CardDto::from(&game.cut()));
                let plays = None;
                let winner = None;
                Ok(GameState::InProgress {
                    user_state,
                    opponent_state,
                    crib,
                    cut,
                    plays,
                    winner,
                })
            }
            State::ScoringCrib(game) => {
                let opponent = game.opponent(user)?;
                let user_state = PlayerState::from(&game, user, |c| CardState::Revealed(c));
                let opponent_state = PlayerState::from(&game, opponent, |c| CardState::Revealed(c));
                let crib = map_cards(game.crib().as_ref(), |c| CardState::Revealed(c));
                let cut = Some(CardDto::from(&game.cut()));
                let plays = None;
                let winner = None;
                Ok(GameState::InProgress {
                    user_state,
                    opponent_state,
                    crib,
                    cut,
                    plays,
                    winner,
                })
            }
            State::Finished(game) => {
                let opponent = game.opponent(user)?;
                let user_state = PlayerState::from(&game, user, |c| CardState::Revealed(c));
                let opponent_state = PlayerState::from(&game, opponent, |c| CardState::Revealed(c));
                let crib = vec![if game.crib().is_empty() {
                    CardState::Placeholder
                } else {
                    CardState::Hidden
                }];
                let cut = Some(CardDto::from(&game.cut()));
                let plays = None;
                let winner = None;
                Ok(GameState::InProgress {
                    user_state,
                    opponent_state,
                    crib,
                    cut,
                    plays,
                    winner,
                })
            }
        }
    }
}
