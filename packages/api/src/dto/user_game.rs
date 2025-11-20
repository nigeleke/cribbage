use crate::{
    CardDTO, CribDTO, PendingDTO, PhaseDTO, PlayerDTO, PlayerStateDTO, PlaysDTO, ScoreDTO,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserGameDTO {
    pub name: String,
    pub phase: PhaseDTO,
    pub pending: PendingDTO,
    pub dealer: Option<PlayerDTO>,
    pub user_state: PlayerStateDTO,
    pub opponent_state: PlayerStateDTO,
    pub crib: CribDTO,
    pub plays: Option<PlaysDTO>,
    pub winner: Option<PlayerDTO>,
}

impl UserGameDTO {
    fn new(name: &str, phase: PhaseDTO) -> Self {
        Self {
            name: String::from(name),
            phase,
            pending: PendingDTO::Nobody,
            dealer: None,
            user_state: PlayerStateDTO::default(),
            opponent_state: PlayerStateDTO::default(),
            crib: CribDTO::default(),
            plays: None,
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
        self.crib = CribDTO { starter_cut, cards };
        self
    }

    fn with_plays(mut self, plays: PlaysDTO) -> Self {
        self.plays = Some(plays);
        self
    }
}

#[cfg(feature = "server")]
mod server_only {
    use crate::dto::plays::PlayActionDTO;

    use super::*;
    use server::domain::{
        Game, HasCrib, HasCutsForDeal, HasHands, HasPending, HasPlayState, HasRoles, HasScoreboard,
        HasStarterCut, PLAYER0, PLAYER1, Play, Player, Roles, State, UserId,
    };

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
        roles
            .map(|roles| dealer_from_roles(&roles, player_dto_map))
            .flatten()
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
        ScoreDTO::from(s.pegging(p))
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
            if play_state.all_cards_are_played() {
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
                    (vec![], PlayActionDTO::Pass(*next_to_play_dto))
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

                State::Playing(state) => {
                    dioxus::prelude::debug!("dto:user_game: mapping {state}");
                    Self::new(name, PhaseDTO::Playing)
                        .with_scores(score(state, me), score(state, them))
                        .with_dealer(dealer(state, &player_dto_map))
                        .with_hands(hand_up(state, me), hand_down(state, them))
                        .with_crib_and_starter_cut(crib_down(state), starter_cut(state))
                        .with_plays(plays(state, &player_dto_map))
                }

                State::ScoringPone(state) => UserGameDTO::new(name, PhaseDTO::ScoringPone)
                    .with_scores(score(state, me), score(state, them))
                    .with_dealer(dealer(state, &player_dto_map))
                    .with_pending(pending(state, me))
                    .with_hands(hand_up(state, me), hand_down(state, them))
                    .with_crib_and_starter_cut(crib_down(state), starter_cut(state)),

                State::ScoringDealer(state) => UserGameDTO::new(name, PhaseDTO::ScoringDealer)
                    .with_scores(score(state, me), score(state, them))
                    .with_dealer(dealer(state, &player_dto_map))
                    .with_pending(pending(state, me))
                    .with_hands(hand_up(state, me), hand_up(state, them))
                    .with_crib_and_starter_cut(crib_down(state), starter_cut(state)),

                State::ScoringCrib(state) => UserGameDTO::new(name, PhaseDTO::ScoringCrib)
                    .with_scores(score(state, me), score(state, them))
                    .with_dealer(dealer(state, &player_dto_map))
                    .with_pending(pending(state, me))
                    .with_hands(hand_up(state, me), hand_up(state, them))
                    .with_crib_and_starter_cut(crib_up(state), starter_cut(state)),

                State::Finished(state) => UserGameDTO::new(name, PhaseDTO::Finished)
                    .with_scores(score(state, me), score(state, them))
                    .with_dealer(dealer(state, &player_dto_map))
                    .with_hands(hand_up(state, me), hand_up(state, them))
                    .with_crib_and_starter_cut(crib_up(state), starter_cut(state)),
            }
        }
    }
}
