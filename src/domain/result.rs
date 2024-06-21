use super::cards::Card;

use crate::types::Player;

use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("an action was attempted which is not permitted in the current game state")]
    ActionNotPermitted,

    #[error("not this player's turn to pass")]
    CannotPass,

    #[error("not this player's turn to play a card")]
    CannotPlay,

    #[error("cannot play the desired card")]
    CannotPlayCard,

    #[error("cannot score pone as it is still possible to play cards")]
    CannotScorePone,

    #[error("cannot redraw for start of game as cut for dealer was decisive")]
    CutForStartDecided,

    #[error("cannot start game as there is not a descisive cut for dealer")]
    CutForStartUndecided,

    #[error("the card {0} does belong to the player")]
    InvalidCard(Card),

    #[error("player {0} is not a participant of the current game")]
    InvalidPlayer(Player),

    #[error("there are not enough players to allow ther current game to be started")]
    NotEnoughPlayers,

    #[error("only two cards can be discarded to the crib")]
    TooManyDiscards,

    #[error("too many players requested for new game")]
    TooManyPlayers,
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn errors_can_be_displayed() {
        let card = Card::from("AS");
        let player = Player::new();
        let errors = vec![
            Error::ActionNotPermitted,
            Error::CannotPass,
            Error::CannotPlay,
            Error::CannotPlayCard,
            Error::CannotScorePone,
            Error::CutForStartDecided,
            Error::CutForStartUndecided,
            Error::InvalidCard(card),
            Error::InvalidPlayer(player),
            Error::NotEnoughPlayers,
            Error::TooManyDiscards,
            Error::TooManyPlayers,
        ];

        let expected_invalid_card = format!("the card {} does belong to the player", card);
        let expected_invalid_player = format!("player {} is not a participant of the current game", player);
        let expected = vec![
            "an action was attempted which is not permitted in the current game state",
            "not this player's turn to pass",
            "not this player's turn to play a card",
            "cannot play the desired card",
            "cannot score pone as it is still possible to play cards",
            "cannot redraw for start of game as cut for dealer was decisive",
            "cannot start game as there is not a descisive cut for dealer",
            &expected_invalid_card,
            &expected_invalid_player,
            "there are not enough players to allow ther current game to be started",
            "only two cards can be discarded to the crib",
            "too many players requested for new game",
        ];

        for (i, error) in errors.into_iter().enumerate() {
            assert_eq!(error.to_string(), expected[i])
        }
    }
}