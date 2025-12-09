#[cfg(feature = "server")]
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use crate::dto::ScoreDTO;
use crate::dto::{CardDTO, PeggingDTO, PendingDTO, PhaseDTO, PlayerDTO, PlayerStateDTO, PlaysDTO};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserGameDTO {
    pub name: String,
    pub phase: PhaseDTO,
    pub pending: PendingDTO,
    pub dealer: Option<PlayerDTO>,
    pub user_state: PlayerStateDTO,
    pub opponent_state: PlayerStateDTO,
    pub crib: Vec<CardDTO>,
    pub starter_cut: Option<CardDTO>,
    pub plays: Option<PlaysDTO>,
    pub pegging: PeggingDTO,
    pub winner: Option<PlayerDTO>,
}

#[cfg(feature = "server")]
impl UserGameDTO {
    fn new(name: &str, phase: PhaseDTO) -> Self {
        Self {
            name: String::from(name),
            phase,
            pending: PendingDTO::Nobody,
            dealer: None,
            user_state: PlayerStateDTO::default(),
            opponent_state: PlayerStateDTO::default(),
            crib: Vec::default(),
            starter_cut: None,
            plays: None,
            pegging: PeggingDTO::default(),
            winner: None,
        }
    }

    fn with_pending(mut self, pending: PendingDTO) -> Self {
        self.pending = pending;
        self
    }

    fn with_cuts_for_deal(
        mut self,
        user_cut: Option<CardDTO>,
        opponent_cut: Option<CardDTO>,
    ) -> Self {
        self.user_state.cut = user_cut;
        self.opponent_state.cut = opponent_cut;
        self
    }

    fn with_hands(mut self, user_hand: Vec<CardDTO>, opponent_hand: Vec<CardDTO>) -> Self {
        self.user_state.hand = user_hand;
        self.opponent_state.hand = opponent_hand;
        self
    }

    fn with_scores(mut self, user_score: ScoreDTO, opponent_score: ScoreDTO) -> Self {
        self.user_state.score = user_score;
        self.opponent_state.score = opponent_score;
        self
    }

    fn with_dealer(mut self, dealer: Option<PlayerDTO>) -> Self {
        self.dealer = dealer;
        self
    }

    fn with_crib_and_starter_cut(
        mut self,
        cards: Vec<CardDTO>,
        starter_cut: Option<CardDTO>,
    ) -> Self {
        self.crib = cards;
        self.starter_cut = starter_cut;
        self
    }

    fn with_plays(mut self, plays: PlaysDTO) -> Self {
        self.plays = Some(plays);
        self
    }

    fn with_pegging(mut self, score_sheet: PeggingDTO) -> Self {
        self.pegging = score_sheet;
        self
    }

    fn with_winner(mut self, winner: PlayerDTO) -> Self {
        self.winner = Some(winner);
        self
    }
}

#[cfg(feature = "server")]
mod server_only {
    use server::domain::{
        Finished, Game, HasCrib, HasCutsForDeal, HasHands, HasPegging, HasPending, HasPlayState,
        HasRoles, HasScoreboard, HasStarterCut, PLAYER0, PLAYER1, Play, Player, Roles, ScoreKind,
        State, UserId,
    };

    use super::*;
    use crate::dto::{pegging::PeggingKindDTO, plays::PlayActionDTO};

    fn players(game: &Game, user_id: UserId) -> (Player, Player) {
        let is_host = game.host() == &user_id;
        if is_host {
            (PLAYER0, PLAYER1)
        } else {
            (PLAYER1, PLAYER0)
        }
    }

    fn pending<T: HasPending>(s: &T, p: Player) -> PendingDTO {
        PendingDTO::new(p, s.pending())
    }

    fn cut_for_deal<T: HasCutsForDeal>(s: &T, p: Player) -> Option<CardDTO> {
        s.cut_for_deal(p).map(CardDTO::face_up)
    }

    fn starter_cut<T: HasStarterCut>(s: &T) -> Option<CardDTO> {
        Some(CardDTO::face_up(s.starter_cut()))
    }

