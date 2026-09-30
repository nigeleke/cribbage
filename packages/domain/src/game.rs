mod plays;
mod state;

pub(crate) use plays::*;

#[cfg(test)]
#[coverage(off)]
pub(crate) mod tests;

// ------------------------------------
use crate::{Card, Crib, CutsForDeal, Deck, Discard, Discards, Hands, Player, Roles, Scoreboard};

/// A cribbage game in a specific phase of play.
///
/// `Game<T>` is a typestate wrapper: the type parameter `T` encodes the current
/// phase (`Starting`, `Dealing`, `Discarding`, …). Only the methods that are
/// legal in that phase are available, so illegal sequences are rejected at
/// compile time.
///
/// The game carries a [`Scoreboard`] that accumulates every scoring event for
/// the lifetime of the match. State transitions consume `self` and return an
/// outcome enum that holds the next `Game<…>`.
///
/// # Example flow
///
/// ```text
/// Game<Starting>
///     → cut_for_deal  → Game<Dealing>
///     → deal          → Game<Discarding>
///     → discard       → Game<Cutting>
///     → cut_starter   → Game<Playing>
///     → play / go     → Game<ScoringPone>
///     → score_pone    → Game<ScoringDealer>
///     → score_dealer  → Game<ScoringCrib>
///     → score_crib    → Game<Dealing>   (next round)
///                     ↘ Game<Finished>  (when a player reaches the winning score)
/// ```
#[derive(PartialEq, Eq)]
pub struct Game<T>
where
    T: std::fmt::Debug + std::cmp::PartialEq + std::cmp::Eq,
{
    scoreboard: Scoreboard,
    state: T,
}

/// Opening phase: players cut the deck to decide who deals first.
///
/// Each player selects one card from the deck. The player who cuts the
/// **lower** card (ace low) becomes the dealer (the pone is the other player).
/// Equal ranks force a re-cut and the game remains in this state.
#[derive(PartialEq, Eq)]
pub struct Starting {
    deck: Deck,
    cuts: CutsForDeal,
}

/// Roles have been assigned; the dealer is about to deal the hands.
///
/// Calling [`Game::deal`] distributes six cards to each player and moves the
/// game into the [`Discarding`] phase.
#[derive(PartialEq, Eq)]
pub struct Dealing {
    roles: Roles,
}

/// Each player must discard two cards from their six-card hand into the crib.
///
/// The game stays in this phase until both players have discarded. Once both
/// discards are recorded the four cards form the crib and the game advances to
/// [`Cutting`].
#[derive(PartialEq, Eq)]
pub struct Discarding {
    roles: Roles,
    hands: Hands,
    deck: Deck,
    discards: Discards,
}

/// Hands and crib are set; the starter (cut) card is about to be revealed.
///
/// Calling [`Game::cut_starter`] draws the top card of the remaining deck.
/// If that card is a Jack the dealer scores “His Heels” (two points). The game
/// then enters the pegging ([`Playing`]) phase, or finishes immediately if the
/// heels points end the match.
#[derive(PartialEq, Eq)]
pub struct Cutting {
    roles: Roles,
    hands: Hands,
    crib: Crib,
    deck: Deck,
}

/// Pegging phase: players alternately play cards toward a running total of 31.
///
/// A player may play a card whose value does not exceed the remaining distance
/// to 31, or call “go” when they have no legal play. Points are scored for
/// fifteens, pairs, runs, last card, and reaching 31. When neither player can
/// play any further cards the game moves to hand-scoring ([`ScoringPone`]), or
/// finishes if a player has already reached the winning total.
#[derive(PartialEq, Eq)]
pub struct Playing {
    roles: Roles,
    hands: Hands,
    crib: Crib,
    starter: Card,
    play_state: PlayState,
}

/// Hand-counting phase, parameterised by which hand is being scored.
///
/// The type parameter distinguishes the three successive scoring steps:
///
/// * [`ScorePone`] – non-dealer’s hand
/// * [`ScoreDealer`] – dealer’s hand
/// * [`ScoreCrib`] – the crib (belongs to the dealer)
///
/// Each step awards points for combinations in the four-card hand plus the
/// starter, then checks whether the game has been won before advancing.
#[derive(PartialEq, Eq)]
pub struct Scoring<T> {
    roles: Roles,
    hands: Hands,
    crib: Crib,
    starter: Card,
    _marker: std::marker::PhantomData<T>,
}

/// Marker indicating that the pone’s (non-dealer’s) hand is being scored.
#[derive(Debug, PartialEq, Eq)]
pub struct ScorePone;

/// Marker indicating that the dealer’s hand is being scored.
#[derive(Debug, PartialEq, Eq)]
pub struct ScoreDealer;

/// Marker indicating that the crib is being scored.
#[derive(Debug, PartialEq, Eq)]
pub struct ScoreCrib;

/// Convenience alias for the phase that scores the pone’s hand.
pub type ScoringPone = Scoring<ScorePone>;

