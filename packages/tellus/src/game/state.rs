use cribbage_domain::prelude::{
    Cutting, Dealing, Discarding, Finished, Game, Playing, ScoringCrib, ScoringDealer, ScoringPone,
    Starting,
};

use crate::game::Users;

#[derive(Debug, Default)]
pub enum GameState {
    #[default]
    PendingCreation,

    Starting {
        game: Game<Starting>,
        users: Users,
    },

    Dealing {
        game: Game<Dealing>,
        users: Users,
    },

    Discarding {
        game: Game<Discarding>,
        users: Users,
    },

    Cutting {
        game: Game<Cutting>,
        users: Users,
    },

    Playing {
        game: Game<Playing>,
        users: Users,
    },

    ScoringPone {
        game: Game<ScoringPone>,
        users: Users,
    },

    ScoringDealer {
        game: Game<ScoringDealer>,
        users: Users,
    },

    ScoringCrib {
        game: Game<ScoringCrib>,
        users: Users,
    },

    Finished {
        game: Game<Finished>,
        users: Users,
    },
}
