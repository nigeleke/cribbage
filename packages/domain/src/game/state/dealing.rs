use crate::{Deck, Game, Roles};

use crate::game::{DealOutcome, Dealing, Discarding, Result};

impl From<Roles> for Dealing {
    fn from(roles: Roles) -> Self {
        Self { roles }
    }
}

pub fn deal(game: Game<Dealing>, mut deck: Deck) -> Result<DealOutcome> {
    let hands = deck.deal();

    let game = game.transition(|state| {
        let roles = state.roles;
        Discarding::new(roles, hands, deck)
    });

    Ok(DealOutcome::Discarding(game))
}

impl std::fmt::Debug for Dealing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "dealing(
  {:?}
)",
            self.roles
        )
    }
}