/// Convenience alias for the phase that scores the dealer’s hand.
pub type ScoringDealer = Scoring<ScoreDealer>;

/// Convenience alias for the phase that scores the crib.
pub type ScoringCrib = Scoring<ScoreCrib>;

/// Terminal state: one player has reached or exceeded the winning score.
///
/// No further commands are accepted. The final hands, crib, starter and
/// (optional) last play-state are retained for inspection or display.
#[derive(PartialEq, Eq)]
pub struct Finished {
    roles: Roles,
    hands: Hands,
    crib: Crib,
    starter: Card,
    play_state: Option<PlayState>,
}

/// Errors that can arise when a command is rejected by the domain rules.
#[derive(Debug)]
pub enum GameError {
    /// The given player has already cut a card in the current deal-cut round.
    PlayerAlreadyCut,

    /// The requested card is not present in the deck.
    CardNotInDeck,

    /// The given player has already discarded to the crib this round.
    PlayerAlreadyDiscarded,

    /// One or more of the supplied cards are not in the player’s hand.
    CardsNotInHand,

    /// It is not the given player’s turn to act.
    OutOfTurn,

    /// The card cannot legally be played (would exceed 31, or is not in hand).
    InvalidPlay,

    /// “Go” was declared when the player still has a legal play, or when go
    /// has already been resolved for this sequence.
    InvalidGo,
}

/// Domain-level result type used by all state-transition methods.
pub(crate) type Result<T> = std::result::Result<T, GameError>;

/// Possible results of a cut-for-deal action.
#[derive(Debug)]
pub enum CutForDealOutcome {
    /// Only one player has cut so far (or the cuts tied); the game remains
    /// in the starting phase for another cut.
    Starting(Game<Starting>),

    /// Both players have cut unequal ranks; roles are assigned and the game
    /// advances to dealing.
    Dealing(Game<Dealing>),
}

impl Game<Starting> {
    /// Creates a new game in the [`Starting`] phase with the given deck.
    ///
    /// The scoreboard is empty. The supplied `deck` is used for the opening
    /// cut-for-deal; it is typically already shuffled by the caller. The domain
    /// does not shuffle.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cribbage_domain::prelude::*;
    /// let deck = Deck::new();
    /// let game = Game::new(deck);
    /// ```
    pub fn new(deck: Deck) -> Self {
        Self {
            scoreboard: Default::default(),
            state: Starting::new(deck),
        }
    }

    /// Records a player’s cut for the deal.
    ///
    /// The chosen `card` must still be in the deck and the `player` must not
    /// have cut already. When both players have cut:
    ///
    /// * unequal ranks → transitions to [`Game<Dealing>`]
    /// * equal ranks   → stays in [`Game<Starting>`] for a re-cut
    ///
    /// # Errors
    ///
    /// * [`GameError::PlayerAlreadyCut`] – the player already selected a card
    /// * [`GameError::CardNotInDeck`] – the card is not available
    pub fn cut_for_deal(self, player: Player, card: Card) -> Result<CutForDealOutcome> {
        state::cut_for_deal(self, player, card)
    }
}

/// Possible results of dealing the hands.
#[derive(Debug)]
pub enum DealOutcome {
    /// Six cards have been dealt to each player; the game is ready for
    /// discards.
    Discarding(Game<Discarding>),
}

impl Game<Dealing> {
    /// Deals six cards to each player from the supplied `deck`.
    ///
    /// On success the game transitions to [`Game<Discarding>`].
    ///
    /// The caller is responsible for shuffling the deck before passing it in;
    /// this method performs a pure deal with no additional randomisation.
    pub fn deal(self, deck: Deck) -> Result<DealOutcome> {
        state::deal(self, deck)
    }
}

/// Possible results of a discard action.
#[derive(Debug)]
pub enum DiscardOutcome {
    /// Only one player has discarded so far; waiting for the other
    Discarding(Game<Discarding>),

    /// Both players have discarded; the crib is formed and the game advances
    /// to cutting the starter.
    Cutting(Game<Cutting>),
}

impl Game<Discarding> {
    /// Discards two cards from the specified player’s hand into the crib.
    ///
    /// The two cards must belong to the player’s current hand and the player
    /// must not have discarded already this round. The game remains in
    /// [`Discarding`] until both players have discarded, at which point it
    /// advances to [`Cutting`].
    ///
    /// # Errors
    ///
    /// * [`GameError::PlayerAlreadyDiscarded`] – the player already discarded
    /// * [`GameError::CardsNotInHand`] – one or both cards are not in the hand
    pub fn discard(self, player: Player, discard: Discard) -> Result<DiscardOutcome> {
        state::discard(self, player, discard)
    }
}

/// Possible results of cutting the starter card.
#[derive(Debug)]
pub enum CutStarterOutcome {
    /// Starter revealed; pegging begins.
    Playing(Game<Playing>),

    /// His Heels (or a prior score) reached the winning total; the game is over.
    Finished(Game<Finished>),
}

