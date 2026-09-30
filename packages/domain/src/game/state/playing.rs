use crate::{Card, Crib, Hands, Player, Roles};

use crate::game::{
    Finished, Game, GameError, GoOutcome, PlayOutcome, PlayState, Playing, Result, ScoringPone,
};

impl Playing {
    pub(crate) fn new(roles: Roles, hands: Hands, crib: Crib, starter: Card) -> Self {
        let play_state = PlayState::new(roles.pone().player(), &hands);

        Self {
            roles,
            hands,
            crib,
            starter,
            play_state,
        }
    }
}

pub fn play(mut game: Game<Playing>, player: Player, card: Card) -> Result<PlayOutcome> {
    let state = &mut game.state;

    (state.play_state.next_to_play() == player)
        .then_some(())
        .ok_or(GameError::OutOfTurn)?;

    (state.hands[player].contains(&card))
        .then_some(())
        .ok_or(GameError::CardsNotInHand)?;

    (state.play_state.legal_plays(player).contains(&card))
        .then_some(())
        .ok_or(GameError::InvalidPlay)?;

    state.hands[player].remove(card);

    let score = state.play_state.play(card);
    game.scoreboard.record(score);

    let outcome = if game.scoreboard.winner().is_some() {
        let game = game.transition(|state| {
            Finished::new(state.roles, state.hands, state.crib, state.starter)
                .with_play_state(state.play_state)
        });
        PlayOutcome::Finished(game)
    } else {
        if state.play_state.is_finished() {
            let game = game.transition(|mut state| {
                ScoringPone::new(
                    state.roles,
                    state.play_state.finish_plays(),
                    state.crib,
                    state.starter,
                )
            });
            PlayOutcome::Scoring(game)
        } else {
            PlayOutcome::Playing(game)
        }
    };

    Ok(outcome)
}

pub fn go(mut game: Game<Playing>, player: Player) -> Result<GoOutcome> {
    let state = &mut game.state;

    (state.play_state.next_to_play() == player)
        .then_some(())
        .ok_or(GameError::OutOfTurn)?;

    (state.play_state.legal_plays(player).is_empty())
        .then_some(())
        .ok_or(GameError::InvalidGo)?;

    let score = state.play_state.go();
    game.scoreboard.record(score);

    let outcome = if game.scoreboard.winner().is_some() {
        let game = game.transition(|state| {
            Finished::new(state.roles, state.hands, state.crib, state.starter)
                .with_play_state(state.play_state)
        });
        GoOutcome::Finished(game)
    } else {
        GoOutcome::Playing(game)
    };

    Ok(outcome)
}

impl std::fmt::Debug for Playing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            r#"playing(
  {:?}
  {:?}
  {:?}
  {:?}
  {:?}
)"#,
            self.roles, self.hands, self.crib, self.starter, self.play_state
        )
    }
}

#[cfg(test)]
#[coverage(off)]
mod tests {
    use crate::game::tests::GameFixture;

    #[test]
    fn has_debug_text() {
        let given = format!("{:?}", GameFixture::default().as_playing());
        insta::assert_snapshot!(given, @r"
        game(playing(
          roles(dealer(player-0), pone(player-1))
          [Hand([]), Hand([])]
          Crib([])
          AS
          play_state: (next: player-1, go: not-called, current: [], previous: [])
        )

          score(player-0: 0->0 player-1: 0->0) <- [])
        ");
    }
}
