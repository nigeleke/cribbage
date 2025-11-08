use crate::cqrs::Command;
use crate::domain::constants::PLAYER_COUNT;
use crate::domain::{
    Crib, Deck, Discarding, DomainError, Event, Game, PLAYER0, PLAYER1, Pending, Player, Roles,
    Scoreboard, Starting, State,
};

#[derive(Debug)]
pub struct CutForDeal {
    player: Player,
}

impl CutForDeal {
    pub fn new(player: Player) -> Self {
        Self { player }
    }
}

impl Command<Game, Event, DomainError> for CutForDeal {
    async fn execute(&self, mut game: Game) -> Result<(Vec<Event>, Game), DomainError> {
        let player = self.player;

        let cut_for_deal = |starting: &Starting| {
            let (cuts, deck, pending) = &mut starting.clone().into_parts();

            let cut = deck.cut();
            cuts[player] = Some(cut);
            let can_proceed = pending.acknowledge(player);

            let as_starting = |cuts, deck, pending| {
                let starting = Starting::new(cuts, deck, pending);
                State::Starting(starting)
            };

            let as_discarding = |roles| {
                let scoreboard = Scoreboard::default();
                let crib = Crib::default();
                let mut deck = Deck::shuffled_pack();
                let hands = deck.deal(PLAYER_COUNT);
                let hands = [hands[PLAYER0].clone(), hands[PLAYER1].clone()];
                let discarding =
                    Discarding::new(scoreboard, roles, hands, crib, deck, pending.clone());
                State::Discarding(discarding)
            };

            if can_proceed {
                if let Some(roles) = Roles::from_cuts_when_ready(cuts, pending) {
                    let dealer = roles.dealer().clone();
                    let events = Vec::from([
                        Event::CutForDealMade { player, cut },
                        Event::CutForDealDecided { dealer },
                    ]);
                    let discarding = as_discarding(roles);
                    (events, discarding)
                } else {
                    let events =
                        Vec::from([Event::CutForDealMade { player, cut }, Event::CutForDealTied]);
                    let starting = as_starting(*cuts, Deck::default(), Pending::default());
                    (events, starting)
                }
            } else {
                let events = Vec::from([Event::CutForDealMade { player, cut }]);
                let starting = as_starting(*cuts, deck.clone(), pending.clone());
                (events, starting)
            }
        };

        match game.state() {
            State::Starting(starting) => {
                let (events, new_state) = cut_for_deal(starting);
                game.set_state(new_state);
                Ok((events, game))
            }
            _ => Err(DomainError::NotPermitted(String::from("cut for deal"))),
        }
    }
}
