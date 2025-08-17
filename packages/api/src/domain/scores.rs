// use crate::{NUMBER_OF_PLAYERS_IN_GAME, PlayerRef, Score};

// pub type Scores = [Score; NUMBER_OF_PLAYERS_IN_GAME];

// pub trait ScoresExt {
//     fn score(&self, player_ref: PlayerRef) -> &Score;
//     fn score_mut(&mut self, player_ref: PlayerRef) -> &mut Score;
// }

// impl ScoresExt for Scores {
//     fn score(&self, player_ref: PlayerRef) -> &Score {
//         &self[player_ref]
//     }

//     fn score_mut(&mut self, player_ref: PlayerRef) -> &mut Score {
//         &mut self[player_ref]
//     }
// }
