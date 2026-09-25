mod cutting;
mod dealing;
mod discarding;
mod finished;
mod playing;
mod scoring;
mod starting;

#[cfg(test)]
#[coverage(off)]
mod tests;

// ------------------------------------
use crate::{
    Card, Crib, CutsForDeal, Deck, Discard, Discards, Hands, PlayState, Player, Roles, Scoreboard,
};

/// Represents a game current state.
#[derive(PartialEq, Eq)]
pub struct Game<T>
where
    T: std::fmt::Debug + std::cmp::PartialEq + std::cmp::Eq,
{
    scoreboard: Scoreboard,
    state: T,
}

#[derive(PartialEq, Eq)]
struct Starting {
    deck: Deck,
    cuts: CutsForDeal,
}

#[derive(PartialEq, Eq)]
struct Dealing {
    roles: Roles,
}

#[derive(PartialEq, Eq)]
struct Discarding {
    roles: Roles,
    hands: Hands,
    deck: Deck,
    discards: Discards,
}

#[derive(PartialEq, Eq)]
struct Cutting {
    roles: Roles,
    hands: Hands,
    crib: Crib,
    deck: Deck,
}

#[derive(PartialEq, Eq)]
struct Playing {
    roles: Roles,
    hands: Hands,
    crib: Crib,
    starter: Card,
    play_state: PlayState,
}

#[derive(PartialEq, Eq)]
struct Scoring<T> {
    roles: Roles,
    hands: Hands,
    crib: Crib,
    starter: Card,
    _marker: std::marker::PhantomData<T>,
}

#[derive(Debug, PartialEq, Eq)]
struct ScorePone;

#[derive(Debug, PartialEq, Eq)]
struct ScoreDealer;

#[derive(Debug, PartialEq, Eq)]
struct ScoreCrib;

type ScoringPone = Scoring<ScorePone>;
type ScoringDealer = Scoring<ScoreDealer>;
type ScoringCrib = Scoring<ScoreCrib>;

#[derive(PartialEq, Eq)]
struct Finished {
    roles: Roles,
    hands: Hands,
    crib: Crib,
    starter: Card,
    play_state: Option<PlayState>,
}

#[derive(Debug)]
enum GameError {
    PlayerAlreadyCut,
    CardNotInDeck,
    PlayerAlreadyDiscarded,
    CardsNotInHand,
    PlayOutOfTurn,
    InvalidPlay,
}

type Result<T> = std::result::Result<T, GameError>;

#[derive(Debug)]
enum CutForDealOutcome {
    Starting(Game<Starting>),
    Dealing(Game<Dealing>),
}

impl Game<Starting> {
    /// Records a player's cut for the deal.
    ///
    /// The cut card must be in the deck and the player must not have
    /// already cut. Once both players have cut, the game transitions
    /// to [`Game<Dealing>`].
    pub fn cut_for_deal(self, player: Player, card: Card) -> Result<CutForDealOutcome> {
        starting::cut_for_deal(self, player, card)
    }
}

#[derive(Debug)]
enum DealOutcome {
    Discarding(Game<Discarding>),
}

impl Game<Dealing> {
    /// Deals a hand to each player using the supplied deck.
    ///
    /// On success, the game transitions to [`Game<Discarding>`].
    pub fn deal(self, deck: Deck) -> Result<DealOutcome> {
        dealing::deal(self, deck)
    }
}

#[derive(Debug)]
enum DiscardOutcome {
    Discarding(Game<Discarding>),
    Cutting(Game<Cutting>),
}

impl Game<Discarding> {
    /// Discards two cards from the specified player's hand to the crib.
    ///
    /// On success, the game remains in the discarding state until both
    /// players have discarded their cards.
    pub fn discard(self, player: Player, discard: Discard) -> Result<DiscardOutcome> {
        discarding::discard(self, player, discard)
    }
}

#[derive(Debug)]
enum CutStarterOutcome {
    Playing(Game<Playing>),
    Finished(Game<Finished>),
}

impl Game<Cutting> {
    /// Cuts the starter card to begin the play.
    ///
    /// On success, the game transitions from the cutting state to the playing
    /// state.
    pub fn cut_starter(self) -> Result<CutStarterOutcome> {
        cutting::cut_starter(self)
    }
}

#[derive(Debug)]
pub enum PlayOutcome {
    Playing(Game<Playing>),
    Scoring(Game<ScoringPone>),
    Finished(Game<Finished>),
}

