use std::str::FromStr;

use crate::constants::*;
use crate::*;

#[derive(Debug)]
pub struct GameBuilder {
    dealer: usize,
    scoreboard: Scoreboard,
    hands: Vec<Hand>,
    current_plays: Vec<Play>,
    previous_plays: Vec<Play>,
    pass_count: usize,
    crib: Crib,
    cut: Option<Card>,
    deck: Deck,
}

#[coverage(off)]
impl GameBuilder {
    pub fn new() -> Self {
        Self {
            dealer: usize::default(),
            scoreboard: Scoreboard::default(),
            hands: Vec::default(),
            current_plays: Vec::default(),
            previous_plays: Vec::default(),
            pass_count: 0,
            crib: Crib::default(),
            cut: None,
            deck: Deck::shuffled_pack(),
        }
    }

    pub fn with_peggings(mut self, points0: usize, points1: usize) -> Self {
        let score0 = Pegging::default() + Points::from(points0);
        let score1 = Pegging::default() + Points::from(points1);

        let scores = [score0, score1];

        self.scoreboard = Scoreboard::new(scores);
        self
    }

    pub fn with_hands(mut self, hand0: &str, hand1: &str) -> Self {
        let mut add_hand = |hand: &str| {
            let deal = cards!(hand);
            self.deck.remove_all(&deal);

            let hand = hand!(hand);
            self.hands.push(hand);
        };

        add_hand(hand0);
        add_hand(hand1);

        self
    }

    pub fn with_crib(mut self, crib: &str) -> Self {
        let discards = Vec::from(crib!(crib).as_ref());
        self.deck.remove_all(&discards);
        self.crib.add_all(&discards);
        self
    }

    pub fn with_cut(mut self, cut: &str) -> Self {
        let cut = card!(cut);
        self.deck.remove(cut);
        self.cut = Some(cut);
        self
    }

    pub fn with_current_plays(mut self, plays: &[(usize, &str)]) -> Self {
        let plays = plays
            .iter()
            .map(|(p, c)| Play::new(Player::from(*p), card!(c)));
        self.current_plays = Vec::from_iter(plays);
        self
    }

    pub fn with_previous_plays(mut self, plays: &[(usize, &str)]) -> Self {
        let plays = plays
            .iter()
            .map(|(p, c)| Play::new(Player::from(*p), card!(c)));
        self.previous_plays = Vec::from_iter(plays);
        self
    }

    pub fn with_pass(mut self) -> Self {
        self.pass_count += 1;
        self
    }

    pub fn into_new(self) -> State {
        State::default()
    }

    pub fn into_starting(self) -> Starting {
        let mut deck = self.deck.clone();
        let mut cuts = Vec::<Cut>::default();

        if cuts.len() < PLAYER_COUNT {
            let n = PLAYER_COUNT - cuts.len();
            (0..n).for_each(|_| {
                let cut = deck.cut();
                cuts.push(cut);
            });
        }

        let cuts = [cuts[0], cuts[1]];
        Starting::new(cuts, deck, Pending::default())
    }

    pub fn into_discarding(self) -> Discarding {
        let scoreboard = self.scoreboard.clone();
        let roles = Roles::new(
            Dealer::from(PLAYERS[self.dealer]),
            Pone::from(PLAYERS[1 - self.dealer]),
        );
        let hands = [self.hands[0].clone(), self.hands[1].clone()];
        let crib = self.crib.clone();
        let deck = self.deck.clone();
        let pending = Pending::default();
        Discarding::new(scoreboard, roles, hands, crib, deck, pending)
    }

    pub fn into_playing(self, next_to_play: usize) -> Playing {
        let player = PLAYERS[next_to_play];
        let scoreboard = self.scoreboard.clone();

        let roles = Roles::new(
            Dealer::from(PLAYERS[self.dealer]),
            Pone::from(PLAYERS[1 - self.dealer]),
        );

        let hands = [self.hands[0].clone(), self.hands[1].clone()];
        let mut play_state = PlayState::new(player)
            .with_pending_plays(PLAYER0, hands[PLAYER0].as_ref())
            .with_pending_plays(PLAYER1, hands[PLAYER1].as_ref());
        play_state.force_pass_count(self.pass_count);

        self.current_plays
            .iter()
            .for_each(|p| play_state.force_current_play(p.player(), p.card()));
        self.previous_plays
            .iter()
            .for_each(|p| play_state.force_previous_play(p.player(), p.card()));
        let crib = self.crib.clone();
        let cut = self.cut.expect("cut");

        let hands = [hands[PLAYER0].clone(), hands[PLAYER1].clone()];

        Playing::new(scoreboard, roles, hands, play_state, crib, cut)
    }

    pub fn into_scoring_pone(self) -> ScoringPone {
        let scoreboard = self.scoreboard;

        let roles = Roles::new(
            Dealer::from(Player::from(self.dealer)),
            Pone::from(Player::from(1 - self.dealer)),
        );
        let hands = self.hands.as_slice();
        let crib = self.crib.clone();
        let cut = self.cut.expect("cut");

        let hands = [hands[PLAYER0].clone(), hands[PLAYER1].clone()];

        let breakdown = ScoreBreakdown::hand(&hands[roles.pone()], cut);
        let pending = Pending::default();

        ScoringPone::new(scoreboard, roles, hands, crib, cut, breakdown, pending)
    }

    pub fn into_scoring_dealer(self) -> ScoringDealer {
        let scoreboard = self.scoreboard;

        let roles = Roles::new(
            Dealer::from(Player::from(self.dealer)),
            Pone::from(Player::from(1 - self.dealer)),
        );
        let hands = self.hands.as_slice();
        let crib = self.crib.clone();
        let cut = self.cut.expect("cut");

        let hands = [hands[PLAYER0].clone(), hands[PLAYER1].clone()];

        let breakdown = ScoreBreakdown::hand(&hands[roles.dealer()], cut);
        let pending = Pending::default();

        ScoringDealer::new(scoreboard, roles, hands, crib, cut, breakdown, pending)
    }

    pub fn into_scoring_crib(self) -> ScoringCrib {
        let mut scoreboard = self.scoreboard.clone();

        let roles = Roles::new(
            Dealer::from(Player::from(self.dealer)),
            Pone::from(Player::from(1 - self.dealer)),
        );
        let hands = self.hands.as_slice();
        let crib = self.crib.clone();
        let cut = self.cut.expect("cut");

        let hands = [hands[PLAYER0].clone(), hands[PLAYER1].clone()];

        let breakdown = ScoreBreakdown::crib(&crib, cut);

        let pending = Pending::default();

        ScoringCrib::new(scoreboard, roles, hands, crib, cut, breakdown, pending)
    }

    // pub fn into_finished(self) -> Finished {
    //     let mut scoreboard = self.scoreboard.clone();
    //     let _ = scoreboard.peg_score(PLAYER0, &self.composition);
    //     let roles = Roles::new(
    //         Dealer::from(Player::from(self.dealer)),
    //         Pone::from(Player::from(1 - self.dealer)),
    //     );
    //     let hands = self.hands.as_slice();
    //     let crib = self.crib.clone();
    //     let cut = self.cut.expect("cut");
    //     let Some(winner) = self.winner else {
    //         panic!("must have winner")
    //     };
    //     let winner = Player::from(winner);

    //     let hands = [hands[PLAYER0].clone(), hands[PLAYER1].clone()];

    //     Finished::new(winner, scoreboard, roles, hands, crib, cut)
    // }
}

impl Default for GameBuilder {
    fn default() -> Self {
        Self::new()
    }
}
