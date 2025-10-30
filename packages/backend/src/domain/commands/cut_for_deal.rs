use std::cmp::Ordering;

use eventsourced::{Command, CommandEffect};

use crate::domain::constants::PLAYER_COUNT;
use crate::domain::{
    Crib, Cut, Dealer, Deck, Discarding, DomainError, Event, Game, GameId, PLAYER0, PLAYER1,
    Pending, Player, Roles, Scoreboard, Starting, State,
};
use crate::prettify;

#[derive(Debug)]
pub struct CutForDeal {
    game_id: GameId,
    player: Player,
}

impl CutForDeal {
    pub fn new(game_id: GameId, player: Player) -> Self {
        Self { game_id, player }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CutForDealReply {
    cut: Cut,
    can_proceed: bool,
}

impl CutForDealReply {
    pub fn cut(&self) -> Cut {
        self.cut
    }

    pub fn can_proceed(&self) -> bool {
        self.can_proceed
    }
}

impl Command<Game> for CutForDeal {
    type Reply = CutForDealReply;
    type Error = DomainError;

    fn handle_command(
        self,
        id: &GameId,
        game: &Game,
    ) -> CommandEffect<Game, Self::Reply, Self::Error> {
        if game.id() != id {
            return CommandEffect::reject(DomainError::InvalidGame(*id));
        };

        match game.state() {
            State::Starting(starting) => {
                let (cuts, deck, pending) = &mut starting.clone().into_parts();

                let player = self.player;
                let cut = deck.cut();
                cuts[player] = cut;
                let can_proceed = pending.acknowledge(player);

                let as_starting = |cuts, deck, pending| {
                    let starting = Starting::new(cuts, deck, pending);
                    State::Starting(starting)
                };

                let as_discarding = |dealer| {
                    let scoreboard = Scoreboard::default();
                    let roles = Roles::new(dealer, dealer.opponent());
                    let crib = Crib::default();
                    let mut deck = Deck::shuffled_pack();
                    let hands = deck.deal(PLAYER_COUNT);
                    let hands = [hands[0].clone(), hands[1].clone()];
                    let discarding =
                        Discarding::new(scoreboard, roles, hands, crib, deck, pending.clone());
                    State::Discarding(discarding)
                };

                let state = if can_proceed {
                    match cuts[PLAYER0]
                        .face()
                        .rank()
                        .cmp(&cuts[PLAYER1].face().rank())
                    {
                        Ordering::Less => as_discarding(Dealer::from(PLAYER0)),
                        Ordering::Greater => as_discarding(Dealer::from(PLAYER1)),
                        Ordering::Equal => as_starting(*cuts, Deck::default(), Pending::default()),
                    }
                } else {
                    as_starting(*cuts, deck.clone(), pending.clone())
                };

                CommandEffect::emit_and_reply(Event::state_updated(*id, state), move |_| {
                    CutForDealReply { cut, can_proceed }
                })
            }
            _ => CommandEffect::reject(DomainError::NotPermitted(prettify!(CutForDeal))),
        }
    }
}