#[derive(Debug)]
pub enum GoOutcome {
    Playing(Game<Playing>),
    Scoring(Game<ScoringPone>),
    Finished(Game<Finished>),
}

impl Game<Playing> {
    /// Plays a card for the specified player.
    ///
    /// On success, the game either remains in the playing state or
    /// transitions to scoring or finished when the plays end or the
    /// play ends the game.
    pub fn play(self, player: Player, card: Card) -> Result<PlayOutcome> {
        playing::play(self, player, card)
    }

    /// Declares go for the specified player.
    ///
    /// On success, the game either remains in the playing state or
    /// transitions to scoring or finished when the play ends the game.
    pub fn go(self, player: Player) -> Result<GoOutcome> {
        todo!()
    }
}

#[derive(Debug)]
enum ScorePoneOutcome {
    Scoring(Game<ScoringDealer>),
    Finished(Game<Finished>),
}

impl Game<Scoring<ScorePone>> {
    /// Scores the pone's hand.
    ///
    /// On success, the game transitions to scoring the dealer's hand,
    /// unless the game has been won and transitions to the finished state.
    pub fn score_pone(self) -> Result<ScorePoneOutcome> {
        todo!()
    }
}

#[derive(Debug)]
enum ScoreDealerOutcome {
    Scoring(Game<ScoringCrib>),
    Finished(Game<Finished>),
}

impl Game<Scoring<ScoreDealer>> {
    /// Scores the dealer's hand.
    ///
    /// On success, the game transitions to scoring the crib,
    /// unless the game has been won and transitions to the finished state.
    pub fn score_dealer(self) -> Result<ScoreDealerOutcome> {
        todo!()
    }
}

#[derive(Debug)]
enum ScoreCribOutcome {
    Dealing(Game<Dealing>),
    Finished(Game<Finished>),
}

impl Game<Scoring<ScoreCrib>> {
    /// Scores the crib.
    ///
    /// On success, the game transitions to dealing the next round,
    /// unless the game has been won and transitions to the finished state.
    pub fn score_crib(self) -> Result<ScoreCribOutcome> {
        todo!()
    }
}

impl Default for Game<Starting> {
    fn default() -> Self {
        Self {
            scoreboard: Default::default(),
            state: Default::default(),
        }
    }
}

impl<T> Game<T>
where
    T: std::fmt::Debug + std::cmp::PartialEq + std::cmp::Eq,
{
    fn new(scoreboard: Scoreboard, state: T) -> Self {
        Game { scoreboard, state }
    }

    fn transition<U>(self, f: impl FnOnce(T) -> U) -> Game<U>
    where
        U: std::fmt::Debug + std::cmp::PartialEq + std::cmp::Eq,
    {
        let Game { scoreboard, state } = self;
        Game::new(scoreboard, f(state))
    }
}

impl<T> std::fmt::Debug for Game<T>
where
    T: std::fmt::Debug + std::cmp::PartialEq + std::cmp::Eq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            r#"game({:?}
  {:?})"#,
            self.state, self.scoreboard
        )
    }
}

// impl Game {
//     /// Creates a new `Game` with the specified ID, host, optional guest, name, and state.
//     pub fn new(id: GameId, host: UserId, guest: Option<UserId>, name: &str, state: Phase) -> Self {
//         let name = String::from(name);
//         Self {
//             id,
//             host,
//             guest,
//             name,
//             phase: state,
//         }
//     }

//     /// Returns the unique identifier of the game.
//     pub fn id(&self) -> GameId {
//         self.id
//     }

//     /// Returns the user ID of the host.
//     pub fn host(&self) -> UserId {
//         self.host
//     }

//     /// Returns the user ID of the guest, if any.
//     pub fn guest(&self) -> Option<UserId> {
//         self.guest
//     }

//     /// Returns the name of the game.
//     pub fn name(&self) -> &str {
//         &self.name
//     }

//     #[cfg(test)]
//     pub(crate) fn name_mut(&mut self) -> &mut String {
//         &mut self.name
//     }

//     /// Returns the current phase of the game.
//     pub fn phase(&self) -> &Phase {
//         &self.phase
//     }

//     #[cfg(test)]
//     pub(crate) fn phase_mut(&mut self) -> &mut Phase {
//         &mut self.phase
//     }

//     /// Returns the player corresponding to the given user ID, if they are part of this game.
//     pub fn validate_user(&self, user_id: UserId) -> Option<Player> {
//         match user_id {
//             id if id == self.host => Some(PLAYER0),
//             id if Some(id) == self.guest => Some(PLAYER1),
//             _ => None,
//         }
//     }
// }

