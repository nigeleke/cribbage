use super::player::Player;
use super::card::Card;

#[derive(Debug, PartialEq)]
pub enum Error {
    ActionNotPermitted,
    CannotPass,
    CannotPlay,
    CannotPlayCard,
    CannotScorePone,
    CutForStartDecided,
    CutForStartUndecided,
    InvalidCard(Card),
    InvalidPlayer(Player),
    NotEnoughPlayers,
    TooManyDiscards,
    TooManyPlayers,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ActionNotPermitted => write!(f, "an action was attempted which is not permitted in the current game state"),
            Error::CannotPass => write!(f, "not player's turn to pass"),
            Error::CannotPlay => write!(f, "not player's turn to play a card"),
            Error::CannotPlayCard => write!(f, "cannot play the desired card"),
            Error::CannotScorePone => write!(f, "cannot score pone as it is still possible to play cards"),
            Error::CutForStartDecided => write!(f, "cannot redraw for start of game as cut for dealer was decisive"),
            Error::CutForStartUndecided => write!(f, "cannot start game as there is not a descisive cut for dealer"),
            Error::InvalidCard(c) => write!(f, "the card {} does belong to the player", c),
            Error::InvalidPlayer(p) => write!(f, "player {} is not a participant of the current game", p),
            Error::NotEnoughPlayers => write!(f, "there are not enough players to allow ther current game to be started"),
            Error::TooManyDiscards => write!(f, "only two cards can be discarded to the crib"),
            Error::TooManyPlayers => write!(f, "too many players requested for new game"),
        }
    }
}

impl std::error::Error for Error {}

pub(crate) type Result<T> = std::result::Result<T, Error>;

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
            "not player's turn to pass",
            "not player's turn to play a card",
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