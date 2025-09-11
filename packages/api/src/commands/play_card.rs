use crate::{Card, Error, Event, EventKind, Game, GameId, Player, State, prettify};
use eventsourced::{Command, CommandEffect};

#[derive(Debug)]
pub struct PlayCard {
    game_id: GameId,
    player: Player,
    card: Card,
}

impl PlayCard {
    pub fn new(game_id: GameId, player: Player, card: Card) -> Self {
        Self {
            game_id,
            player,
            card,
        }
    }
}

impl Command<Game> for PlayCard {
    type Reply = ();
    type Error = Error;

    fn handle_command(
        self,
        id: &GameId,
        state: &Game,
    ) -> CommandEffect<Game, Self::Reply, Self::Error> {
        match state.state() {
            State::Playing(playing) => {
                let player = self.player;
                let card = self.card;

                if playing.play_state().next_to_play() != player {
                    CommandEffect::reject(Error::NotPlayersTurn(player))
                } else if !(playing.hand(player).contains(card)
                    && playing.play_state().legal_plays(player).contains(&card))
                {
                    CommandEffect::reject(Error::InvalidPlay(card))
                } else {
                    CommandEffect::emit(Event::new(*id, EventKind::CardPlayed { player, card }))
                }
            }
            _ => CommandEffect::reject(Self::Error::NotPermitted(prettify!(CutCardAtStartOfPlay))),
        }
    }
}
