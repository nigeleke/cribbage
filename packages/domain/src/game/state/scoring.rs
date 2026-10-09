use crate::card::Card;
use crate::cards::Crib;
use crate::game::{Finished, Game, Hands, Roles, Scoring};

impl<T> Scoring<T> {
    pub(crate) const fn new(roles: Roles, hands: Hands, crib: Crib, starter: Card) -> Self {
        Scoring {
            roles,
            hands,
            crib,
            starter,
            _marker: std::marker::PhantomData,
        }
    }

    pub(crate) fn into_finished<U: PartialEq + Eq>(game: Game<Scoring<U>>) -> Game<Finished> {
        game.transition(|state| Finished::new(state.roles, state.hands, state.crib, state.starter))
    }
}

impl<T> std::fmt::Debug for Scoring<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            r#"scoring(
  {:?}
  {:?}
  {:?}
  {:?}
)"#,
            self.roles, self.hands, self.crib, self.starter
        )
    }
}

#[cfg(test)]
#[coverage(off)]
mod tests {
    use crate::game::tests::GameFixture;

    #[test]
    fn has_debug_text() {
        let given = format!("{:?}", GameFixture::default().as_scoring_pone());
        insta::assert_snapshot!(given, @r"
        game(scoring(
          roles(dealer(player-0), pone(player-1))
          [Hand([]), Hand([])]
          Crib([])
          AS
        )

          score(player-0: 0->0 player-1: 0->0) <- [])
        ");
    }
}
