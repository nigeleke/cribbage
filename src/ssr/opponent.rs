use crate::domain::*;
use crate::types::Player;

pub struct Opponent;

impl Opponent {
    pub fn discard(opponent: Player, game: &Game) -> Game {
        match game {
            Game::Discarding(_, _dealer, hands, _, _) => {
                let hand = hands[&opponent].clone();
                // TODO: Analyse
                let discards = hand.as_ref();
                game.discard(opponent, &discards[0..=1]).ok().unwrap()
            },
            _ => unreachable!(),
        }
    }

    pub fn play(opponent: Player, game: &Game) -> Game {
        match game {
            Game::Playing(_, _, _, ref play_state, _, _) => {
                if play_state.next_to_play() == opponent {
                    let current_player_passed = play_state.pass_count() == 1;
                    let legal_plays = play_state.legal_plays(opponent).ok().unwrap();
                    if legal_plays.is_empty() {
                        game.pass(opponent).ok().unwrap()
                    } else {
                        // TODO: Analyse
                        let card = legal_plays.as_ref().into_iter().next().unwrap();
                        let mut game = game.play(opponent, *card).ok().unwrap();

                        let still_playing = matches!(game, Game::Playing(_, _, _, _, _, _));
                        if still_playing && current_player_passed {
                            game = Self::play(opponent, &game);
                        }
                        game
                    }
                } else {
                    game.clone()
                }
            },
            _ => unreachable!(),
        }
    }
}