// impl Game {
//     fn host_game(&self, host: UserId, game_id: GameId) -> Result<Vec<GameEvent>, DomainError> {
//         let not_permitted = || Err(DomainError::NotPermitted(String::from("host game")));

//         if self.id != GameId::default() {
//             not_permitted()
//         } else {
//             let name = generate_game_name();
//             let events = vec![GameEvent::LobbyGameCreated {
//                 game_id,
//                 host,
//                 name,
//             }];
//             Ok(events)
//         }
//     }

//     fn join_game(&self, guest: UserId) -> Result<Vec<GameEvent>, DomainError> {
//         let not_permitted = || Err(DomainError::NotPermitted(String::from("join game")));

//         if self.id == GameId::default() || self.guest.is_some() {
//             not_permitted()
//         } else if self.host == guest {
//             Err(DomainError::InvalidOpponent)
//         } else {
//             let events = vec![GameEvent::LobbyGameJoined { guest }];
//             Ok(events)
//         }
//     }

//     fn play_computer(&self, host: UserId, game_id: GameId) -> Result<Vec<GameEvent>, DomainError> {
//         let not_permitted = || Err(DomainError::NotPermitted(String::from("play computer")));

//         if self.id != GameId::default() {
//             not_permitted()
//         } else {
//             let guest = UserId::new();
//             let name = generate_game_name();
//             let events = vec![GameEvent::ComputerGameCreated {
//                 game_id,
//                 host,
//                 guest,
//                 name,
//             }];
//             Ok(events)
//         }
//     }

//     fn cut_for_deal(&self, player: Player) -> Result<Vec<GameEvent>, DomainError> {
//         let not_permitted = || Err(DomainError::NotPermitted(String::from("cut for deal")));

//         let cut_for_deal = |starting: &Starting| {
//             if !starting.pending().waiting_on(player) {
//                 not_permitted()
//             } else {
//                 let mut deck = starting.deck().clone();
//                 let cut = deck.cut();
//                 let events = vec![GameEvent::CutForDealMade { player, cut }];
//                 Ok(events)
//             }
//         };

//         if self.id == GameId::default() {
//             not_permitted()
//         } else {
//             match &self.phase {
//                 Phase::Starting(starting) => {
//                     let events = cut_for_deal(starting)?;
//                     Ok(events)
//                 }
//                 _ => not_permitted(),
//             }
//         }
//     }

//     fn start_game(&self, player: Player) -> Result<Vec<GameEvent>, DomainError> {
//         let not_permitted = || Err(DomainError::NotPermitted(String::from("start game")));

//         let start_game = |starting: &Starting| {
//             let mut events = vec![GameEvent::GameStarted { player }];

//             let proceed = starting.pending().clone().acknowledge(player);
//             if proceed {
//                 if let Some(roles) = Roles::from_cuts(starting.cuts_for_deal()) {
//                     let mut deck = Deck::shuffled_pack();
//                     let hands = deck.deal(PLAYER_COUNT);

//                     events.append(&mut vec![
//                         GameEvent::CutForDealDecided {
//                             dealer: *roles.dealer(),
//                         },
//                         GameEvent::HandDealt {
//                             player: PLAYER0,
//                             hand: hands[PLAYER0].clone(),
//                         },
//                         GameEvent::HandDealt {
//                             player: PLAYER1,
//                             hand: hands[PLAYER1].clone(),
//                         },
//                     ]);
//                 } else {
//                     events.push(GameEvent::CutForDealTied);
//                 }
//             }

//             events
//         };

//         if self.id == GameId::default() {
//             not_permitted()
//         } else {
//             match &self.phase {
//                 Phase::Starting(starting) => {
//                     let events = start_game(starting);
//                     Ok(events)
//                 }
//                 _ => not_permitted(),
//             }
//         }
//     }

//     fn discard_cards(
//         &self,
//         player: Player,
//         cards: Vec<Card>,
//     ) -> Result<Vec<GameEvent>, DomainError> {
//         let not_permitted = || Err(DomainError::NotPermitted("discard cards".into()));

