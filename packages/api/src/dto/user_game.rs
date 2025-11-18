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
}

#[cfg(feature = "server")]
mod server_only {
    use super::*;
    use server::domain::{
        Game, HasCrib, HasCutsForDeal, HasHands, HasPending, HasRoles, HasScoreboard,
        HasStarterCut, PLAYER0, PLAYER1, Player, Roles, State, UserId,
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
        player_dto_map: HashMap<Player, PlayerDTO>,
    ) -> Option<PlayerDTO> {
        player_dto_map.get(&roles.dealer().player()).cloned()
    }

    fn dealer_from_maybe_roles(
        roles: Option<Roles>,
        player_dto_map: HashMap<Player, PlayerDTO>,
    ) -> Option<PlayerDTO> {
        roles
            .map(|roles| dealer_from_roles(&roles, player_dto_map))
            .flatten()
    }

    fn dealer<T: HasRoles>(s: &T, player_dto_map: HashMap<Player, PlayerDTO>) -> Option<PlayerDTO> {
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

    impl From<(UserId, &Game)> for UserGameDTO {
        fn from((user_id, game): (UserId, &Game)) -> Self {
            match &game.state() {
                State::Starting(state) => {
                    if let Some(_) = game.guest() {
                        let (me, them) = players(game, user_id);
                        Self::new(game.name(), PhaseDTO::CuttingForDeal)
                            .with_pending(pending(state, me))
                            .with_cuts_for_deal(cut_for_deal(state, me), cut_for_deal(state, them))
                            .with_dealer(dealer_from_maybe_roles(
                                state.roles(),
                                player_dto_map(me, them),
                            ))
                    } else {
                        Self::new(game.name(), PhaseDTO::InLobby).with_pending(PendingDTO::Opponent)
                    }
                }
                State::Discarding(state) => {
                    let (me, them) = players(game, user_id);
                    dto_base(game.name(), PhaseDTO::Discarding, state, me, them)
                        .with_pending(pending(state, me))
                        .with_hands(hand_up(state, me), hand_down(state, them))
                        .with_crib_and_starter_cut(crib_down(state), None)
                }
                State::Playing(state) => {
                    let (me, them) = players(game, user_id);
                    dto_base(game.name(), PhaseDTO::Playing, state, me, them)
                        .with_hands(hand_up(state, me), hand_down(state, them))
                        .with_crib_and_starter_cut(crib_down(state), starter_cut(state))
                }
                State::ScoringPone(state) => {
                    let (me, them) = players(game, user_id);
                    dto_base(game.name(), PhaseDTO::ScoringPone, state, me, them)
                        .with_pending(pending(state, me))
                        .with_hands(hand_up(state, me), hand_down(state, them))
                        .with_crib_and_starter_cut(crib_down(state), starter_cut(state))
                }
                State::ScoringDealer(state) => {
                    let (me, them) = players(game, user_id);
                    dto_base(game.name(), PhaseDTO::ScoringDealer, state, me, them)
                        .with_pending(pending(state, me))
                        .with_hands(hand_up(state, me), hand_up(state, them))
                        .with_crib_and_starter_cut(crib_down(state), starter_cut(state))
                }
                State::ScoringCrib(state) => {
                    let (me, them) = players(game, user_id);
                    dto_base(game.name(), PhaseDTO::ScoringCrib, state, me, them)
                        .with_pending(pending(state, me))
                        .with_hands(hand_up(state, me), hand_up(state, them))
                        .with_crib_and_starter_cut(crib_up(state), starter_cut(state))
                }
                State::Finished(state) => {
                    let (me, them) = players(game, user_id);
                    dto_base(game.name(), PhaseDTO::Finished, state, me, them)
                        .with_hands(hand_up(state, me), hand_up(state, them))
                        .with_crib_and_starter_cut(crib_up(state), starter_cut(state))
                }
            }
        }
    }

    #[inline]
    fn player_dto_map(me: Player, them: Player) -> HashMap<Player, PlayerDTO> {
        HashMap::from([(me, PlayerDTO::User), (them, PlayerDTO::Opponent)])
    }

    fn dto_base<S>(name: &str, phase: PhaseDTO, state: &S, me: Player, them: Player) -> UserGameDTO
    where
        S: HasRoles + HasScoreboard,
    {
        UserGameDTO::new(name, phase)
            .with_scores(score(state, me), score(state, them))
            .with_dealer(dealer(state, player_dto_map(me, them)))
    }
}
