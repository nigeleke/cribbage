/// The number of players in the game.
pub(crate) const PLAYER_COUNT: usize = 2;

#[cfg(test)]
/// The number of cards in a standard deck of cards.
//  Only used in tests.
pub(crate) const STANDARD_DECK_SIZE: usize = 52;

/// The number of cards dealt to each player's hand at the start of a round.
pub(crate) const CARDS_DEALT_PER_HAND: usize = 6;

/// The number of cards each player keeps in their hand after discarding.
pub(crate) const CARDS_KEPT_PER_HAND: usize = 4;

/// The number of cards each player discards to the crib.
pub(crate) const CARDS_DISCARDED_TO_CRIB: usize = CARDS_DEALT_PER_HAND - CARDS_KEPT_PER_HAND;

/// The number of cards in the crib, once all players discarded.
pub(crate) const CARDS_IN_CRIB: usize = 4;

/// The target score for the play phase, where players lay down cards.
pub(crate) const PLAY_TARGET: usize = 31;

/// The minimum number of cards that make up a run.
pub(crate) const MINIMUM_RUN_LENGTH: usize = 3;

/// The score required to win the game.
pub(crate) const WINNING_SCORE: usize = 121;