    fn dealer_from_roles(
        roles: &Roles,
        player_dto_map: &HashMap<Player, PlayerDTO>,
    ) -> Option<PlayerDTO> {
        player_dto_map.get(&roles.dealer().player()).cloned()
    }

    fn dealer_from_maybe_roles(
        roles: Option<Roles>,
        player_dto_map: &HashMap<Player, PlayerDTO>,
    ) -> Option<PlayerDTO> {
        roles.and_then(|roles| dealer_from_roles(&roles, player_dto_map))
    }

    fn dealer<T: HasRoles>(
        s: &T,
        player_dto_map: &HashMap<Player, PlayerDTO>,
    ) -> Option<PlayerDTO> {
        dealer_from_roles(s.roles(), player_dto_map)
    }

    fn hand_down<T: HasHands>(s: &T, p: Player) -> Vec<CardDTO> {
        s.hand(p).as_ref().iter().map(CardDTO::face_down).collect()
    }

    fn hand_up<T: HasHands>(s: &T, p: Player) -> Vec<CardDTO> {
        s.hand(p).as_ref().iter().map(CardDTO::face_up).collect()
    }

    fn score<T: HasScoreboard>(s: &T, p: Player) -> ScoreDTO {
        ScoreDTO::from(s.positions(p))
    }

    fn crib_down<T: HasCrib>(s: &T) -> Vec<CardDTO> {
        s.crib().as_ref().iter().map(CardDTO::face_down).collect()
    }

    fn crib_up<T: HasCrib>(s: &T) -> Vec<CardDTO> {
        s.crib().as_ref().iter().map(CardDTO::face_up).collect()
    }

    fn plays<T: HasPlayState>(s: &T, player_dto_map: &HashMap<Player, PlayerDTO>) -> PlaysDTO {
        let play_state = s.play_state();

        let (legal_plays, next_action) = {
            if play_state.is_finished() {
                (vec![], PlayActionDTO::ScorePone)
            } else {
                let next_to_play = play_state.next_to_play();
                let next_to_play_dto = player_dto_map
                    .get(&next_to_play)
                    .expect("next player must be defined");

                let legal_plays = play_state.legal_plays(next_to_play);
                let legal_play_cids = legal_plays
                    .iter()
                    .map(|card| card.cid().clone())
                    .collect::<Vec<_>>();

                let can_play = !legal_plays.is_empty();
                if can_play {
                    (legal_play_cids, PlayActionDTO::Play(*next_to_play_dto))
                } else {
                    (vec![], PlayActionDTO::Go(*next_to_play_dto))
                }
            }
        };

        let plays_to_dto = |plays: &Vec<Play>| {
            plays
                .iter()
                .map(|play| {
                    (
                        *player_dto_map.get(&play.player()).expect("valid player"),
                        CardDTO::face_up(&play.card()),
                    )
                })
                .collect::<Vec<_>>()
        };

        let current = plays_to_dto(&play_state.current_plays());
        let previous = plays_to_dto(&play_state.previous_plays());
        let running_total = play_state.running_total().value() as u8;

        PlaysDTO {
            next_action,
            legal_plays,
            current,
            previous,
            running_total,
        }
    }

    fn pegging<T: HasPegging>(s: &T) -> PeggingDTO {
        let score_items = s.pegging().score_sheet().items();

        score_items
            .into_iter()
            .fold(PeggingDTO::default(), |mut acc, item| {
                let kind = match item.kind() {
                    ScoreKind::Fifteen => Some(PeggingKindDTO::Fifteens),
                    ScoreKind::Pair | ScoreKind::Triplet | ScoreKind::Quadruplet => {
                        Some(PeggingKindDTO::Pairs)
                    }
                    ScoreKind::Run => Some(PeggingKindDTO::Runs),
                    ScoreKind::Flush => Some(PeggingKindDTO::Flush),
                    ScoreKind::LastCard => None, // currently used for hand & crib scores
                    ScoreKind::ThirtyOne => None, // currently used for hand & crib scores
                    ScoreKind::HisHeels => None, // currently used for hand & crib scores
                    ScoreKind::Nobs => Some(PeggingKindDTO::Nob),
                };

                if let Some(kind) = kind {
                    let points = item.points().value();
                    let cids = item
                        .cards()
                        .into_iter()
                        .map(|c| c.cid())
                        .collect::<Vec<_>>();
                    let entry = acc.entry(kind).or_default();
                    entry.points += points;
                    entry.breakdown.push(cids);
                }

                acc
            })
    }

