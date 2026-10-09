use crate::card::Card;
use crate::players::Player;

/// Represents a single play in the pegging phase of the game.
///
/// A `Play` pairs a player with the card they played.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Play {
    player: Player,
    card: Card,
}

impl Play {
    /// Creates a new `Play` with the given player and card.
    pub(crate) const fn new(player: Player, card: Card) -> Self {
        Self { player, card }
    }

    /// Returns the player who made this play.
    pub const fn player(self) -> Player {
        self.player
    }

    /// Returns the card played.
    pub const fn card(self) -> Card {
        self.card
    }
}

impl std::fmt::Debug for Play {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.player, self.card)
    }
}

pub(crate) fn plays_to_string(plays: &[Play]) -> String {
    plays
        .iter()
        .map(|p| format!("{:?}", p))
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
#[coverage(off)]
mod tests {
    use super::*;

    use crate::card;

    #[test]
    fn has_debug_text() {
        let given = format!(
            "{:?}",
            plays_to_string(&[
                Play::new(Player::Player1, card!("AH")),
                Play::new(Player::Player0, card!("2H"))
            ])
        );
        insta::assert_snapshot!(given, @r#""(player-1, AH), (player-0, 2H)""#);
    }
}
