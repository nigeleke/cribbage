mod plays;
mod state;

pub(crate) use plays::*;
pub(crate) use state::*;

#[cfg(test)]
#[coverage(off)]
pub(crate) mod tests;

// ------------------------------------
use crate::{Card, Crib, CutsForDeal, Deck, Discard, Discards, Hands, Player, Roles, Scoreboard};

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
pub struct Starting {
    deck: Deck,
    cuts: CutsForDeal,
}

#[derive(PartialEq, Eq)]
pub struct Dealing {
    roles: Roles,
}

#[derive(PartialEq, Eq)]
pub struct Discarding {
    roles: Roles,
    hands: Hands,
    deck: Deck,
    discards: Discards,
}

#[derive(PartialEq, Eq)]
pub struct Cutting {
    roles: Roles,
    hands: Hands,
    crib: Crib,
    deck: Deck,
}

#[derive(PartialEq, Eq)]
pub struct Playing {
    roles: Roles,
    hands: Hands,
    crib: Crib,
    starter: Card,
    play_state: PlayState,
}

#[derive(PartialEq, Eq)]
pub struct Scoring<T> {
    roles: Roles,
    hands: Hands,
    crib: Crib,
    starter: Card,
    _marker: std::marker::PhantomData<T>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ScorePone;

#[derive(Debug, PartialEq, Eq)]
pub struct ScoreDealer;

#[derive(Debug, PartialEq, Eq)]
pub struct ScoreCrib;

pub type ScoringPone = Scoring<ScorePone>;
pub type ScoringDealer = Scoring<ScoreDealer>;
pub type ScoringCrib = Scoring<ScoreCrib>;

#[derive(PartialEq, Eq)]
pub struct Finished {
    roles: Roles,
    hands: Hands,
    crib: Crib,
    starter: Card,
    play_state: Option<PlayState>,
}

#[derive(Debug)]
pub enum GameError {
    PlayerAlreadyCut,
    CardNotInDeck,
    PlayerAlreadyDiscarded,
    CardsNotInHand,
    OutOfTurn,
    InvalidPlay,
    InvalidGo,
}

pub type Result<T> = std::result::Result<T, GameError>;

#[derive(Debug)]
pub enum CutForDealOutcome {
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
        state::cut_for_deal(self, player, card)
    }
}

#[derive(Debug)]
pub enum DealOutcome {
    Discarding(Game<Discarding>),
}

impl Game<Dealing> {
    /// Deals a hand to each player using the supplied deck.
    ///
    /// On success, the game transitions to [`Game<Discarding>`].
    pub fn deal(self, deck: Deck) -> Result<DealOutcome> {
        state::deal(self, deck)
    }
}

#[derive(Debug)]
pub enum DiscardOutcome {
    Discarding(Game<Discarding>),
    Cutting(Game<Cutting>),
}

impl Game<Discarding> {
    /// Discards two cards from the specified player's hand to the crib.
    ///
    /// On success, the game remains in the discarding state until both
    /// players have discarded their cards.
    pub fn discard(self, player: Player, discard: Discard) -> Result<DiscardOutcome> {
        state::discard(self, player, discard)
    }
}

#[derive(Debug)]
pub enum CutStarterOutcome {
    Playing(Game<Playing>),
    Finished(Game<Finished>),
}

impl Game<Cutting> {
    /// Cuts the starter card to begin the play.
    ///
    /// On success, the game transitions from the cutting state to the playing
    /// state.
    pub fn cut_starter(self) -> Result<CutStarterOutcome> {
        state::cut_starter(self)
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
        state::play(self, player, card)
    }

    /// Declares go for the specified player.
    ///
    /// On success, the game either remains in the playing state or
    /// transitions to scoring or finished when the play ends the game.
    pub fn go(self, player: Player) -> Result<GoOutcome> {
        state::go(self, player)
    }
}

#[derive(Debug)]
pub enum ScorePoneOutcome {
    Scoring(Game<ScoringDealer>),
    Finished(Game<Finished>),
}

impl Game<Scoring<ScorePone>> {
    /// Scores the pone's hand.
    ///
    /// On success, the game transitions to scoring the dealer's hand,
    /// unless the game has been won and transitions to the finished state.
    pub fn score_pone(self) -> Result<ScorePoneOutcome> {
        state::score_pone(self)
    }
}

#[derive(Debug)]
pub enum ScoreDealerOutcome {
    Scoring(Game<ScoringCrib>),
    Finished(Game<Finished>),
}

impl Game<Scoring<ScoreDealer>> {
    /// Scores the dealer's hand.
    ///
    /// On success, the game transitions to scoring the crib,
    /// unless the game has been won and transitions to the finished state.
    pub fn score_dealer(self) -> Result<ScoreDealerOutcome> {
        state::score_dealer(self)
    }
}

#[derive(Debug)]
pub enum ScoreCribOutcome {
    Dealing(Game<Dealing>),
    Finished(Game<Finished>),
}

impl Game<Scoring<ScoreCrib>> {
    /// Scores the crib.
    ///
    /// On success, the game transitions to dealing the next round,
    /// unless the game has been won and transitions to the finished state.
    pub fn score_crib(self) -> Result<ScoreCribOutcome> {
        state::score_crib(self)
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
