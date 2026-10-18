use crate::card::Card;
use crate::cards::Deck;
use crate::game::{Discarding, DomainError, Result};
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

    pub(crate) fn validate_discard(&self, player: Player, discard: Discard) -> Result<()> {
        self.discards[player]
            .is_none()
            .then_some(())
            .ok_or(DomainError::PlayerAlreadyDiscarded)?;

        discard
            .iter()
            .all(|card| self.hands[player].contains(card))
            .then_some(())
            .ok_or(DomainError::CardsNotInHand)?;

        Ok(())
    }

    pub(crate) fn discard(&mut self, player: Player, discard: Discard) -> Result<()> {
        self.validate_discard(player, discard)?;

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
    use super::*;
    use crate::{card, deck, hands, roles};
    use std::assert_matches;

    #[test]
    fn player_can_discard_own_cards_to_the_crib() {
        let mut given = Discarding::new(
            roles!(Player0),
            hands!("AH2H3H4H5H6H", "AD2D3D4D5D6D"),
            deck!(""),
        );

        let outcome = given.discard(Player::Player0, [card!("AH"), card!("2H")].into());
        assert_matches!(outcome, Ok(()));
    }

    #[test]
    fn player_cannot_discard_unowned_cards_to_the_crib() {
        let mut given = Discarding::new(
            roles!(Player0),
            hands!("AH2H3H4H5H6H", "AD2D3D4D5D6D"),
            deck!(""),
        );
        let outcome = given.discard(Player::Player0, [card!("AH"), card!("2D")].into());

        assert_matches!(outcome, Err(DomainError::CardsNotInHand));
    }

    #[test]
    fn player_cannot_discard_if_already_discarded() {
        let mut given = Discarding::new(
            roles!(Player0),
            hands!("AH2H3H4H5H6H", "AD2D3D4D5D6D"),
            deck!(""),
        );
        let _ = given.discard(Player::Player0, [card!("AH"), card!("2H")].into());
        let outcome = given.discard(Player::Player0, [card!("3H"), card!("4H")].into());

        assert!(matches!(outcome, Err(DomainError::PlayerAlreadyDiscarded)));
    }

    #[test]
    fn has_debug_text() {
        let given = format!(
            "{:?}",
            Discarding::new(roles!(Player0), hands!("", ""), deck!(""))
        );
        insta::assert_snapshot!(given, @r"
        discarding(
          roles(dealer(player-0), pone(player-1))
          [Hand([]), Hand([])]
          Deck([])
          [None, None]
        )
        ");
    }
}
