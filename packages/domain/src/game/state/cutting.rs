use crate::{
    Crib, CutStarterOutcome, Cutting, Deck, Event, Finished, Game, Hands, Playing, Result, Roles,
};

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

    game.scoreboard
        .record(Event::try_starter(dealer.player(), starter));

    let outcome = if game.scoreboard.winner().is_some() {
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

#[cfg(test)]
#[coverage(off)]
mod tests {
    use crate::game::tests::GameFixture;

    #[test]
    fn has_debug_text() {
        let given = format!("{:?}", GameFixture::default().as_cutting());
        insta::assert_snapshot!(given, @r"
        game(cutting(
          roles(dealer(player-0), pone(player-1))
          [[], []]
          []
          [AH, 2H, 3H, 4H, 5H, 6H, 7H, 8H, 9H, TH, JH, QH, KH, AC, 2C, 3C, 4C, 5C, 6C, 7C, 8C, 9C, TC, JC, QC, KC, AD, 2D, 3D, 4D, 5D, 6D, 7D, 8D, 9D, TD, JD, QD, KD, AS, 2S, 3S, 4S, 5S, 6S, 7S, 8S, 9S, TS, JS, QS, KS]
        )

          score(player-0: 0->0 player-1: 0->0) <- [])
        ");
    }
}
