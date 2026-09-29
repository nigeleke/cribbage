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

#[cfg(test)]
#[coverage(off)]
mod tests {
    use crate::game::tests::GameFixture;

    #[test]
    fn has_debug_text() {
        let given = format!("{:?}", GameFixture::default().as_dealing());
        insta::assert_snapshot!(given, @r"
        game(dealing(
          roles(dealer(player-0), pone(player-1))
        )

          score(player-0: 0->0 player-1: 0->0) <- [])
        ");
    }
}