impl Game<Cutting> {
    /// Cuts the starter card from the remaining deck to begin pegging.
    ///
    /// If the starter is a Jack the dealer scores “His Heels” (two points).
    /// Should those points (or any earlier score) reach the winning total the
    /// game finishes immediately; otherwise it enters the [`Playing`] phase.
    pub fn cut_starter(self) -> Result<CutStarterOutcome> {
        state::cut_starter(self)
    }
}

/// Possible results of playing a card during pegging.
#[derive(Debug)]
pub enum PlayOutcome {
    /// The card was accepted; pegging continues.
    Playing(Game<Playing>),

    /// All cards have been played (or both players have called go); hand
    /// scoring begins with the pone.
    Scoring(Game<ScoringPone>),

    /// The play awarded enough points to win the game.
    Finished(Game<Finished>),
}

/// Possible results of declaring “go” during pegging.
#[derive(Debug)]
pub enum GoOutcome {
    /// Go was accepted; pegging continues (opponent may still play, or a new
    /// sequence starts).
    Playing(Game<Playing>),

    /// Neither player can play further; hand scoring begins with the pone.
    Scoring(Game<ScoringPone>),

    /// The go (or resulting last-card / 31 score) won the game.
    Finished(Game<Finished>),
}

impl Game<Playing> {
    /// Plays a card for the specified player during the pegging phase.
    ///
    /// The card must be in the player’s hand and must not cause the running
    /// total to exceed 31. Points are awarded for any combinations completed
    /// by the play (fifteen, pair, run, thirty-one, last card).
    ///
    /// # Errors
    ///
    /// * [`GameError::OutOfTurn`] – it is not this player’s turn
    /// * [`GameError::InvalidPlay`] – the card is illegal or not in hand
    pub fn play(self, player: Player, card: Card) -> Result<PlayOutcome> {
        state::play(self, player, card)
    }

    /// Declares “go” for the specified player.
    ///
    /// Legal only when the player has no card that can be played without
    /// exceeding 31. If the opponent also cannot play, the current sequence
    /// ends (last-card / thirty-one points may be awarded) and a new sequence
    /// may begin, or the pegging phase finishes entirely.
    ///
    /// # Errors
    ///
    /// * [`GameError::OutOfTurn`] – it is not this player’s turn
    /// * [`GameError::InvalidGo`] – the player still has a legal play, or go
    ///   has already been resolved
    pub fn go(self, player: Player) -> Result<GoOutcome> {
        state::go(self, player)
    }
}

/// Possible results of scoring the pone’s hand.
#[derive(Debug)]
pub enum ScorePoneOutcome {
    /// Pone’s hand scored; proceed to the dealer’s hand.
    Scoring(Game<ScoringDealer>),

    /// The pone’s points reached the winning total.
    Finished(Game<Finished>),
}

impl Game<Scoring<ScorePone>> {
    /// Scores the pone’s (non-dealer’s) four-card hand plus the starter.
    ///
    /// Points are awarded for fifteens, pairs, runs, flushes and “his nobs”.
    /// If the resulting total reaches the winning score the game finishes;
    /// otherwise it advances to scoring the dealer’s hand.
    pub fn score_pone(self) -> Result<ScorePoneOutcome> {
        state::score_pone(self)
    }
}

/// Possible results of scoring the dealer’s hand.
#[derive(Debug)]
pub enum ScoreDealerOutcome {
    /// Dealer’s hand scored; proceed to the crib.
    Scoring(Game<ScoringCrib>),

    /// The dealer’s points reached the winning total.
    Finished(Game<Finished>),
}

impl Game<Scoring<ScoreDealer>> {
    /// Scores the dealer’s four-card hand plus the starter.
    ///
    /// Same combination rules as the pone’s hand. On success the game either
    /// finishes (winning score reached) or advances to scoring the crib.
    pub fn score_dealer(self) -> Result<ScoreDealerOutcome> {
        state::score_dealer(self)
    }
}

/// Possible results of scoring the crib.
#[derive(Debug)]
pub enum ScoreCribOutcome {
    /// Crib scored and no winner yet; a new round begins with roles swapped.
    Dealing(Game<Dealing>),

    /// The crib points reached the winning total.
    Finished(Game<Finished>),
}

impl Game<Scoring<ScoreCrib>> {
    /// Scores the crib (four discarded cards plus the starter).
    ///
    /// The crib belongs to the dealer. Flush scoring requires all five cards
    /// (hand + starter) to be the same suit. After scoring, roles are swapped
    /// and a new round starts at [`Dealing`], unless a player has won.
    pub fn score_crib(self) -> Result<ScoreCribOutcome> {
        state::score_crib(self)
    }
}

impl<T> Game<T>
where
    T: std::fmt::Debug + std::cmp::PartialEq + std::cmp::Eq,
{
    fn transition<U>(self, f: impl FnOnce(T) -> U) -> Game<U>
    where
        U: std::fmt::Debug + std::cmp::PartialEq + std::cmp::Eq,
    {
        let Game { scoreboard, state } = self;
        Game {
            scoreboard,
            state: f(state),
        }
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
