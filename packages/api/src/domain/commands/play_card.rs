use eventsourced::{Command, CommandEffect};

use crate::{
    Card, GameError, Event, Finished, Game, GameId, Pending, Player, Playing, ScoreBreakdown,
    ScoringPone, State, prettify,
};

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
    type Error = GameError;

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
                    CommandEffect::reject(GameError::NotPlayersTurn(player))
                } else if !(playing.hand(player).contains(card)
                    && playing.play_state().legal_plays(player).contains(&card))
                {
                    CommandEffect::reject(GameError::InvalidPlay(card))
                } else {
                    let (mut scoreboard, roles, mut hands, mut play_state, crib, cut) =
                        playing.clone().into_parts();

                    hands[player].remove(card);
                    play_state.play(card);

                    let breakdown = ScoreBreakdown::play_card(&play_state);
                    scoreboard.peg(player, &breakdown);

                    if let Some(winner) = scoreboard.winner() {
                        let finished = Finished::new(winner, scoreboard, roles, hands, crib, cut);
                        let state = State::Finished(finished);
                        CommandEffect::emit(Event::state_updated(*id, state))
                    } else if play_state.all_cards_are_played() {
                        let hands = play_state.finish_plays();
                        let pending = Pending::default();
                        let breakdown = ScoreBreakdown::hand(&hands[roles.pone()], cut);
                        let scoring = ScoringPone::new(
                            scoreboard, roles, hands, crib, cut, breakdown, pending,
                        );
                        let state = State::ScoringPone(scoring);
                        CommandEffect::emit(Event::state_updated(*id, state))
                    } else {
                        if play_state.is_current_play_finished() {
                            play_state.start_new_play();
                        }
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
