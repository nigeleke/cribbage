use crate::card::Card;
use crate::cards::Deck;
use crate::game::{Discarding, GameError, Result};
use crate::players::{Player, Roles};
use crate::types::{Discard, Discards, Hands};

impl Discarding {
    pub(crate) fn new(roles: Roles, hands: Hands, deck: Deck) -> Self {
        Discarding {
            roles,
            hands,
            deck,
            discards: Discards::default(),
        }
    }

    pub(crate) fn all_discards(&self) -> Vec<Card> {
        self.discards.iter().copied().flatten().flatten().collect()
    }

    pub(crate) fn discard(&mut self, player: Player, discard: Discard) -> Result<()> {
        self.discards[player]
            .is_none()
            .then_some(())
            .ok_or(GameError::PlayerAlreadyDiscarded)?;

        discard
            .iter()
            .all(|card| self.hands[player].contains(card))
            .then_some(())
            .ok_or(GameError::CardsNotInHand)?;

        self.hands[player].remove_all(&discard);
        self.discards[player] = Some(discard);

        Ok(())
    }
}

impl std::fmt::Debug for Discarding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            r#"discarding(
  {:?}
  {:?}
  {:?}
  {:?}
)"#,
            self.roles, self.hands, self.deck, self.discards
        )
    }
}

#[cfg(test)]
#[coverage(off)]
mod tests {
    use crate::game::tests::GameFixture;

    #[test]
    fn has_debug_text() {
        let given = format!("{:?}", GameFixture::default().as_discarding());
        insta::assert_snapshot!(given, @r"
        game(discarding(
          roles(dealer(player-0), pone(player-1))
          [Hand([]), Hand([])]
          Deck([AH, 2H, 3H, 4H, 5H, 6H, 7H, 8H, 9H, TH, JH, QH, KH, AC, 2C, 3C, 4C, 5C, 6C, 7C, 8C, 9C, TC, JC, QC, KC, AD, 2D, 3D, 4D, 5D, 6D, 7D, 8D, 9D, TD, JD, QD, KD, AS, 2S, 3S, 4S, 5S, 6S, 7S, 8S, 9S, TS, JS, QS, KS])
          [None, None]
        )

          score(player-0: 0->0 player-1: 0->0) <- [])
        ");
    }
}
