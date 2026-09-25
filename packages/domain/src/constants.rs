/// The number of players in the game.
pub const PLAYER_COUNT: usize = 2;

/// The number of cards in a standard deck of cards.
pub const STANDARD_DECK_SIZE: usize = 52;

/// The number of cards dealt to each player's hand at the start of a round.
pub const CARDS_DEALT_PER_HAND: usize = 6;

/// The number of cards each player keeps in their hand after discarding.
pub const CARDS_KEPT_PER_HAND: usize = 4;

/// The number of cards each player discards to the crib.
pub const CARDS_DISCARDED_TO_CRIB: usize = CARDS_DEALT_PER_HAND - CARDS_KEPT_PER_HAND;

/// The number of cards in the crib, once all players discarded.
pub const CARDS_IN_CRIB: usize = 4;

/// The target score for the play phase, where players lay down cards.
pub const PLAY_TARGET: usize = 31;

/// The minimum number of cards that make up a run.
pub const MINIMUM_RUN_LENGTH: usize = 3;

/// The score required to win the game.
pub const WINNING_SCORE: usize = 121;
