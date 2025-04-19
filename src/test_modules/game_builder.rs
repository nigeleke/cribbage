use crate::domain::*;

use std::collections::HashMap;

#[derive(Debug)]
pub struct GameBuilder {
    players: Vec<Player>,
    dealer: usize,
    cuts: Vec<Card>,
    peggings: Vec<Pegging>,
    reasons: ScoreReasons,
    hands: Vec<Hand>,
    current_plays: Vec<Play>,
    previous_plays: Vec<Play>,
    pass_count: usize,
    crib: Crib,
    cut: Option<Card>,
    deck: Deck,
    winner: usize,
}

impl GameBuilder {
    pub fn new(player_count: usize) -> Self {
        Self {
            players: Vec::from_iter((0..player_count).map(|_| Player::new())),
            dealer: usize::default(),
            cuts: Vec::default(),
            peggings: Vec::default(),
            reasons: ScoreReasons::default(),
            hands: Vec::default(),
            current_plays: Vec::default(),
            previous_plays: Vec::default(),
            pass_count: 0,
            crib: Crib::default(),
            cut: None,
            deck: Deck::shuffled_pack(),
            winner: 0,
        }
    }

    pub fn players(&self) -> Vec<Player> {
        self.players.clone()
    }

    pub fn with_cuts(mut self, cuts: &str) -> Self {
        let cuts = Deck::from(cuts);
        self.deck.remove_all(cuts.as_ref());
        cuts.as_ref().iter().for_each(|c| self.cuts.push(*c));
        self
    }

    pub fn cuts(&self) -> Cuts {
        self.merged(self.cuts.clone())
    }

    pub fn with_peggings(mut self, points0: usize, points1: usize) -> Self {
        self.peggings.push(Pegging::default().add(points0.into()));
        self.peggings.push(Pegging::default().add(points1.into()));
        self
    }

    pub fn with_score_reasons(mut self, reasons: &[ScoreReason]) -> Self {
        self.reasons.add(reasons);
        self
    }

    pub fn with_hands(mut self, hand0: &str, hand1: &str) -> Self {
        let mut add_hand = |hand: &str| {
            let hand = Hand::from(hand);
            self.deck.remove_all(hand.as_ref());
            self.hands.push(hand);
        };

        add_hand(hand0);
        add_hand(hand1);

        self
    }

    pub fn with_crib(mut self, crib: &str) -> Self {
        let crib = Crib::from(crib);
        self.deck.remove_all(crib.as_ref());
        self.crib = crib;
        self
    }

    pub fn with_cut(mut self, cut: &str) -> Self {
        let cut = Card::from(cut);
        self.deck.remove(cut);
        self.cut = Some(cut);
        self
    }

    pub fn with_current_plays(mut self, plays: &[(usize, &str)]) -> Self {
        let plays = plays
            .iter()
            .map(|(p, c)| (self.players[*p], Card::from(*c)));
        let plays = plays.map(|(p, c)| Play::new(p, c));
        self.current_plays = Vec::from_iter(plays);
        self
    }

    pub fn with_previous_plays(mut self, plays: &[(usize, &str)]) -> Self {
        let plays = plays
            .iter()
            .map(|(p, c)| (self.players[*p], Card::from(*c)));
        let plays = plays.map(|(p, c)| Play::new(p, c));
        self.previous_plays = Vec::from_iter(plays);
        self
    }

    // pub fn with_pass(mut self) -> Self {
    //     self.pass_count += 1;
    //     self
    // }
    //

    pub fn with_winner(mut self, winner: usize) -> Self {
        self.winner = winner;
        self
    }

    pub fn into_new(self) -> Result<Game<Starting>, GameError> {
        Game::<Starting>::try_new(&Players::from_iter(self.players))
    }

    pub fn into_starting(self) -> Game<Starting> {
        let mut deck = self.deck.clone();
        let mut cuts = self.cuts.clone();

        if cuts.len() < self.players.len() {
            let n = self.players.len() - cuts.len();
            (0..n).for_each(|_| {
                let cut = deck.cut();
                cuts.push(cut);
            });
        }

        let cuts = self.cuts();
        Game::<_>::new(Starting::new(cuts, Deck::from(deck)))
    }

