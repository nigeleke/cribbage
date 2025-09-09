use crate::{Card, Error, Event, EventKind, Game, GameId, Player, Playing, State, prettify};
use eventsourced::{Command, CommandEffect};
use eventsourced_ext::lift_effect;

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

impl Command<Playing> for PlayCard {
    type Reply = ();
    type Error = Error;

    fn handle_command(
        self,
        id: &GameId,
        state: &Playing,
    ) -> CommandEffect<Playing, Self::Reply, Self::Error> {
        let player = self.player;
        let card = self.card;

        if state.play_state().next_to_play() != player {
            CommandEffect::reject(Error::NotPlayersTurn(player))
        } else if !(state.hand(player).contains(card)
            && state.play_state().legal_plays(player).contains(&card))
        {
            CommandEffect::reject(Error::InvalidPlay(card))
        } else {
            CommandEffect::emit(Event::new(*id, EventKind::CardPlayed { player, card }))
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
            State::Playing(playing) => lift_effect!(
                playing,
                PlayCard::new(*id, self.player, self.card).handle_command(id, playing)
            ),
            _ => CommandEffect::reject(Self::Error::NotPermitted(prettify!(CutCardAtStartOfPlay))),
        }
    }
}
