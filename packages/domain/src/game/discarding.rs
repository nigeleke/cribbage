use crate::constants::CARDS_IN_CRIB;
use crate::{Card, Crib, Deck, Discard, Discards, Game, Hands, Player, Roles};

use crate::game::{Cutting, DiscardOutcome, Discarding, GameError, Result};

impl Discarding {
    pub fn new(roles: Roles, hands: Hands, deck: Deck) -> Self {
        Discarding {
            roles,
            hands,
            deck,
            discards: Discards::default(),
        }
    }

    fn all_discards(&self) -> Vec<Card> {
        self.discards.iter().copied().flatten().flatten().collect()
    }

    fn discard(&mut self, player: Player, discard: Discard) -> Result<()> {
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

pub fn discard(
    mut game: Game<Discarding>,
    player: Player,
    discard: Discard,
) -> Result<DiscardOutcome> {
    game.state.discard(player, discard)?;

    let all_discards = game.state.all_discards();

    if all_discards.len() == CARDS_IN_CRIB {
        let crib = Crib::from(all_discards);

        let game = game.transition(|state| {
            let roles = state.roles;
            let hands = state.hands;
            let deck = state.deck;
            Cutting::new(roles, hands, crib, deck)
        });

        Ok(DiscardOutcome::Cutting(game))
    } else {
        Ok(DiscardOutcome::Discarding(game))
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
