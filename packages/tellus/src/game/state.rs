use cribbage_domain::prelude::{
    Cutting, Dealing, Discarding, Finished, Game, Playing, ScoringCrib, ScoringDealer, ScoringPone,
    Starting,
};

use crate::game::Users;

/// The current state of a cribbage game and its participating users.
#[derive(Debug, Default, PartialEq, Eq)]
pub enum GameState {
    /// The game has not yet been created.
    #[default]
    PendingCreation,

    /// The game is awaiting the cut for the deal.
    Starting {
        /// The game in its starting phase.
        game: Game<Starting>,
        /// The users participating in the game.
        users: Users,
    },

    /// The cards are being dealt to the players.
    Dealing {
        /// The game in its dealing phase.
        game: Game<Dealing>,
        /// The users participating in the game.
        users: Users,
    },

    /// The players are discarding cards to form the crib.
    Discarding {
        /// The game in its discard phase.
        game: Game<Discarding>,
        /// The users participating in the game.
        users: Users,
    },

    /// The starter card is being cut.
    Cutting {
        /// The game in its starter-cut phase.
        game: Game<Cutting>,
        /// The users participating in the game.
        users: Users,
    },

    /// The players are playing cards.
    Playing {
        /// The game in its play phase.
        game: Game<Playing>,
        /// The users participating in the game.
        users: Users,
    },

    /// The pone's hand is being scored.
    ScoringPone {
        /// The game in its pone-scoring phase.
        game: Game<ScoringPone>,
        /// The users participating in the game.
        users: Users,
    },

    /// The dealer's hand is being scored.
    ScoringDealer {
        /// The game in its dealer-scoring phase.
        game: Game<ScoringDealer>,
        /// The users participating in the game.
        users: Users,
    },

    /// The crib is being scored.
    ScoringCrib {
        /// The game in its crib-scoring phase.
        game: Game<ScoringCrib>,
        /// The users participating in the game.
        users: Users,
    },

    /// The game has finished.
    Finished {
        /// The completed game.
        game: Game<Finished>,
        /// The users who participated in the game.
        users: Users,
    },
}

