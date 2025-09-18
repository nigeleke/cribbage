use eventsourced::{Command, CommandEffect};

use crate::{
    GameError, Event, Finished, Game, GameId, Player, Playing, ScoreBreakdown, State, prettify,
};

#[derive(Debug)]
pub struct Pass {
    game_id: GameId,
    player: Player,
}

impl Pass {
    pub fn new(game_id: GameId, player: Player) -> Self {
        Self { game_id, player }
    }
}

impl Command<Game> for Pass {
    type Reply = ();
    type Error = GameError;

    fn handle_command(
        self,
        id: &GameId,
        state: &Game,
    ) -> CommandEffect<Game, Self::Reply, Self::Error> {
        match state.state() {
            State::Playing(playing) => {
                let player = self.player;

                if playing.play_state().next_to_play() != player {
                    CommandEffect::reject(GameError::NotPlayersTurn(player))
                } else if !playing.play_state().legal_plays(player).is_empty() {
                    CommandEffect::reject(GameError::InvalidPass)
                } else {
                    let (mut scoreboard, roles, hands, mut play_state, crib, cut) =
                        playing.clone().into_parts();

                    play_state.pass();

                    if play_state.all_players_passed() {
                        scoreboard.peg(player, &ScoreBreakdown::pass(&play_state));
                        play_state.start_new_play();
                    }

                    if let Some(winner) = scoreboard.winner() {
                        let finished = Finished::new(winner, scoreboard, roles, hands, crib, cut);
                        let state = State::Finished(finished);
                        CommandEffect::emit(Event::state_updated(*id, state))
                    } else {
                        let playing = Playing::new(scoreboard, roles, hands, play_state, crib, cut);
                        let state = State::Playing(playing);
                        CommandEffect::emit(Event::state_updated(*id, state))
                    }
                }
            }
            _ => CommandEffect::reject(Self::Error::NotPermitted(prettify!(CutCardAtStartOfPlay))),
        }
    }
}