    pub fn into_discarding(self) -> Game<Discarding> {
        let players = self.players.clone();
        let peggings = self.peggings.clone();
        let peggings = self.merged(peggings);
        let mut scores = Scores::from(&peggings);
        scores.score_points(players[0], &self.reasons);
        let roles = Roles::new(players[self.dealer], players[1 - self.dealer]);
        let hands = self.hands.clone();
        let hands = self.merged(hands);
        let crib = self.crib.clone();
        let deck = self.deck.clone();
        Game::<_>::new(Discarding::new(scores, roles, hands, crib, deck))
    }

    pub fn into_playing(self, next_to_play: usize) -> Game<Playing> {
        let players = self.players.clone();
        let player = players[next_to_play];
        let peggings = self.peggings.clone();
        let peggings = self.merged(peggings);
        let mut scores = Scores::from(&peggings);
        scores.score_points(players[0], &self.reasons);
        let roles = Roles::new(players[self.dealer], players[1 - self.dealer]);
        let hands = self.hands.clone();
        let hands = self.merged(hands);
        let mut play_state = PlayState::new(player, &hands);
        play_state.force_pass_count(self.pass_count);
        self.current_plays
            .iter()
            .for_each(|p| play_state.force_current_play(p.player(), p.card()));
        self.previous_plays
            .iter()
            .for_each(|p| play_state.force_previous_play(p.player(), p.card()));
        let cut = self.cut.expect("cut");
        let crib = self.crib.clone();
        let playing_state = Playing::new(scores, roles, hands, play_state, cut, crib);
        Game::<_>::new(playing_state)
    }

    pub fn into_scoring_pone(self) -> Game<ScoringPone> {
        let players = self.players.clone();
        let peggings = self.peggings.clone();
        let peggings = self.merged(peggings);
        let mut scores = Scores::from(&peggings);
        scores.score_points(players[0], &self.reasons);
        let roles = Roles::new(players[self.dealer], players[1 - self.dealer]);
        let hands = self.hands.clone();
        let hands = self.merged(hands);
        let cut = self.cut.expect("cut");
        let crib = self.crib.clone();
        let scoring_state = ScoringPone::new(scores, roles, hands, cut, crib);
        Game::<_>::new(scoring_state)
    }

    pub fn into_scoring_dealer(self) -> Game<ScoringDealer> {
        let players = self.players.clone();
        let peggings = self.peggings.clone();
        let peggings = self.merged(peggings);
        let mut scores = Scores::from(&peggings);
        scores.score_points(players[0], &self.reasons);
        let roles = Roles::new(players[self.dealer], players[1 - self.dealer]);
        let hands = self.hands.clone();
        let hands = self.merged(hands);
        let cut = self.cut.expect("cut");
        let crib = self.crib.clone();
        let scoring_state = ScoringDealer::new(scores, roles, hands, cut, crib);
        Game::<_>::new(scoring_state)
    }

    pub fn into_scoring_crib(self) -> Game<ScoringCrib> {
        let players = self.players.clone();
        let peggings = self.peggings.clone();
        let peggings = self.merged(peggings);
        let mut scores = Scores::from(&peggings);
        scores.score_points(players[0], &self.reasons);
        let roles = Roles::new(players[self.dealer], players[1 - self.dealer]);
        let hands = self.hands.clone();
        let hands = self.merged(hands);
        let cut = self.cut.expect("cut");
        let crib = self.crib.clone();
        let scoring_state = ScoringCrib::new(scores, roles, hands, cut, crib);
        Game::<_>::new(scoring_state)
    }

    pub fn into_finished(self) -> Game<Finished> {
        let players = self.players.clone();
        let peggings = self.peggings.clone();
        let peggings = self.merged(peggings);
        let cut = self.cut.expect("cut defined");
        let finished_state = Finished::new(players[self.winner], peggings, cut);
        Game::<_>::new(finished_state)
    }

    fn merged<T>(&self, items: Vec<T>) -> HashMap<Player, T> {
        let players = self.players.clone();
        let zipped = players.into_iter().zip(items);
        zipped.collect()
    }
}

impl Default for GameBuilder {
    fn default() -> Self {
        GameBuilder::new(2)
    }
}