    fn winner(finished: &Finished, player_dto_map: &HashMap<Player, PlayerDTO>) -> PlayerDTO {
        *player_dto_map
            .get(&finished.winner())
            .expect("valid player")
    }

    impl From<(UserId, &Game)> for UserGameDTO {
        fn from((user_id, game): (UserId, &Game)) -> Self {
            let (me, them) = players(game, user_id);
            let player_dto_map: HashMap<Player, PlayerDTO> =
                [(me, PlayerDTO::User), (them, PlayerDTO::Opponent)]
                    .into_iter()
                    .collect();
            let name = game.name();

            match &game.state() {
                State::Starting(state) if game.guest().is_none() => {
                    Self::new(name, PhaseDTO::InLobby).with_pending(PendingDTO::Opponent)
                }

                State::Starting(state) => Self::new(name, PhaseDTO::CuttingForDeal)
                    .with_pending(pending(state, me))
                    .with_cuts_for_deal(cut_for_deal(state, me), cut_for_deal(state, them))
                    .with_dealer(dealer_from_maybe_roles(state.roles(), &player_dto_map)),

                State::Discarding(state) => UserGameDTO::new(name, PhaseDTO::Discarding)
                    .with_scores(score(state, me), score(state, them))
                    .with_dealer(dealer(state, &player_dto_map))
                    .with_pending(pending(state, me))
                    .with_hands(hand_up(state, me), hand_down(state, them))
                    .with_crib_and_starter_cut(crib_down(state), None),

                State::Playing(state) => Self::new(name, PhaseDTO::Playing)
                    .with_scores(score(state, me), score(state, them))
                    .with_dealer(dealer(state, &player_dto_map))
                    .with_pending(pending(state, me))
                    .with_hands(hand_up(state, me), hand_down(state, them))
                    .with_crib_and_starter_cut(crib_down(state), starter_cut(state))
                    .with_plays(plays(state, &player_dto_map)),

                State::ScoringPone(state) => {
                    let they_are_pone = state.pone().player() == them;
                    UserGameDTO::new(name, PhaseDTO::ScoringPone)
                        .with_scores(score(state, me), score(state, them))
                        .with_dealer(dealer(state, &player_dto_map))
                        .with_pending(pending(state, me))
                        .with_hands(
                            hand_up(state, me),
                            if they_are_pone {
                                hand_up(state, them)
                            } else {
                                hand_down(state, them)
                            },
                        )
                        .with_crib_and_starter_cut(crib_down(state), starter_cut(state))
                        .with_pegging(pegging(state))
                }

                State::ScoringDealer(state) => UserGameDTO::new(name, PhaseDTO::ScoringDealer)
                    .with_scores(score(state, me), score(state, them))
                    .with_dealer(dealer(state, &player_dto_map))
                    .with_pending(pending(state, me))
                    .with_hands(hand_up(state, me), hand_up(state, them))
                    .with_crib_and_starter_cut(crib_down(state), starter_cut(state))
                    .with_pegging(pegging(state)),

                State::ScoringCrib(state) => UserGameDTO::new(name, PhaseDTO::ScoringCrib)
                    .with_scores(score(state, me), score(state, them))
                    .with_dealer(dealer(state, &player_dto_map))
                    .with_pending(pending(state, me))
                    .with_hands(hand_up(state, me), hand_up(state, them))
                    .with_crib_and_starter_cut(crib_up(state), starter_cut(state))
                    .with_pegging(pegging(state)),

                State::Finished(state) => UserGameDTO::new(name, PhaseDTO::Finished)
                    .with_scores(score(state, me), score(state, them))
                    .with_dealer(dealer(state, &player_dto_map))
                    .with_hands(hand_up(state, me), hand_up(state, them))
                    .with_crib_and_starter_cut(crib_up(state), starter_cut(state))
                    .with_winner(winner(state, &player_dto_map)),
            }
        }
    }
}
