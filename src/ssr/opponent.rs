use crate::domain::prelude::*;

pub struct Opponent;

impl Opponent {
    pub fn discard(opponent: Player, game: &Game) -> Game {
        match game {
            Game::Discarding(_, _dealer, hands, _, _) => {
                let hand = hands[&opponent].clone();
                // TODO: Analyse
                let discards = hand.cards();
                game.discard(opponent, &discards[0..=1]).ok().unwrap()
            },
            _ => unreachable!(),
        }
    }

    pub fn play(opponent: Player, game: &Game) -> Game {
        match game {
            Game::Playing(_, _dealer, _hands, play_state, _, _) => {
                let legal_plays = play_state.legal_plays(opponent).ok().unwrap();
                if legal_plays.is_empty() {
                    game.pass(opponent).ok().unwrap()
                } else {
                    // TODO: Analyse
                    let card = legal_plays.cards().into_iter().next().unwrap();
                    game.play(opponent, card).ok().unwrap()
                }
            },
            _ => unreachable!(),
        }
    }
}