//         let discard_cards_to_crib = |discarding: &Discarding| {
//             if !discarding.pending().waiting_on(player) {
//                 not_permitted()
//             } else if !discarding.hand(player).contains_all(&cards)
//                 || cards.len() != CARDS_DISCARDED_TO_CRIB
//                 || discarding.hand(player).len() - cards.len() != CARDS_KEPT_PER_HAND
//             {
//                 Err(DomainError::InvalidDiscards(format_vec(&cards)))
//             } else {
//                 let mut events = vec![GameEvent::CardsDiscarded {
//                     player,
//                     cards: cards.clone(),
//                 }];

//                 let proceed = discarding.pending().clone().acknowledge(player);
//                 if proceed {
//                     let cut = discarding.deck().clone().cut();
//                     let dealer = discarding.dealer().player();
//                     let pegging = Pegging::new(dealer, ScoreSheet::his_heels(cut));
//                     events.push(GameEvent::StarterSelected { cut, pegging });
//                 }

//                 Ok(events)
//             }
//         };

//         if self.id == GameId::default() {
//             not_permitted()
//         } else {
//             match &self.phase {
//                 Phase::Discarding(discarding) => {
//                     let events = discard_cards_to_crib(discarding)?;
//                     Ok(events)
//                 }
//                 _ => not_permitted(),
//             }
//         }
//     }

//     fn play_card(&self, player: Player, card: Card) -> Result<Vec<GameEvent>, DomainError> {
//         let not_permitted = || Err(DomainError::NotPermitted("play card".into()));

//         let play_card = |playing: &Playing| {
//             let play_state = playing.play_state();
//             if play_state.next_to_play() != player {
//                 Err(DomainError::NotPlayersTurn(player))
//             } else if !play_state.legal_plays(player).contains(&card) {
//                 Err(DomainError::InvalidPlay(card))
//             } else {
//                 let mut play_state = play_state.clone();
//                 let score_sheet = play_state.play(card);
//                 let pegging = Pegging::new(player, score_sheet);
//                 Ok(vec![GameEvent::CardPlayed {
//                     player,
//                     card,
//                     pegging,
//                 }])
//             }
//         };

//         if self.id == GameId::default() {
//             not_permitted()
//         } else {
//             match &self.phase {
//                 Phase::Playing(playing) => {
//                     let events = play_card(playing)?;
//                     Ok(events)
//                 }
//                 _ => not_permitted(),
//             }
//         }
//     }

//     fn go(&self, player: Player) -> Result<Vec<GameEvent>, DomainError> {
//         let not_permitted = || Err(DomainError::NotPermitted("go".into()));

//         let go = |playing: &Playing| {
//             let play_state = playing.play_state();
//             if play_state.next_to_play() != player {
//                 Err(DomainError::NotPlayersTurn(player))
//             } else if !play_state.legal_plays(player).is_empty() {
//                 Err(DomainError::InvalidGo)
//             } else {
//                 // There will always be a valid play before a go can occur. The `or` condition
//                 // in `map_or` will never occur.
//                 let recipient = play_state
//                     .current_plays()
//                     .last()
//                     .map_or(player, Play::player);

//                 let mut play_state = play_state.clone();
//                 let score_sheet = play_state.go();
//                 let pegging = Pegging::new(recipient, score_sheet);

//                 Ok(vec![GameEvent::GoCalled { player, pegging }])
//             }
//         };

//         if self.id == GameId::default() {
//             not_permitted()
//         } else {
//             match &self.phase {
//                 Phase::Playing(playing) => {
//                     let events = go(playing)?;
//                     Ok(events)
//                 }
//                 _ => not_permitted(),
//             }
//         }
//     }

//     fn score_pone(&self, player: Player) -> Result<Vec<GameEvent>, DomainError> {
//         let not_permitted = || Err(DomainError::NotPermitted(String::from("score pone")));

//         let score_pone = |playing: &Playing| {
//             let pone = playing.pone().player();
//             let hands = playing.play_state().clone().finish_plays();
//             let hand = &hands[pone];
//             let cut = playing.starter_cut();

//             let pegging = Pegging::new(pone, ScoreSheet::hand(hand, *cut));
//             vec![GameEvent::PoneScored { player, pegging }]
//         };

//         if self.id == GameId::default() {
//             not_permitted()
//         } else {
//             match &self.phase {
//                 Phase::Playing(playing) => {
//                     let events = score_pone(playing);
//                     Ok(events)
//                 }
//                 _ => not_permitted(),
//             }
//         }
//     }

//     fn score_dealer(&self, player: Player) -> Result<Vec<GameEvent>, DomainError> {
//         let not_permitted = || Err(DomainError::NotPermitted(String::from("score dealer")));

