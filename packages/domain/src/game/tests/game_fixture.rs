use macros::*;

use crate::{
    Call, Card, Crib, CutsForDeal, Dealer, Deck, Discards, Game, Hands, Player, Roles, Scoreboard,
};

use crate::game::{
    Cutting, Dealing, Discarding, Finished, GoStatus, Play, PlayState, Playing, Scoring,
    ScoringCrib, ScoringDealer, ScoringPone, Starting,
};

pub struct GameFixture {
    cuts: CutsForDeal,
    deck: Deck,

    roles: Roles,
    hands: Hands,
    discards: Discards,
    crib: Crib,
    starter: Card,

    next_to_play: Player,
    go_status: GoStatus,
    current_plays: Vec<Play>,
    previous_plays: Vec<Play>,

    scoreboard: Scoreboard,
}

impl GameFixture {
    pub fn with_cuts(mut self, cuts: [Option<&str>; 2]) -> Self {
        self.cuts = cuts.map(|card| card.map(|card| card!(card))).into();
        self
    }

    pub fn with_deck(mut self, deck: &str) -> Self {
        self.deck = deck!(deck);
        self
    }

    pub fn with_dealer(mut self, player: Player) -> Self {
        self.roles = Roles::new(Dealer::from(player));
        self
    }

    pub fn with_hands(mut self, hands: [&str; 2]) -> Self {
        self.hands = hands.map(|hand| hand!(hand)).into();
        self
    }

    pub fn with_discards(mut self, discards: [Option<&str>; 2]) -> Self {
        self.discards = discards
            .map(|cards| {
                cards.map(|cards| cards!(cards).try_into().expect("require valid discards"))
            })
            .into();
        self
    }

    pub fn with_crib(mut self, crib: &str) -> Self {
        self.crib = crib!(crib);
        self
    }

    pub fn with_starter(mut self, starter: &str) -> Self {
        self.starter = card!(starter);
        self
    }

    pub fn with_next_to_play(mut self, player: Player) -> Self {
        self.next_to_play = player;
        self
    }

    pub fn with_go_status(mut self, go_status: GoStatus) -> Self {
        self.go_status = go_status;
        self
    }

    pub fn with_current_plays(mut self, plays: &[Play]) -> Self {
        self.current_plays = plays.to_vec();
        self
    }

    pub fn with_previous_plays(mut self, plays: &[Play]) -> Self {
        self.previous_plays = plays.to_vec();
        self
    }

    pub fn at_120(mut self, player: Player) -> Self {
        self.scoreboard = Scoreboard::at_120(player);
        self
    }

    pub fn with_calls(mut self, player: Player, calls: &[Call]) -> Self {
        self.scoreboard = self.scoreboard.with_calls(player, &calls);
        self
    }

    pub fn as_cutting(self) -> Game<Cutting> {
        Game {
            state: Cutting {
                roles: self.roles,
                hands: self.hands,
                crib: self.crib,
                deck: self.deck,
            },
            scoreboard: self.scoreboard,
        }
    }

    pub fn as_dealing(self) -> Game<Dealing> {
        Game {
            state: Dealing { roles: self.roles },
            scoreboard: self.scoreboard,
        }
    }

    pub fn as_discarding(self) -> Game<Discarding> {
        Game {
            state: Discarding {
                roles: self.roles,
                hands: self.hands,
                deck: self.deck,
                discards: self.discards,
            },
            scoreboard: self.scoreboard,
        }
    }

    pub fn as_finished(self) -> Game<Finished> {
        Game {
            state: Finished {
                roles: self.roles,
                hands: self.hands,
                crib: self.crib,
                starter: self.starter,
                play_state: None,
            },
            scoreboard: self.scoreboard,
        }
    }

    pub fn as_finished_with_play_state(self) -> Game<Finished> {
        let play_state = PlayState::new(self.next_to_play, &self.hands)
            .with_go_status(self.go_status)
            .with_current_plays(&self.current_plays)
            .with_previous_plays(&self.previous_plays);

        Game {
            state: Finished {
                roles: self.roles,
                hands: self.hands,
                crib: self.crib,
                starter: self.starter,
                play_state: Some(play_state),
            },
            scoreboard: self.scoreboard,
        }
    }

    pub fn as_playing(self) -> Game<Playing> {
        let play_state = PlayState::new(self.next_to_play, &self.hands)
            .with_go_status(self.go_status)
            .with_current_plays(&self.current_plays)
            .with_previous_plays(&self.previous_plays);

        Game {
            state: Playing {
                roles: self.roles,
                hands: self.hands,
                crib: self.crib,
                starter: self.starter,
                play_state,
            },
            scoreboard: self.scoreboard,
        }
    }

    pub fn as_scoring_pone(self) -> Game<ScoringPone> {
        self.as_scoring()
    }

    pub fn as_scoring_dealer(self) -> Game<ScoringDealer> {
        self.as_scoring()
    }

    pub fn as_scoring_crib(self) -> Game<ScoringCrib> {
        self.as_scoring()
    }

    fn as_scoring<T: PartialEq + Eq>(self) -> Game<Scoring<T>> {
        Game {
            state: Scoring {
                roles: self.roles,
                hands: self.hands,
                crib: self.crib,
                starter: self.starter,
                _marker: std::marker::PhantomData,
            },
            scoreboard: self.scoreboard,
        }
    }

    pub fn as_starting(self) -> Game<Starting> {
        Game {
            state: Starting {
                deck: self.deck,
                cuts: self.cuts,
            },
            scoreboard: self.scoreboard,
        }
    }
}

impl Default for GameFixture {
    fn default() -> Self {
        Self {
            cuts: Default::default(),
            deck: Deck::new(),
            roles: Roles::new(Dealer::from(Player::Player0)),
            hands: Default::default(),
            discards: Discards::default(),
            crib: Default::default(),
            starter: card!("AS"),
            next_to_play: Player::Player1,
            go_status: Default::default(),
            current_plays: Default::default(),
            previous_plays: Default::default(),
            scoreboard: Default::default(),
        }
    }
}
