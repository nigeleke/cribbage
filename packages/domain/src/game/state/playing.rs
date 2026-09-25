use crate::{Card, Crib, Hands, Player, Roles};

use crate::game::{
    Finished, Game, GameError, PlayOutcome, PlayState, Playing, Result, ScoringPone,
};

impl Playing {
    pub fn new(roles: Roles, hands: Hands, crib: Crib, starter: Card) -> Self {
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
        .ok_or(GameError::PlayOutOfTurn)?;

    (state.hands[player].contains(&card))
        .then_some(())
        .ok_or(GameError::CardsNotInHand)?;

    (state.play_state.legal_plays(player).contains(&card))
        .then_some(())
        .ok_or(GameError::InvalidPlay)?;

    let score = state.play_state.play(card);
    state.hands[player].remove(card);

    if let Some(event) = score {
        game.scoreboard.record_score(event);
    }

    let outcome = if let Some(_) = game.scoreboard.winner() {
        let game = game.transition(|state| {
            Finished::new(state.roles, state.hands, state.crib, state.starter)
                .with_play_state(state.play_state)
        });
        PlayOutcome::Finished(game)
    } else {
        let all_played = state.hands.iter().all(|h| h.is_empty());
        if all_played {
            let game = game.transition(|mut state| ScoringPone {
                roles: state.roles,
                hands: state.play_state.finish_plays(),
                crib: state.crib,
                starter: state.starter,
                _marker: std::marker::PhantomData,
            });
            PlayOutcome::Scoring(game)
        } else {
            PlayOutcome::Playing(game)
        }
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