//         let score_dealer = |scoring: &ScoringPone| {
//             let dealer = scoring.dealer().player();
//             let hand = scoring.hand(dealer);
//             let cut = scoring.starter_cut();

//             let pegging = Pegging::new(dealer, ScoreSheet::hand(hand, *cut));
//             vec![GameEvent::DealerScored { player, pegging }]
//         };

//         if self.id == GameId::default() {
//             not_permitted()
//         } else {
//             match &self.phase {
//                 Phase::ScoringPone(scoring) => {
//                     let events = score_dealer(scoring);
//                     Ok(events)
//                 }
//                 _ => not_permitted(),
//             }
//         }
//     }

//     fn score_crib(&self, player: Player) -> Result<Vec<GameEvent>, DomainError> {
//         let not_permitted = || Err(DomainError::NotPermitted(String::from("score crib")));

//         let score_crib = |scoring: &ScoringDealer| {
//             let dealer = scoring.dealer().player();
//             let crib = scoring.crib();
//             let cut = scoring.starter_cut();

//             let pegging = Pegging::new(dealer, ScoreSheet::crib(crib, *cut));
//             vec![GameEvent::CribScored { player, pegging }]
//         };

//         if self.id == GameId::default() {
//             not_permitted()
//         } else {
//             match &self.phase {
//                 Phase::ScoringDealer(scoring) => {
//                     let events = score_crib(scoring);
//                     Ok(events)
//                 }
//                 _ => not_permitted(),
//             }
//         }
//     }

//     fn start_next_round(&self, player: Player) -> Result<Vec<GameEvent>, DomainError> {
//         let not_permitted = || Err(DomainError::NotPermitted(String::from("start next round")));

//         let start_next_round = |scoring: &ScoringCrib| {
//             let mut events = vec![GameEvent::NextRoundStarted { player }];

//             let proceed = scoring.pending().clone().acknowledge(player);
//             if proceed {
//                 let mut deck = Deck::shuffled_pack();
//                 let hands = deck.deal(PLAYER_COUNT);
//                 events.append(&mut vec![
//                     GameEvent::HandDealt {
//                         player: PLAYER0,
//                         hand: hands[PLAYER0].clone(),
//                     },
//                     GameEvent::HandDealt {
//                         player: PLAYER1,
//                         hand: hands[PLAYER1].clone(),
//                     },
//                 ]);
//             };

//             events
//         };

//         if self.id == GameId::default() {
//             not_permitted()
//         } else {
//             match &self.phase {
//                 Phase::ScoringCrib(scoring) => {
//                     let events = start_next_round(scoring);
//                     Ok(events)
//                 }
//                 _ => not_permitted(),
//             }
//         }
//     }

//     pub(crate) fn handle_command(
//         &self,
//         command: GameCommand,
//     ) -> Result<Vec<GameEvent>, DomainError> {
//         tracing::debug!("COMMAND --- Game:handle_command: {:?}", command);
//         match command {
//             GameCommand::HostGame { user_id, game_id } => self.host_game(user_id, game_id),
//             GameCommand::JoinGame { user_id } => self.join_game(user_id),
//             GameCommand::PlayComputer { user_id, game_id } => self.play_computer(user_id, game_id),
//             GameCommand::CutForDeal { player } => self.cut_for_deal(player),
//             GameCommand::StartGame { player } => self.start_game(player),
//             GameCommand::DiscardCards { player, cards } => self.discard_cards(player, cards),
//             GameCommand::PlayCard { player, card } => self.play_card(player, card),
//             GameCommand::Go { player } => self.go(player),
//             GameCommand::ScorePone { player } => self.score_pone(player),
//             GameCommand::ScoreDealer { player } => self.score_dealer(player),
//             GameCommand::ScoreCrib { player } => self.score_crib(player),
//             GameCommand::StartNextRound { player } => self.start_next_round(player),
//         }
//     }
// }

// // #[cfg(test)]
// // impl From<Phase> for Game {
// //     fn from(state: Phase) -> Self {
// //         let id = GameId::new();
// //         let host = UserId::new();
// //         let guest = Some(UserId::new());
// //         let name = format!("test_game_{}_{}", state.as_ref(), chrono::Utc::now());
// //         Self {
// //             id,
// //             host,
// //             guest,
// //             name,
// //             phase: state,
// //         }
// //     }
// // }

// // #[cfg(test)]
// // #[coverage(off)]
// // mod test {}
