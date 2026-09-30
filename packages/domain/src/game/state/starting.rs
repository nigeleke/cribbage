use crate::{Card, Deck, Game, Player, Roles};

use crate::game::{CutForDealOutcome, Dealing, GameError, Result, Starting};

impl Starting {
    pub(crate) fn new(deck: Deck) -> Self {
        Self {
            deck,
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

    state.deck.contains(&card).ok_or(GameError::CardNotInDeck)?;

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

#[cfg(test)]
#[coverage(off)]
mod tests {
    use crate::game::tests::GameFixture;

    #[test]
    fn has_debug_text() {
        let given = format!("{:?}", GameFixture::default().as_starting());
        insta::assert_snapshot!(given, @r"
        game(starting(
          Deck([AH, 2H, 3H, 4H, 5H, 6H, 7H, 8H, 9H, TH, JH, QH, KH, AC, 2C, 3C, 4C, 5C, 6C, 7C, 8C, 9C, TC, JC, QC, KC, AD, 2D, 3D, 4D, 5D, 6D, 7D, 8D, 9D, TD, JD, QD, KD, AS, 2S, 3S, 4S, 5S, 6S, 7S, 8S, 9S, TS, JS, QS, KS])
          [None, None]
        )

          score(player-0: 0->0 player-1: 0->0) <- [])
        ");
    }
}
