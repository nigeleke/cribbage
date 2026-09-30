use crate::{Card, Crib, Event, Game, Hands, Roles};

use crate::game::{
    Dealing, Finished, Result, ScoreCribOutcome, ScoreDealerOutcome, ScorePoneOutcome, Scoring,
    ScoringCrib, ScoringDealer, ScoringPone,
};

impl<T> Scoring<T> {
    pub(crate) fn new(roles: Roles, hands: Hands, crib: Crib, starter: Card) -> Self {
        Scoring {
            roles,
            hands,
            crib,
            starter,
            _marker: std::marker::PhantomData,
        }
    }

    fn into_finished<U: PartialEq + Eq>(game: Game<Scoring<U>>) -> Game<Finished> {
        game.transition(|state| Finished::new(state.roles, state.hands, state.crib, state.starter))
    }
}

pub fn score_pone(mut game: Game<ScoringPone>) -> Result<ScorePoneOutcome> {
    let player = game.state.roles.pone().player();
    let hand = &game.state.hands[player];
    let starter = game.state.starter;

    game.scoreboard
        .record(Event::try_pone_hand(player, hand, starter));

    let outcome = if game.scoreboard.winner().is_some() {
        ScorePoneOutcome::Finished(ScoringPone::into_finished(game))
    } else {
        let game = game.transition(|state| {
            ScoringDealer::new(state.roles, state.hands, state.crib, state.starter)
        });
        ScorePoneOutcome::Scoring(game)
    };

    Ok(outcome)
}

pub fn score_dealer(mut game: Game<ScoringDealer>) -> Result<ScoreDealerOutcome> {
    let player = game.state.roles.dealer().player();
    let hand = &game.state.hands[player];
    let starter = game.state.starter;

    game.scoreboard
        .record(Event::try_dealer_hand(player, hand, starter));

    let outcome = if game.scoreboard.winner().is_some() {
        ScoreDealerOutcome::Finished(ScoringDealer::into_finished(game))
    } else {
        let game = game.transition(|state| {
            ScoringCrib::new(state.roles, state.hands, state.crib, state.starter)
        });
        ScoreDealerOutcome::Scoring(game)
    };

    Ok(outcome)
}

pub fn score_crib(mut game: Game<ScoringCrib>) -> Result<ScoreCribOutcome> {
    let player = game.state.roles.dealer().player();
    let crib = &game.state.crib;
    let starter = game.state.starter;

    game.scoreboard
        .record(Event::try_crib(player, crib, starter));

    let outcome = if game.scoreboard.winner().is_some() {
        ScoreCribOutcome::Finished(ScoringCrib::into_finished(game))
    } else {
        let game = game.transition(|mut state| {
            state.roles.swap();
            Dealing::from(state.roles)
        });
        ScoreCribOutcome::Dealing(game)
    };

    Ok(outcome)
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
