use crate::{Card, Error, Event, Game, GameId, Player, Playing, State, prettify};
use eventsourced::{Command, CommandEffect};
use eventsourced_ext::lift_effect;

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

impl Command<Playing> for Pass {
    type Reply = ();
    type Error = Error;

    fn handle_command(
        self,
        _id: &GameId,
        state: &Playing,
    ) -> CommandEffect<Playing, Self::Reply, Self::Error> {
        let Pass { game_id, player } = self;

        if state.play_state().next_to_play() != player {
            CommandEffect::reject(Error::NotPlayersTurn(player))
        } else if !state.play_state().legal_plays(player).is_empty() {
            CommandEffect::reject(Error::InvalidPass)
        } else {
            let event = Event::Passed { game_id, player };
            CommandEffect::emit(event)
        }
    }
}

impl Command<Game> for Pass {
    type Reply = ();
    type Error = Error;

    fn handle_command(
        self,
        id: &GameId,
        state: &Game,
    ) -> CommandEffect<Game, Self::Reply, Self::Error> {
        match state.state() {
            State::Playing(playing) => lift_effect!(
                playing,
                Pass::new(*id, self.player).handle_command(id, playing)
            ),
            _ => CommandEffect::reject(Self::Error::NotPermitted(prettify!(CutCardAtStartOfPlay))),
        }
    }
}
