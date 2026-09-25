use crate::{Crib, Deck, Game, Hands, Roles, ScoreEvent};

use crate::game::{CutStarterOutcome, Cutting, Finished, Playing, Result};

impl Cutting {
    pub fn new(roles: Roles, hands: Hands, crib: Crib, deck: Deck) -> Self {
        Self {
            roles,
            hands,
            crib,
            deck,
        }
    }
}

pub fn cut_starter(mut game: Game<Cutting>) -> Result<CutStarterOutcome> {
    let dealer = game.state.roles.dealer();
    let starter = game.state.deck.cut();

    if let Some(event) = ScoreEvent::try_starter(dealer.player(), starter) {
        game.scoreboard.record_score(event);
    }

    let outcome = if let Some(_) = game.scoreboard.winner() {
        let game =
            game.transition(|state| Finished::new(state.roles, state.hands, state.crib, starter));
        CutStarterOutcome::Finished(game)
    } else {
        let game =
            game.transition(|state| Playing::new(state.roles, state.hands, state.crib, starter));
        CutStarterOutcome::Playing(game)
    };

    Ok(outcome)
}

impl std::fmt::Debug for Cutting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            r#"cutting(
  {:?}
  {:?}
  {:?}
  {:?}
)"#,
            self.roles, self.hands, self.crib, self.deck
        )
    }
}
