use crate::domain::prelude::*;

pub struct Opponent;

impl Opponent {
    pub fn discard(opponent: &Player, game: &Game) -> Game {
        match game {
            Game::Discarding(_, _dealer, hands, _, _) => {
                let hand = hands[opponent].clone();
                // TODO: Analyse
                let discards = hand.cards();
                game.discard(opponent, &discards[0..=1]).ok().unwrap()
            },
            _ => unreachable!(),
        }
    }
}