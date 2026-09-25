use crate::{Card, Deck, Game, Player, Roles};

use crate::game::{CutForDealOutcome, Dealing, GameError, Result, Starting};

impl Default for Starting {
    fn default() -> Self {
        Self {
            deck: Deck::new(),
            cuts: [None, None].into(),
        }
    }
}

pub fn cut_for_deal(
    mut game: Game<Starting>,
    player: Player,
    card: Card,
) -> Result<CutForDealOutcome> {
    let state = &mut game.state;

    state.cuts[player]
        .is_none()
        .then_some(())
        .ok_or(GameError::PlayerAlreadyCut)?;

    state
        .deck
        .contains(&card)
        .then_some(())
        .ok_or(GameError::CardNotInDeck)?;

    state.cuts[player] = Some(card);
    state.deck.remove(card);

    let all_cut = state.cuts.iter().all(|c| c.is_some());

    if all_cut {
        if let Ok(roles) = Roles::try_from(&state.cuts) {
            let game = game.transition(|_| Dealing::from(roles));
            Ok(CutForDealOutcome::Dealing(game))
        } else {
            state.cuts = [None, None].into();
            Ok(CutForDealOutcome::Starting(game))
        }
    } else {
        Ok(CutForDealOutcome::Starting(game))
    }
}

impl std::fmt::Debug for Starting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            r#"starting(
  {:?}
  {:?}
)"#,
            self.deck, self.cuts
        )
    }
}
