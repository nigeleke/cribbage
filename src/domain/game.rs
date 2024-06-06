use super::constants::*;
use super::card::{Card, Cuts, Rank};
use super::cards::{Crib, Deck, Hand, Hands};
use super::format::format_hashmap;
use super::player::Player;
use super::plays::PlayState;
use super::result::{Error, Result};
use super::score::{Score, Scores};
use super::game_scorer::*;

#[cfg(test)]
use super::builder::Builder;

use serde::{Deserialize, Serialize};

use std::collections::{HashMap, HashSet};
use std::fmt::Display;

/// The game state, waiting for opponent, discarding, playing, scoring, finished.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Game {
    Starting(Cuts, Deck),
    Discarding(Scores, Player, Hands, Crib, Deck),
    Playing(Scores, Player, Hands, PlayState, Card, Crib),
    ScoringPone(Scores, Player, Hands, Card, Crib),
//     ScoringDealerCards(MyState, OpponentState, Cut, Crib),
//     ScoringCrib(MyState, OpponentState, Cut, Crib),
    Finished(Scores),
}

impl Game {
    pub fn new(players: &HashSet<Player>) -> Result<Game> {
        if players.len() > NUMBER_OF_PLAYERS_IN_GAME {
            return Err(Error::TooManyPlayers)
        }

        let make_cut = |(mut cuts, deck): (HashMap<Player, Card>, Deck), player: &Player| {
            let (cut, deck) = deck.cut();
            cuts.insert(*player, cut);
            (cuts, deck)
        };

        let deck = Deck::shuffled_pack();
        let (cuts, deck) = players.iter().fold((HashMap::new(), deck), make_cut);

        Ok(Game::Starting(cuts, deck))
    }

    pub fn players(&self) -> HashSet<Player> {
        match self {
            Game::Starting(cuts, _) => HashSet::from_iter(cuts.keys().cloned()),
            Game::Discarding(scores, _, _, _, _) => HashSet::from_iter(scores.keys().cloned()),
            Game::Playing(scores, _, _, _, _, _) => HashSet::from_iter(scores.keys().cloned()),
            Game::ScoringPone(scores, _, _, _, _) => HashSet::from_iter(scores.keys().cloned()),
            Game::Finished(scores) => HashSet::from_iter(scores.keys().cloned()),
        }
    }

    pub(crate) fn player_1_2(&self) -> (Player, Player) {
        let players = Vec::from_iter(self.players());
        (players[0], players[1])
    }

    pub fn deck(&self) -> Deck {
        match self {
            Game::Starting(_, deck) => deck.clone(),
            Game::Discarding(_, _, _, _, deck) => deck.clone(),
            _ => unreachable!(),
        }
    }

    pub fn scores(&self) -> Scores {
        match self {
            Game::Starting(_, _) => unreachable!(),
            Game::Discarding(scores, _, _, _, _) => scores,
            Game::Playing(scores, _, _, _, _, _) => scores,
            Game::ScoringPone(scores, _, _, _, _) => scores,
            Game::Finished(scores) => scores,
        }.clone()
    }

    fn has_winner(&self) -> bool {
        let scores = self.scores();
        !scores.values()
            .filter(|s| s.value() >= WINNING_SCORE)
            .collect::<Vec<_>>()
            .is_empty()
    }

    pub fn start(&self) -> Result<Self> {
        match self {
            Game::Starting(cuts, _) => {
                let players = self.players();
                verify::players(&players)?;
                verify::different_cuts(cuts)?;
                let scores: Scores = Scores::from_iter(players.iter().map(|&p| (p, Score::default())));
                let mut cuts = cuts.iter();
                let Some((player1, cut1)) = cuts.next() else { unreachable!() };
                let Some((player2, cut2)) = cuts.next() else { unreachable!() };
                let dealer = if cut1.rank() < cut2.rank() { player1 } else { player2 };
                let deck = Deck::shuffled_pack();
                let (hands, deck) = deck.deal(&players);
                let crib = Crib::default();
                Ok(Game::Discarding(scores, *dealer, hands, crib, deck))
            },
            _ => Err(Error::InvalidAction("start".into()))
        }
    }

    pub fn redraw(&self) -> Result<Self> {
        match self {
            Game::Starting(cuts, _) => {
                let players = self.players();
                verify::players(&players)?;
                verify::same_cuts(cuts)?;
                Ok(Game::new(&self.players())?)
            },
            _ => Err(Error::InvalidAction("redraw".into()))
        }
    }

    pub fn discard(&self, player: Player, discards: &[Card]) -> Result<Self> {
        match self {
            Game::Discarding(scores, dealer, hands, crib, deck) => {
                let players = self.players();
                verify::player(player, &players)?;

                let mut hands = hands.clone();
                let mut hand = hands[&player].clone();
                verify::discards(discards, &hand)?;

                hand.remove_all(discards);
                hands.insert(player, hand);

                let mut crib = crib.clone();
                crib.add(discards);

                if crib.len() == CARDS_REQUIRED_IN_CRIB {
                    let (cut, _) = deck.cut();
                    let pone = self.pone();
                    let play_state = PlayState::new(pone, &hands);
                    let game = Game::Playing(scores.clone(), *dealer, hands, play_state, cut, crib);
                    let score = GameScorer::his_heels_on_cut_pre_play(cut);
                    game.score_points(*dealer, score)
                } else {
                    Ok(Game::Discarding(scores.clone(), *dealer, hands, crib, deck.clone()))
                }
            },
            _ => Err(Error::InvalidAction("discard".into()))
        }
    }

    pub fn dealer(&self) -> Player {
        match self {
            Game::Starting(_, _) => unreachable!(),
            Game::Discarding(_, dealer, _, _, _) => *dealer,
            Game::Playing(_, dealer, _, _, _, _) => *dealer,
            Game::ScoringPone(_, dealer, _, _, _) => *dealer,
            Game::Finished(_) => unreachable!(),
        }
    }

    pub fn pone(&self) -> Player {
        let (player1, player2) = self.player_1_2();
        if self.dealer() == player1 { player2 } else { player1 }
    }

    pub fn opponent(&self, player: Player) -> Player {
        assert!(self.players().contains(&player));
        let (player1, player2) = self.player_1_2();
        if player == player1 { player2 } else { player1 }
    }

    fn score_points(&self, player: Player, score: usize) -> Result<Self> {
        let update = |mut scores: Scores| {
            scores.insert(player, scores[&player].add(score));
            scores
        };

        let mut game = match self {
            Game::Starting(_, _) => unreachable!(),

            Game::Discarding(scores, dealer, hands, crib, deck) => 
                Game::Discarding(update(scores.clone()), *dealer, hands.clone(), crib.clone(), deck.clone()),

            Game::Playing(scores, dealer, hands, play_state, cut, crib) =>
                Game::Playing(update(scores.clone()), *dealer, hands.clone(), play_state.clone(), *cut, crib.clone()),

            Game::ScoringPone(scores, dealer, hands, cut, crib) =>
                Game::ScoringPone(update(scores.clone()), *dealer, hands.clone(), *cut, crib.clone()),

            Game::Finished(_) => unreachable!(),
        };

        if game.has_winner() {
            let scores = game.scores();
            game = Game::Finished(scores)
        }

        Ok(game)
    }

    pub fn play(&self, player: Player, card: Card) -> Result<Game> {
        let mut game = self.clone();
        match game {
            Game::Playing(scores, dealer, ref mut hands, ref mut play_state, cut, crib) => {
                let players = self.players();
                verify::player(player, &players)?;

                let hand = hands.get_mut(&player).unwrap();
                let legal_plays = play_state.legal_plays(player)?;
                verify::card(card, &hand.cards())?;
                verify::card(card, &legal_plays.cards()).map_err(|_| Error::CannotPlayCard)?;

                hand.remove(card);
                play_state.play(card);
 
                let mut score = GameScorer::current_play(play_state);

                if play_state.target_reached() || play_state.finished_plays() {
                    score += GameScorer::end_of_play(play_state);
                    play_state.start_new_play();
                };

                game = if play_state.finished_plays() {
                    Game::ScoringPone(scores, dealer, hands.clone(), cut, crib.clone())
                } else {
                    Game::Playing(scores, dealer, hands.clone(), play_state.clone(), cut, crib)
                };

                game.score_points(player, score)
            },
            _ => unreachable!(),
        }
    }

    pub(crate) fn pass(&self, player: Player) -> Result<Game> {
        let mut game = self.clone();
        match game {
            Game::Playing(scores, dealer, ref mut hands, ref mut play_state, cut, crib) => {
                let players = self.players();
                verify::player(player, &players)?;

                let legal_plays = play_state.legal_plays(player)?;
                verify::no_legal_plays(&legal_plays.cards())?;

                play_state.pass();

                let mut score = 0;

                if play_state.pass_count() == NUMBER_OF_PLAYERS_IN_GAME {
                    score += GameScorer::end_of_play(play_state);                    
                    play_state.start_new_play();                    
                }

                game = if play_state.finished_plays() {
                    Game::ScoringPone(scores, dealer, hands.clone(), cut, crib.clone())
                } else {
                    Game::Playing(scores, dealer, hands.clone(), play_state.clone(), cut, crib)
                };
                game.score_points(player, score)

            },
            _ => unreachable!(),
        }
    }

}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Game::Starting(cuts, deck) =>
                write!(f, "Starting(Cuts({}), Deck({}))", format_hashmap(cuts), deck),

            Game::Discarding(scores, dealer, hands, crib, deck) =>
                write!(f, "Discarding(Scores({}), Dealer({}), Hands({}), Crib({}), Deck({}))",
                    format_hashmap(scores), dealer, format_hashmap(hands), crib, deck),

            Game::Playing(scores, dealer, hands, play_state, cut, crib) =>
                write!(f, "Playing(Scores({}), Dealer({}), Hands({}), PlayState({}), Cut({})), Crib({}))",
                    format_hashmap(scores), dealer, format_hashmap(hands), play_state, cut, crib),

            Game::ScoringPone(scores, dealer, hands, cut, crib) =>
                write!(f, "ScoringPone(Scores({}), Dealer({}), Hands({}), Cut({})), Crib({}))",
                    format_hashmap(scores), dealer, format_hashmap(hands), cut, crib),

            Game::Finished(scores) =>
                write!(f, "Finished(Scores({}))", format_hashmap(scores)),
        }
    }
}

mod verify {
    use super::*;

    pub(crate) fn players(players: &HashSet<Player>) -> Result<()> {
        if players.len() != NUMBER_OF_PLAYERS_IN_GAME {
            Err(Error::NotEnoughPlayers)
        } else {
            Ok(())
        }
    }

    pub(crate) fn player(player: Player, players: &HashSet<Player>) -> Result<()> {
        if !players.contains(&player) {
            Err(Error::InvalidPlayer(player))
        } else {
            Ok(())
        }
    }

    pub(crate) fn different_cuts(cuts: &HashMap<Player, Card>) -> Result<()> {
        let cuts: HashSet<Rank> = HashSet::from_iter(cuts.values().map(|c| c.rank()));
        if cuts.len() != NUMBER_OF_PLAYERS_IN_GAME {
            Err(Error::CutForStartUndecided)
        } else {
            Ok(())
        }
    }

    pub(crate) fn same_cuts(cuts: &HashMap<Player, Card>) -> Result<()> {
        let cuts: HashSet<Rank> = HashSet::from_iter(cuts.values().map(|c| c.rank()));
        if cuts.len() == NUMBER_OF_PLAYERS_IN_GAME {
            Err(Error::CutForStartDecided)
        } else {
            Ok(())
        }
    }

    pub(crate) fn discards(discards: &[Card], hand: &Hand) -> Result<()> {
        for discard in discards {
            verify::card(*discard, &hand.cards())?
        }

        if hand.len() - discards.len() < CARDS_KEPT_PER_HAND {
            Err(Error::TooManyDiscards)
        } else {
            Ok(())
        }
    }

    pub(crate) fn card(card: Card, cards: &[Card]) -> Result<()> {
        if !cards.contains(&card) {
            Err(Error::InvalidCard(card))
        } else {
            Ok(())
        }
    }

    pub(crate) fn no_legal_plays(cards: &[Card]) -> Result<()> {
        if cards.is_empty() {
            Ok(())
        } else {
            Err(Error::CannotPass)
        }
    }

}

/// # [Cribbage Rules](https://www.officialgamerules.org/cribbage)
#[cfg(test)]
mod test {
    use super::*;
    use crate::domain::card::{Cut, Face};

    /// ## Number of Players
    ///
    /// Two or three people can play. Or four people can play two against two as partners. But
    /// Cribbage is basically best played by two people, and the rules that follow are for that
    /// number.
    ///
    #[test]
    fn play_with_zero_one_or_two_players() {
        for n in 0..=2 {
            let builder = Builder::new(n);
            let players = builder.players.clone();
            let game = builder.as_new().ok().unwrap();
            assert_eq!(game.players().len(), n);
            for player in players.into_iter() {
                assert!(game.players().contains(&player))
            }
        }
    }

    #[test]
    fn fail_to_play_with_more_than_two_players() {
        let builder = Builder::new(3);
        let error = builder.as_new().err().unwrap();
        assert_eq!(error, Error::TooManyPlayers);
    }

    /// ## The Pack
    ///
    /// The standard 52-card pack is used.
    ///
    /// Rank of Cards: K (high), Q, J, 10, 9, 8, 7, 6, 5, 4, 3, 2, A.
    ///
    /// [See unit tests](./domain/deck.rs)
    ///
    #[test]
    fn use_a_standard_pack_of_cards() {
        let builder = Builder::new(2);
        let game = builder.as_new().ok().unwrap();
        let _deck = game.deck();
        assert!(true)
    }

    /// ## The Draw, Shuffle and Cut
    ///
    /// From a shuffled pack face down, each player cuts a card, leaving at least four cards at
    /// either end of the pack.
    ///
    /// If both players cut cards of the same rank, each draws again. The player with the lower card
    /// deals the first hand. Thereafter, the turn to deal alternates between the two players,
    /// except that the loser of the game deals first if another game is played. The dealer has the
    /// right to shuffle last, and he presents the cards to the non-dealer for the cut prior to the
    /// deal. (In some games, there is no cut at this time.)
    ///
    #[test]
    fn start_game_with_lowest_cut_as_dealer() {
        for (expected_dealer, cuts) in vec![(0, "ASKS"), (1, "KSAS")] {
            let builder = Builder::new(2)
                .with_cuts(cuts);
            let players = builder.players.clone();
            let game = builder.as_starting();
            let game = game.start().ok().unwrap();
            let Game::Discarding(_, dealer, _, _, _) = game else { panic!("Unexpected state") };
            assert_eq!(dealer, players[expected_dealer]);
        }
    }

    #[test]
    fn fail_to_start_game_if_cuts_are_the_same_value() {
        let game = Builder::new(2)
            .with_cuts("ASAC")
            .as_starting();
        let error = game.start().err().unwrap();
        assert_eq!(error, Error::CutForStartUndecided);
    }

    #[test]
    fn fail_to_start_game_if_not_enough_players() {
        let game = Builder::new(2)
            .with_cuts("AS")
            .as_starting();
        let error = game.start().err().unwrap();
        assert_eq!(error, Error::NotEnoughPlayers);
    }

    #[test]
    fn redraw_if_cuts_are_same_value() {
        let game = Builder::new(2)
            .with_cuts("ASAC")
            .as_starting();
        let game = game.redraw().ok().unwrap();
        let Game::Starting(ref _cuts, _) = game else { panic!("Unexpected state") };
        assert!(true)
    }

    #[test]
    fn fail_to_redraw_if_cuts_are_not_the_same_value() {
        let game = Builder::new(2)
            .with_cuts("ASKS")
            .as_starting();
        let error = game.redraw().err().unwrap();
        assert_eq!(error, Error::CutForStartDecided);
    }

    /// ## The Deal
    /// 
    /// The dealer distributes six cards face down to his opponent and himself, beginning with the
    /// opponent.
    /// 
    #[test]
    fn deal_six_cards_per_player() {
        let game = Builder::new(2)
            .with_cuts("ASKS")
            .as_starting();

        let game = game.start().ok().unwrap();
        let players = game.players();
        assert_eq!(players.len(), 2);

        let Game::Discarding(_, _, hands, _, _) = game else { panic!("Unexpected state") };

        players
            .iter()
            .for_each(|p| assert_eq!(hands[p].cards().len(), CARDS_DEALT_PER_HAND));
    }

    #[test]
    fn deal_when_draw_decided() {
        let game = Builder::new(2)
            .with_cuts("ASKS")
            .as_starting();
        let game = game.start().ok().unwrap();
        let players = game.players();

        let Game::Discarding(scores, _, hands, crib, deck) = game  else { panic!("Unexpected state") };
        
        players.iter().for_each(|p| {
            assert_eq!(scores[p].back_peg(), 0);
            assert_eq!(scores[p].front_peg(), 0);
        });
        
        players.iter().for_each(|p| {
            assert_eq!(hands[p].cards().len(), CARDS_DEALT_PER_HAND);
        });

        assert_eq!(crib.cards().len(), 0);
        assert_eq!(deck.cards().len(), 52 - (NUMBER_OF_PLAYERS_IN_GAME * CARDS_DEALT_PER_HAND));
    }

    #[test]
    #[ignore]
    fn deal_when_scored_crib() {

    }
  //     "scored crib" in dummyPlaying(
  //       poneCards = Seq(Card(Ace, Hearts)),
  //       inPlays = Seq(0 -> Card(Jack, Hearts), 0 -> Card(Queen, Hearts)),
  //       playeds = Seq(
  //         0 -> Card(Nine, Hearts),
  //         0 -> Card(Ten, Hearts),
  //         0 -> Card(Four, Spades),
  //         0 -> Card(Five, Spades),
  //         0 -> Card(Six, Spades)
  //       )
  //     ) { case playing0 @ Playing(_, _, dealer0, pone0, _, _, _) =>
  //       val discarding1 = doPlayFor[Discarding](pone0, Card(Ace, Hearts))(playing0)

  //       val Discarding(deck1, scores1, hands1, dealer1, pone1, crib1) = discarding1

  //       dealer1 should be(pone0)
  //       pone1 should be(dealer0)
  //       val players = Set(dealer1, pone1)
  //       deck1.size should be(40)
  //       scores1.keySet should be(players)
  //       hands1.keySet should be(players)
  //       hands1.foreach(_._2.size should be(CardsDealtPerHand))
  //       crib1.toSeq should be(empty)
  //       deck1.toSeq ++
  //         hands1(dealer1).toSeq ++
  //         hands1(pone1).toSeq should contain theSameElementsAs (Cards.fullDeck.toSeq)
  //     }
  //   }

    /// ## Object of the Game
    /// 
    /// The goal is to be the first player to score 121 points. (Some games are to 61 points.)
    /// Players earn points during play and for making various card combinations.
    ///

    /// ## The Crib
    /// 
    /// Each player looks at his six cards and "lays away" (discards) two of them face down to
    /// reduce the hand to four. The four cards laid away together constitute "the crib". The crib
    /// belongs to the dealer, but these cards are not exposed or used until after the hands have
    /// been played.
    ///
    #[test]
    fn player_can_discard_one_held_card_to_the_crib() {
        let game = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("AH2H3H4H5H6H", "AC2C3C4C5C6C")
            .as_discarding();

        let (player, opponent) = game.player_1_2();

        let Game::Discarding(scores0, dealer0, hands0, _, deck0) = game.clone()  else { panic!("Unexpected state") };
        let player_hand0 = hands0[&player].clone();
        let player_discard = player_hand0.get(&[0]);

        let opponent_hand0 = hands0[&opponent].clone();

        let game = game.discard(player, &player_discard).ok().unwrap();
        let Game::Discarding(scores1, dealer1, hands1, crib1, deck1) = game  else { panic!("Unexpected state") };

        let player_hand1 = hands1[&player].clone();
        let opponent_hand1 = hands1[&opponent].clone();

        assert_eq!(scores1, scores0);
        assert_eq!(dealer1, dealer0);
        assert!(player_hand1.contains_none(&player_discard));
        assert!(crib1.contains_all(&player_discard));
        assert_eq!(opponent_hand1, opponent_hand0);
        assert_eq!(deck1, deck0);
    }

    #[test]
    fn player_can_discard_two_held_cards_to_the_crib() {
        let game = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("AH2H3H4H5H6H", "AC2C3C4C5C6C")
            .as_discarding();

        let (player, opponent) = game.player_1_2();

        let Game::Discarding(scores0, dealer0, hands0, _, deck0) = game.clone()  else { panic!("Unexpected state") };
        let player_hand0 = hands0[&player].clone();
        let player_discard = player_hand0.get(&[0, 1]);

        let opponent_hand0 = hands0[&opponent].clone();

        let game = game.discard(player, &player_discard).ok().unwrap();
        let Game::Discarding(scores1, dealer1, hands1, crib1, deck1) = game  else { panic!("Unexpected state") };

        let player_hand1 = hands1[&player].clone();
        let opponent_hand1 = hands1[&opponent].clone();

        assert_eq!(scores1, scores0);
        assert_eq!(dealer1, dealer0);
        assert!(player_hand1.contains_none(&player_discard));
        assert!(crib1.contains_all(&player_discard));
        assert_eq!(opponent_hand1, opponent_hand0);
        assert_eq!(deck1, deck0);
    }

    #[test]
    fn player_cannot_discard_more_then_two_held_cards_to_the_crib() {
        let game = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("AH2H3H4H5H6H", "AC2C3C4C5C6C")
            .as_discarding();

        let (player, _) = game.player_1_2();

        let Game::Discarding(_, _, hands0, _, _) = game.clone()  else { panic!("Unexpected state") };
        let hand0 = hands0[&player].clone();
        let discard = hand0.get(&[0, 1, 2]);

        let error = game.discard(player, &discard).err().unwrap();
        assert_eq!(error, Error::TooManyDiscards);
    }

    #[test]
    fn player_cannot_discard_non_held_cards_to_the_crib() {
        let game = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("AH2H3H4H5H6H", "AC2C3C4C5C6C")
            .as_discarding();

        let (player, opponent) = game.player_1_2();

        let Game::Discarding(_, _, hands0, _, _) = game.clone()  else { panic!("Unexpected state") };
        let hand0 = hands0[&opponent].clone();
        let discard = hand0.get(&[0, 1]);

        let Error::InvalidCard(_) = game.discard(player, &discard).err().unwrap() else { panic!("Unexpected error") };
    }

    #[test]
    fn cannot_discard_when_player_not_participating() {
        let game = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("AH2H3H4H5H6H", "AC2C3C4C5C6C")
            .as_discarding();
        
        let (player, _) = game.player_1_2();

        let Game::Discarding(_, _, hands0, _, _) = game.clone()  else { panic!("Unexpected state") };
        let hand0 = hands0[&player].clone();
        let discard = hand0.get(&[0, 1]);

        let non_player = Player::new();
        let error = game.discard(non_player, &discard).err().unwrap();
        assert_eq!(error, Error::InvalidPlayer(non_player));
    }

    /// ## Before the Play
    /// 
    /// After the crib is laid away, the non-dealer cuts the pack. The dealer turns up the top card
    /// of the lower packet and places it face up on top of the pack. This card is the "starter." If
    /// the starter is a jack, it is called "His Heels," and the dealer pegs (scores) 2 points at
    /// once. The starter is not used in the play phase of Cribbage , but is used later for making
    /// various card combinations that score points.
    ///
    fn after_discards_common_tests() -> (Scores, Scores, Cut, Player, Player) {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("AH2H3H4H5H6H", "AC2C3C4C5C6C")
            .as_discarding();
        
        let (player, opponent) = game0.player_1_2();

        let Game::Discarding(scores0, dealer0, hands0, _, deck0) = game0.clone()  else { panic!("Unexpected state") };
        let pone = game0.pone().clone();

        let player_hand0 = hands0[&player].clone();
        let player_discard = player_hand0.get(&[0, 1]);

        let opponent_hand0 = hands0[&opponent].clone();
        let opponent_discard = opponent_hand0.get(&[0, 1]);

        let game1 = game0.discard(player, &player_discard).ok().unwrap();
        let game1 = game1.discard(opponent, &opponent_discard).ok().unwrap();
        let Game::Playing(scores1, dealer1, hands1, play_state1, cut1, crib1) = game1 else { panic!("Unexpected state") };

        let hand1 = hands1[&player].clone();
        let opponent_hand1 = hands1[&opponent].clone();

        assert_eq!(dealer1, dealer0);
        assert!(hand1.contains_none(&player_discard));
        assert!(crib1.contains_all(&player_discard));
        assert!(opponent_hand1.contains_none(&opponent_discard));
        assert!(crib1.contains_all(&opponent_discard));
        assert!(deck0.contains(&cut1));
        assert_eq!(crib1.len(), CARDS_REQUIRED_IN_CRIB);
        assert_eq!(play_state1.legal_plays(pone).ok().unwrap(), hands1[&pone]);
        assert_eq!(play_state1.legal_plays(dealer0).err().unwrap(), Error::CannotPlay);
        assert_eq!(play_state1.pass_count(), 0);
        assert_eq!(play_state1.current_plays(), vec![]);
        assert_eq!(play_state1.previous_plays(), vec![]);

        (scores0, scores1, cut1, dealer1, pone)
    }

    #[test]
    fn start_the_play_after_discards() {
        let (scores0, scores1, cut, dealer, pone) = after_discards_common_tests();
        if cut.face() == Face::Jack {
            assert_eq!(scores0[&dealer].add(2), scores1[&dealer]);
            assert_eq!(scores0[&pone], scores1[&pone]);
        } else {
            assert_eq!(scores0, scores1)
        }
    }

    #[test]
    fn score_his_heels_when_jack_cut_after_discards() {
        loop {
            let (scores0, scores1, cut, dealer, pone) = after_discards_common_tests();
            if cut.face() == Face::Jack {
                assert_eq!(scores0[&dealer].add(2), scores1[&dealer]);
                assert_eq!(scores0[&pone], scores1[&pone]);
                break;
            }
        }
        assert!(true)
    }

    /// ## The Play
    ///
    /// After the starter is turned, the non-dealer lays one of his cards face up on the table. The
    /// dealer similarly exposes a card, then non-dealer again, and so on - the hands are exposed
    /// card by card, alternately except for a "Go," (Pass) as noted below. Each player keeps his
    /// cards separate from those of his opponent.
    /// 
    /// As each person plays, he announces a running total of pips reached by the addition of the
    /// last card to all those previously played. (Example: The non-dealer begins with a four,
    /// saying "Four." The dealer plays a nine, saying "Thirteen".) The kings, queens and jacks
    /// count 10 each; every other card counts its pip value (the ace counts one).
    #[test]
    fn accept_valid_play() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("9S", "4S")
            .with_cut("AS")
            .as_playing(1);
        let pone0 = game0.pone();
        let Game::Playing(scores0, dealer0, hands0, play_state0, cut0, crib0) = game0.clone() else { panic!("Unexpected state") };
        let dealer_hand0 = hands0[&dealer0].clone();

        assert_eq!(play_state0.legal_plays(dealer0).err().unwrap(), Error::CannotPlay);
        assert_eq!(play_state0.legal_plays(pone0).ok().unwrap().cards(), Hand::from("4S").cards());

        let game1 = game0.play(pone0, Card::from("4S")).ok().unwrap();
        let pone1 = game1.pone();
        let Game::Playing(scores1, dealer1, hands1, play_state1, cut1, crib1) = game1 else { panic!("Unexpected state") };
        let dealer_hand1 = hands1[&dealer1].clone();
        let pone_hand1 = hands1[&pone1].clone();

        assert_eq!(scores0, scores1); // TODO: Invalid test !
        assert_eq!(dealer0, dealer1);
        assert_eq!(dealer_hand0, dealer_hand1);
        assert_eq!(pone_hand1, Hand::default());
        assert_eq!(play_state1.legal_plays(dealer1).ok().unwrap(), dealer_hand1);
        assert_eq!(cut0, cut1);
        assert_eq!(crib0, crib1);
    }

    #[test]
    fn cannot_play_when_player_not_participating() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("9S", "4S")
            .with_cut("AS")
            .as_playing(1);

        let non_player = Player::new();
        let error = game0.play(non_player, Card::from("4S")).err().unwrap();
        assert_eq!(error, Error::InvalidPlayer(non_player));
    }

    #[test]
    fn cannot_play_when_unheld_card() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("9S", "4S")
            .with_cut("AS")
            .as_playing(1);
        let pone0 = game0.pone();
        let card = Card::from("9S");
        let error = game0.play(pone0, card).err().unwrap();
        assert_eq!(error, Error::InvalidCard(card));
    }

    #[test]
    fn cannot_play_when_not_their_turn() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("9S", "4S")
            .with_cut("AS")
            .as_playing(1);
        let dealer0 = game0.dealer();
        let card = Card::from("9S");
        let error = game0.play(dealer0, card).err().unwrap();
        assert_eq!(error, Error::CannotPlay);
    }

    #[test]
    fn cannot_play_when_play_exceeds_target() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("9S", "4S")
            .with_cut("AS")
            .with_current_plays(&vec![(0, "KH"), (0, "KC"), (0, "KD")])
            .as_playing(1);
        let pone0 = game0.pone();
        let Game::Playing(_, _, _, play_state0, _, _) = game0.clone() else { panic!("Unexpected state") };

        assert_eq!(play_state0.legal_plays(pone0).ok().unwrap().cards(), Hand::from("").cards());

        let error = game0.play(pone0, Card::from("4S")).err().unwrap();
        assert_eq!(error, Error::CannotPlayCard)
    }

    #[test]
    fn score_play_when_target_not_reached_mid_play() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("5S", "5H")
            .with_cut("AS")
            .with_current_plays(&vec![(0, "TH")])
            .as_playing(1);
        let pone = game0.pone();
        let Game::Playing(scores0, _, _, _, _, _) = game0.clone() else { panic!("Unexpected state") };
        let score0_pone = scores0[&pone];

        let game1 = game0.play(pone, Card::from("5H")).ok().unwrap();
        let Game::Playing(scores1, _, _, _, _, _) = game1 else { panic!("Unexpected state") };
        let score1_pone = scores1[&pone];
    
        assert_eq!(score1_pone, score0_pone.add(2));
    }

    #[test]
    fn score_play_when_target_not_reached_end_play() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("", "AH")
            .with_cut("QC")
            .with_current_plays(&vec![(0, "JH"), (0, "QH")])
            .with_previous_plays(&vec![(0, "9H"), (0, "7C"), (1, "6S"), (1, "2S"), (1, "KS")])
            .with_pass()
            .as_playing(1);

        let pone = game0.pone();
        let Game::Playing(scores0, dealer0, _, _, _, _) = game0.clone() else { panic!("Unexpected state") };
        let score0_pone = scores0[&pone];
        let score0_dealer = scores0[&dealer0];

        let game1 = game0.play(pone, Card::from("AH")).ok().unwrap();
        let Game::ScoringPone(scores1, dealer1, _, _, _) = game1.clone() else { panic!("Unexpected state") };
        let score1_pone = scores1[&pone];
        let score1_dealer = scores1[&dealer1];
    
        assert_eq!(score1_pone, score0_pone.add(1));
        assert_eq!(score1_dealer, score0_dealer);
    }

    #[test]
    fn score_play_when_target_not_reached_finished() {
        let game0 = Builder::new(2)
            .with_scores(0, 120)
            .with_hands("AH", "5H")
            .with_cut("QC")
            .with_current_plays(&vec![(0, "JH")])
            .with_previous_plays(&vec![(0, "9H"), (0, "7C"), (1, "6S"), (1, "2S"), (1, "KS")])
            .as_playing(1);
        let pone = game0.pone();
        let Game::Playing(scores0, _, _, _, _, _) = game0.clone() else { panic!("Unexpected state") };
        let score0_pone = scores0[&pone];

        let game1 = game0.play(pone, Card::from("5H")).ok().unwrap();
        let Game::Finished(scores1) = game1.clone() else { panic!("Unexpected state") };
        let score1_pone = scores1[&pone];
    
        assert_eq!(score1_pone, score0_pone.add(2));
    }

    #[test]
    fn score_play_when_target_reached_mid_play() {
        let card = Card::from("AH");
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("9H", "AH")
            .with_cut("KC")
            .with_current_plays(&vec![(0, "TH"), (0, "JH"), (0, "QH")])
            .with_previous_plays(&vec![(1, "2S"), (1, "QS"), (1, "6S")])
            .as_playing(1);
        let pone = game0.pone();
        let Game::Playing(scores0, dealer0, _, play_state0, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.play(pone, card).ok().unwrap();
        let Game::Playing(scores1, dealer1, hands1, play_state1, _, _) = game1.clone() else { panic!("Unexpected state") };

        assert_eq!(scores1[&pone], scores0[&pone].add(2));
        assert_eq!(dealer1, dealer0);
        assert!(!hands1[&pone].contains(&card));
        assert_eq!(play_state1.next_to_play(), dealer0);
        assert!(play_state1.current_plays().is_empty());
        for p in play_state0.current_plays().into_iter() {
            assert!(play_state1.previous_plays().contains(&p))
        }
    }

    #[test]
    fn score_play_when_target_reached_end_play() {
        let card = Card::from("AH");
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("", "AH")
            .with_cut("KC")
            .with_current_plays(&vec![(0, "TH"), (0, "JH"), (0, "QH")])
            .with_previous_plays(&vec![(0, "9H"), (1, "2S"), (1, "QS"), (1, "6S")])
            .as_playing(1);
        let pone = game0.pone();
        let Game::Playing(scores0, dealer0, _, _, cut0, crib0) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.play(pone, card).ok().unwrap();
        let Game::ScoringPone(scores1, dealer1, hands1, cut1, crib1) = game1.clone() else { panic!("Unexpected state") };

        assert_eq!(scores1[&pone], scores0[&pone].add(2));
        assert_eq!(dealer1, dealer0);
        assert!(!hands1[&pone].contains(&card));
        assert_eq!(cut1, cut0);
        assert_eq!(crib1, crib0);
    }

    #[test]
    fn score_play_when_target_reached_finished() {
        let game0 = Builder::new(2)
            .with_scores(0, 120)
            .with_hands("", "AH")
            .with_cut("KC")
            .with_current_plays(&vec![(0, "TH"), (0, "JH"), (0, "QH")])
            .with_previous_plays(&vec![(0, "9H"), (0, "4S"), (0, "5S"), (0, "6S")])
            .as_playing(1);
        let pone = game0.pone();
        let Game::Playing(scores0, dealer0, _, _, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.play(pone, Card::from("AH")).ok().unwrap();
        let Game::Finished(scores1) = game1.clone() else { panic!("Unexpected state") };

        assert_eq!(scores1[&pone], scores0[&pone].add(2));
        assert_eq!(scores1[&dealer0], scores0[&dealer0]);
    }

    #[test]
    fn score_play_when_plays_finished_and_game_finished() {

    }

  //     "plays complete - scoring pone hand wins" in dummyPlaying(
  //       poneCards = Seq(Card(Ten, Hearts)),
  //       poneScore = Score(0, 112),
  //       playeds = Seq(
  //         0 -> Card(Two, Hearts),
  //         0 -> Card(Four, Clubs),
  //         0 -> Card(Five, Clubs),
  //         0 -> Card(Two, Clubs),
  //         1 -> Card(Ten, Clubs),
  //         1 -> Card(Ace, Spades),
  //         1 -> Card(Ace, Diamonds)
  //       ),
  //       maybeCut = Some(Card(Ace, Hearts))
  //     ) { case playing0 @ Playing(_, _, dealer0, pone0, _, _, _) =>
  //       val finished1 = doPlayFor[Finished](pone0, Card(Ten, Hearts))(playing0)

  //       val Finished(scores) = finished1
  //       scores(pone0) should be(Score(113, 121))
  //       scores(dealer0) should be(Score(0, 0))
  //     }

  //     "plays complete - scoring dealer hand wins" in dummyPlaying(
  //       poneCards = Seq(Card(Ten, Hearts)),
  //       dealerScore = Score(0, 119),
  //       playeds = Seq(
  //         0 -> Card(Two, Hearts),
  //         0 -> Card(Four, Clubs),
  //         0 -> Card(Five, Clubs),
  //         0 -> Card(Two, Clubs),
  //         1 -> Card(Ten, Clubs),
  //         1 -> Card(Ace, Spades),
  //         1 -> Card(Ace, Diamonds)
  //       ),
  //       maybeCut = Some(Card(Ace, Hearts))
  //     ) { case playing0 @ Playing(_, _, dealer0, pone0, _, _, _) =>
  //       val finished1 = doPlayFor[Finished](pone0, Card(Ten, Hearts))(playing0)

  //       val Finished(scores) = finished1
  //       scores(pone0) should be(Score(1, 9))
  //       scores(dealer0) should be(Score(119, 121))
  //     }

  //     "plays complete - scoring crib wins" in dummyPlaying(
  //       poneCards = Seq(Card(Ten, Hearts)),
  //       dealerScore = Score(0, 117),
  //       cribCards = Seq(Card(Seven, Diamonds), Card(Eight, Hearts)),
  //       playeds = Seq(
  //         0 -> Card(Two, Hearts),
  //         0 -> Card(Four, Clubs),
  //         0 -> Card(Five, Clubs),
  //         0 -> Card(Two, Clubs),
  //         1 -> Card(Ten, Clubs),
  //         1 -> Card(Ace, Spades),
  //         1 -> Card(Ace, Diamonds)
  //       ),
  //       maybeCut = Some(Card(Ace, Hearts))
  //     ) { case playing0 @ Playing(_, _, dealer0, pone0, _, _, _) =>
  //       val finished1 = doPlayFor[Finished](pone0, Card(Ten, Hearts))(playing0)

  //       val Finished(scores) = finished1
  //       scores(pone0) should be(Score(1, 9))
  //       scores(dealer0) should be(Score(119, 121))
  //     }
  //   }

    /// ## The Go
    ///
    /// During play, the running total of cards may never be carried beyond 31. If a player cannot
    /// add another card without exceeding 31, he or she says "Go" and the opponent pegs 1. After
    /// gaining the Go, the opponent must first lay down any additional cards he can without
    /// exceeding 31. Besides the point for Go, he may then score any additional points that can be
    /// made through pairs and runs (described later). If a player reaches exactly 31, he pegs two
    /// instead of one for Go.
    ///
    /// The player who called Go leads for the next series of plays, with the count starting at
    /// zero. The lead may not be combined with any cards previously played to form a scoring
    /// combination; the Go has interrupted the sequence.
    ///
    /// The person who plays the last card pegs one for Go, plus one extra if the card brings the
    /// count to exactly 31. The dealer is sure to peg at least one point in every hand, for he will
    /// have a Go on the last card if not earlier.
    
    #[test]
    fn accept_pass_when_pone_has_no_valid_card() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("AS")
            .with_hands("AH", "KH")
            .with_current_plays(&vec![(0, "TH"), (0, "JH"), (0, "QH")])
            .as_playing(1);
        let pone = game0.pone();
        let Game::Playing(scores0, dealer0, hands0, play_state0, _, _) = game0.clone() else { panic!("Unexpected state") };
        let game1 = game0.pass(pone);
        let game1 = game1.ok().unwrap();
        let Game::Playing(scores1, dealer1, hands1, play_state1, _, _) = game1.clone() else { panic!("Unexpected state") };

        assert_eq!(scores1, scores0);
        assert_eq!(dealer1, dealer0);
        assert_eq!(hands1, hands0);
        assert_eq!(play_state1.next_to_play(), dealer0);
        assert_eq!(play_state1.pass_count(), 1);
        assert_eq!(play_state1.current_plays(), play_state0.current_plays());
        assert_eq!(play_state1.previous_plays(), play_state0.previous_plays());
    }

    #[test]
    fn accept_pass_when_dealer_has_no_valid_card() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("AS")
            .with_hands("KH", "KS")
            .with_current_plays(&vec![(0, "TH"), (0, "JH"), (1, "QH")])
            .as_playing(1);
        let pone = game0.pone();
        let Game::Playing(scores0, dealer0, hands0, play_state0, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.pass(pone).ok().unwrap();
        let game2 = game1.pass(dealer0).ok().unwrap();

        let Game::Playing(scores2, dealer2, hands2, play_state2, _, _) = game2.clone() else { panic!("Unexpected state") };

        assert_eq!(scores2[&pone], scores0[&pone]);
        assert_eq!(scores2[&dealer2], scores0[&dealer0].add(1));
        assert_eq!(dealer2, dealer0);
        assert_eq!(hands2, hands0);
        assert_eq!(play_state2.next_to_play(), pone);
        assert_eq!(play_state2.pass_count(), 2);
        assert!(play_state2.current_plays().is_empty());
        for p in play_state0.current_plays().into_iter() {
            assert!(play_state2.previous_plays().contains(&p))
        }
    }

    #[test]
    fn cannot_pass_when_player_not_participating() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("AS")
            .with_hands("KH", "KS")
            .with_current_plays(&vec![(0, "TH"), (0, "JH"), (1, "QH")])
            .as_playing(1);

        let non_player = Player::new();
        let error = game0.pass(non_player).err().unwrap();
        assert_eq!(error, Error::InvalidPlayer(non_player));
    }

    #[test]
    fn cannot_pass_when_valid_card_held() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("AS")
            .with_hands("AH", "AS")
            .with_current_plays(&vec![(0, "TH"), (0, "JH"), (0, "8H")])
            .as_playing(1);
        let pone = game0.pone();

        let error = game0.pass(pone).err().unwrap();
        assert_eq!(error, Error::CannotPass);
    }

    #[test]
    fn score_pass_when_both_players_passed_playing() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("AS")
            .with_hands("KH", "KS")
            .with_current_plays(&vec![(0, "TH"), (0, "JH"), (1, "QH")])
            .as_playing(1);
        let pone = game0.pone();
        let Game::Playing(scores0, dealer0, hands0, play_state0, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.pass(pone).ok().unwrap();
        let game2 = game1.pass(dealer0).ok().unwrap();

        let Game::Playing(scores2, dealer2, _, _, _, _) = game2.clone() else { panic!("Unexpected state") };
        assert_eq!(scores2[&pone], scores0[&pone]);
        assert_eq!(scores2[&dealer2], scores0[&dealer0].add(1));
    }

    #[test]
    fn score_pass_when_both_players_passed_finished() {
        let game0 = Builder::new(2)
            .with_scores(120, 0)
            .with_cut("AS")
            .with_hands("KH", "KS")
            .with_current_plays(&vec![(0, "TH"), (0, "JH"), (1, "QH")])
            .as_playing(1);
        let pone = game0.pone();
        let Game::Playing(scores0, dealer0, _, _, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.pass(pone).ok().unwrap();
        let game2 = game1.pass(dealer0).ok().unwrap();

        let Game::Finished(scores2) = game2.clone() else { panic!("Unexpected state") };
        assert_eq!(scores2[&pone], scores0[&pone]);
        assert_eq!(scores2[&dealer0], scores0[&dealer0].add(1));
    }

    /// ## Pegging
    /// 
    /// The object in play is to score points by pegging. In addition to a Go, a player may score
    /// for the following combinations:
    ///
    ///   - Fifteen: For adding a card that makes the total 15 Peg 2
    ///   - Pair: For adding a card of the same rank as the card just played Peg 2 (Note that face
    ///     cards pair only by actual rank: jack with jack, but not jack with queen.)
    ///   - Triplet: For adding the third card of the same rank. Peg 6
    ///   - Four: (also called "Double Pair" or "Double Pair Royal") For adding the fourth card of
    ///     the same rank Peg 12
    ///   - Run (Sequence): For adding a card that forms, with those just played:
    ///     - For a sequence of three Peg 3
    ///     - For a sequence of four. Peg 4
    ///     - For a sequence of five. Peg 5
    ///     - (Peg one point more for each extra card of a sequence. Note that runs are independent
    ///       of suits, but go strictly by rank; to illustrate: 9, 10, J, or J, 9, 10 is a run but
    ///       9, 10, Q is not)
    
    #[test]
    fn score_play_points_for_fifteens() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("AS")
            .with_hands("KH", "8D")
            .with_current_plays(&vec![(0, "7D")])
            .as_playing(1);
        let pone = game0.pone();
        let Game::Playing(scores0, dealer0, _, _, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.play(pone, Card::from("8D")).ok().unwrap();
        let Game::Playing(scores1, dealer1, _, _, _, _) = game1.clone() else { panic!("Unexpected state") };

        assert_eq!(dealer1, dealer0);
        assert_eq!(scores1[&dealer1], scores0[&dealer0]);
        assert_eq!(scores1[&pone], scores0[&pone].add(2));
    }

    #[test]
    fn score_play_points_for_pair() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("AS")
            .with_hands("KH", "8D")
            .with_current_plays(&vec![(0, "8S")])
            .as_playing(1);
        let pone = game0.pone();
        let Game::Playing(scores0, dealer0, _, _, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.play(pone, Card::from("8D")).ok().unwrap();
        let Game::Playing(scores1, dealer1, _, _, _, _) = game1.clone() else { panic!("Unexpected state") };

        assert_eq!(dealer1, dealer0);
        assert_eq!(scores1[&dealer1], scores0[&dealer0]);
        assert_eq!(scores1[&pone], scores0[&pone].add(2));
    }

    #[test]
    fn score_play_points_for_triplet() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("AS")
            .with_hands("KH", "8D")
            .with_current_plays(&vec![(1, "8C"), (0, "8S")])
            .as_playing(1);
        let pone = game0.pone();
        let Game::Playing(scores0, dealer0, _, _, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.play(pone, Card::from("8D")).ok().unwrap();
        let Game::Playing(scores1, dealer1, _, _, _, _) = game1.clone() else { panic!("Unexpected state") };

        assert_eq!(dealer1, dealer0);
        assert_eq!(scores1[&dealer1], scores0[&dealer0]);
        assert_eq!(scores1[&pone], scores0[&pone].add(6));
    }

    #[test]
    fn score_play_points_for_quartet() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("AS")
            .with_hands("KH", "7D")
            .with_current_plays(&vec![(1, "7C"), (0, "7S"), (0, "7H")])
            .as_playing(1);
        let pone = game0.pone();
        let Game::Playing(scores0, dealer0, _, _, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.play(pone, Card::from("7D")).ok().unwrap();
        let Game::Playing(scores1, dealer1, _, _, _, _) = game1.clone() else { panic!("Unexpected state") };

        assert_eq!(dealer1, dealer0);
        assert_eq!(scores1[&dealer1], scores0[&dealer0]);
        assert_eq!(scores1[&pone], scores0[&pone].add(12));
    }

    #[test]
    fn score_play_points_for_run() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("AS")
            .with_hands("KH", "AS")
            .with_current_plays(&vec![(1, "2D"), (0, "3H")])
            .as_playing(1);
        let pone = game0.pone();
        let Game::Playing(scores0, dealer0, _, _, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.play(pone, Card::from("AS")).ok().unwrap();
        let Game::Playing(scores1, dealer1, _, _, _, _) = game1.clone() else { panic!("Unexpected state") };

        assert_eq!(dealer1, dealer0);
        assert_eq!(scores1[&dealer1], scores0[&dealer0]);
        assert_eq!(scores1[&pone], scores0[&pone].add(3));
    }

    /// It is important to keep track of the order in which cards are played to determine whether
    /// what looks like a sequence or a run has been interrupted by a "foreign card." Example:
    /// Cards are played in this order: 8, 7, 7, 6. The dealer pegs 2 for 15, and the opponent
    /// pegs 2 for pair, but the dealer cannot peg for run because of the extra seven (foreign
    /// card) that has been played. Example: Cards are played in this order: 9, 6, 8, 7. The
    /// dealer pegs 2 for fifteen when he plays the six and pegs 4 for run when he plays the seven
    /// (the 6, 7, 8, 9 sequence). The cards were not played in sequential order, but they form a
    /// true run with no foreign card.

    #[test]
    fn score_play_points_for_run_edge_case_1() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("AS")
            .with_hands("5H7H6H", "8S7S")
            .as_playing(1);
        let pone = game0.pone();
        let Game::Playing(scores0, dealer0, _, _, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.play(pone, Card::from("8S")).ok().unwrap();
        let game1 = game1.play(dealer0, Card::from("7H")).ok().unwrap();
        let game1 = game1.play(pone, Card::from("7S")).ok().unwrap();
        let game1 = game1.play(dealer0, Card::from("6H")).ok().unwrap();

        let Game::Playing(scores1, dealer1, _, _, _, _) = game1.clone() else { panic!("Unexpected state") };

        assert_eq!(dealer1, dealer0);
        assert_eq!(scores1[&dealer1], scores0[&dealer0].add(2));
        assert_eq!(scores1[&pone], scores0[&pone].add(2));
    }

    #[test]
    fn score_play_points_for_run_edge_case_2() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("AS")
            .with_hands("5H7H6H", "9S8S")
            .as_playing(1);
        let pone = game0.pone();
        let Game::Playing(scores0, dealer0, _, _, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.play(pone, Card::from("9S")).ok().unwrap();
        let game1 = game1.play(dealer0, Card::from("6H")).ok().unwrap();
        let game1 = game1.play(pone, Card::from("8S")).ok().unwrap();
        let game1 = game1.play(dealer0, Card::from("7H")).ok().unwrap();

        let Game::Playing(scores1, dealer1, _, _, _, _) = game1.clone() else { panic!("Unexpected state") };

        assert_eq!(dealer1, dealer0);
        assert_eq!(scores1[&dealer1], scores0[&dealer0].add(2).add(4));
        assert_eq!(scores1[&pone], scores0[&pone]);
    }

  //   /** ## Counting the Hands
  //     *
  //     * When play ends, the three hands are counted in order: non-dealer's hand (first), dealer's
  //     * hand (second), and then the crib (third). This order is important because, toward the end of
  //     * a game, the non-dealer may "count out" and win before the dealer has a chance to count, even
  //     * though the dealer's total would have exceeded that of the opponent. The starter is
  //     * considered to be a part of each hand, so that all hands in counting comprise five cards. The
  //     * basic scoring formations are as follows:
  //     *
  //     * Combination Counts
  //     *   - Fifteen. Each combination of cards that totals 15 2
  //     *   - Pair. Each pair of cards of the same rank 2
  //     *   - Run. Each combination of three or more 1 cards in sequence (for each card in the
  //     *     sequence)
  //     *   - Flush.
  //     *     - Four cards of the same suit in hand 4 (excluding the crib, and the starter)
  //     *     - Four cards in hand or crib of the same 5 suit as the starter. (There is no count for
  //     *       four-flush in the crib that is not of same suit as the starter)
  //     *   - His Nobs. Jack of the same suit as starter in hand or crib 1
  //     */
  //   "score cards" when {
  //     "fifteens" in dummyPlaying(
  //       poneCards = Seq(Card(Ten, Hearts)),
  //       poneScore = Score(0, -1), // Adjust for final Play
  //       playeds = Seq(
  //         0 -> Card(Seven, Hearts),
  //         0 -> Card(Eight, Clubs),
  //         0 -> Card(Ace, Clubs),
  //         0 -> Card(Two, Clubs),
  //         1 -> Card(Jack, Clubs),
  //         1 -> Card(King, Spades),
  //         1 -> Card(Five, Hearts)
  //       ),
  //       maybeCut = Some(Card(Four, Hearts))
  //     ) { case playing0 @ Playing(_, _, dealer0, pone0, _, _, _) =>
  //       val discarding1 = doPlayFor[Discarding](pone0, Card(Ten, Hearts))(playing0)

  //       val Discarding(_, scores, _, _, _, _) = discarding1
  //       scores(pone0) should be(Score(0, 6))
  //       scores(dealer0) should be(Score(0, 4))
  //     }

  //     "pairs" in dummyPlaying(
  //       poneCards = Seq(Card(Ten, Hearts)),
  //       poneScore = Score(0, -1), // Adjust for final Play
  //       playeds = Seq(
  //         0 -> Card(Two, Hearts),
  //         0 -> Card(Four, Clubs),
  //         0 -> Card(Five, Clubs),
  //         0 -> Card(Two, Clubs),
  //         1 -> Card(Ten, Clubs),
  //         1 -> Card(Ace, Spades),
  //         1 -> Card(Ace, Diamonds)
  //       ),
  //       maybeCut = Some(Card(Ace, Hearts))
  //     ) { case playing0 @ Playing(_, _, dealer0, pone0, _, _, _) =>
  //       val discarding1 = doPlayFor[Discarding](pone0, Card(Ten, Hearts))(playing0)

  //       val Discarding(_, scores, _, _, _, _) = discarding1
  //       scores(pone0) should be(Score(0, 8))
  //       scores(dealer0) should be(Score(0, 2))
  //     }

  //     "runs" in dummyPlaying(
  //       poneCards = Seq(Card(Five, Hearts)),
  //       poneScore = Score(0, -1), // Adjust for final Play
  //       playeds = Seq(
  //         0 -> Card(Jack, Diamonds),
  //         0 -> Card(Queen, Clubs),
  //         0 -> Card(King, Clubs),
  //         0 -> Card(Two, Clubs),
  //         1 -> Card(Three, Clubs),
  //         1 -> Card(Three, Spades),
  //         1 -> Card(Two, Diamonds)
  //       ),
  //       maybeCut = Some(Card(Ace, Hearts))
  //     ) { case playing0 @ Playing(_, _, dealer0, pone0, _, _, _) =>
  //       val discarding1 = doPlayFor[Discarding](pone0, Card(Five, Hearts))(playing0)

  //       val Discarding(_, scores, _, _, _, _) = discarding1
  //       scores(pone0) should be(Score(0, 8))
  //       scores(dealer0) should be(Score(0, 3))
  //     }

  //     "flush - hand" in dummyPlaying(
  //       poneCards = Seq(Card(King, Hearts)),
  //       poneScore = Score(0, -1), // Adjust for final Play
  //       playeds = Seq(
  //         0 -> Card(King, Diamonds),
  //         0 -> Card(Three, Diamonds),
  //         0 -> Card(Eight, Diamonds),
  //         0 -> Card(Queen, Diamonds),
  //         1 -> Card(Three, Hearts),
  //         1 -> Card(Eight, Hearts),
  //         1 -> Card(Queen, Hearts)
  //       ),
  //       maybeCut = Some(Card(Ace, Hearts))
  //     ) { case playing0 @ Playing(_, _, dealer0, pone0, _, _, _) =>
  //       val discarding1 = doPlayFor[Discarding](pone0, Card(King, Hearts))(playing0)

  //       val Discarding(_, scores, _, _, _, _) = discarding1
  //       scores(pone0) should be(Score(0, 5))
  //       scores(dealer0) should be(Score(0, 4))
  //     }

  //     "flush 4 - crib" in dummyPlaying(
  //       poneCards = Seq(Card(King, Clubs)),
  //       poneScore = Score(0, -1), // Adjust for final Play
  //       playeds = Seq(
  //         0 -> Card(King, Diamonds),
  //         0 -> Card(Three, Clubs),
  //         0 -> Card(Eight, Diamonds),
  //         0 -> Card(Queen, Diamonds),
  //         1 -> Card(Three, Spades),
  //         1 -> Card(Eight, Spades),
  //         1 -> Card(Queen, Clubs)
  //       ),
  //       cribCards = Seq(
  //         Card(King, Hearts),
  //         Card(Three, Hearts),
  //         Card(Eight, Hearts),
  //         Card(Queen, Hearts)
  //       ),
  //       maybeCut = Some(Card(Ace, Clubs))
  //     ) { case playing0 @ Playing(_, _, dealer0, pone0, _, _, _) =>
  //       val discarding1 = doPlayFor[Discarding](pone0, Card(King, Clubs))(playing0)

  //       val Discarding(_, scores, _, _, _, _) = discarding1
  //       scores(pone0) should be(Score(-1, 0))
  //       scores(dealer0) should be(Score(0, 0))
  //     }

  //     "flush 5 - crib" in dummyPlaying(
  //       poneCards = Seq(Card(King, Clubs)),
  //       poneScore = Score(0, -1), // Adjust for final Play
  //       playeds = Seq(
  //         0 -> Card(King, Diamonds),
  //         0 -> Card(Three, Clubs),
  //         0 -> Card(Eight, Diamonds),
  //         0 -> Card(Queen, Diamonds),
  //         1 -> Card(Three, Spades),
  //         1 -> Card(Eight, Spades),
  //         1 -> Card(Queen, Clubs)
  //       ),
  //       cribCards = Seq(
  //         Card(King, Hearts),
  //         Card(Three, Hearts),
  //         Card(Eight, Hearts),
  //         Card(Queen, Hearts)
  //       ),
  //       maybeCut = Some(Card(Ace, Hearts))
  //     ) { case playing0 @ Playing(_, _, dealer0, pone0, _, _, _) =>
  //       val discarding1 = doPlayFor[Discarding](pone0, Card(King, Clubs))(playing0)

  //       val Discarding(_, scores, _, _, _, _) = discarding1
  //       scores(pone0) should be(Score(-1, 0))
  //       scores(dealer0) should be(Score(0, 5))
  //     }
  //   }

  //   /* ## Combinations
  //    *
  //    * In the above table, the word combination is used in the strict technical sense. Each and
  //    * every combination of two cards that make a pair, of two or more cards that make 15, or of
  //    * three or more cards that make a run, count separately.
  //    *
  //    * Example: A hand (including the starter) comprised of 8, 7, 7, 6, 2 scores 8 points for four
  //    * combinations that total 15: the 8 with one 7, and the 8 with the other 7; the 6, 2 with each
  //    * of the two 7s. The same hand also scores 2 for a pair, and 6 for two runs of three (8, 7, 6
  //    * using each of the two 7s). The total score is 16. An experienced player computes the hand
  //    * thus: "Fifteen 2, fifteen 4, fifteen 6, fifteen 8, and 8 for double run is 16."
  //    *
  //    * Note that the ace is always low and cannot form a sequence with a king. Further, a flush
  //    * cannot happen during the play of the cards; it occurs only when the hands and the crib are
  //    * counted.
  //    *
  //    * Certain basic formulations should be learned to facilitate counting. For pairs and runs
  //    * alone:
  //    *
  //    * A. A triplet counts 6. A. Four of a kind counts 12. A. A run of three, with one card
  //    * duplicated (double run) counts 8. A. A run of four, with one card duplicated, counts 10. A.
  //    * A run of three, with one card triplicated (triple run), counts 15. A. A run of three, with
  //    * two different cards duplicated, counts 16.
  //    *
  //    * ### A PERFECT 29!
  //    *
  //    * The highest possible score for combinations in a single Cribbage deal is 29, and it may
  //    * occur only once in a Cribbage fan's lifetime -in fact, experts say that a 29 is probably as
  //    * rare as a hole-in-one in golf. To make this amazing score, a player must have a five as the
  //    * starter (upcard) and the other three fives plus the jack of the same suit as the starter -
  //    * His Nobs: 1 point - in his hand. The double pair royal (four 5s) peg another 12 points; the
  //    * various fives used to hit 15 can be done four ways for 8 points; and the jack plus a 5 to
  //    * hit 15 can also be done four ways for 8 points. Total = 29 points.
  //    */
  //   "score combinations" when {
  //     "six sevens and eights" in dummyPlaying(
  //       poneCards = Seq(Card(Ace, Hearts)),
  //       playeds = Seq(
  //         0 -> Card(Eight, Hearts),
  //         0 -> Card(Seven, Hearts),
  //         0 -> Card(Seven, Clubs),
  //         0 -> Card(Six, Clubs),
  //         1 -> Card(Ten, Diamonds),
  //         1 -> Card(Jack, Diamonds),
  //         1 -> Card(King, Diamonds)
  //       )
  //     )

  //     "perfect" in dummyPlaying(
  //       poneCards = Seq(Card(Ace, Hearts)),
  //       playeds = Seq(
  //         0 -> Card(Jack, Hearts),
  //         0 -> Card(Five, Clubs),
  //         0 -> Card(Five, Diamonds),
  //         0 -> Card(Five, Spades),
  //         1 -> Card(Ten, Diamonds),
  //         1 -> Card(Jack, Diamonds),
  //         1 -> Card(King, Diamonds)
  //       ),
  //       maybeCut = Some(Card(Five, Hearts))
  //     ) { case playing0 @ Playing(_, _, _, pone0, _, _, _) =>
  //       val discarding1 = doPlayFor[Discarding](pone0, Card(Ace, Hearts))(playing0)

  //       val Discarding(_, scores1, _, _, pone1, _) = discarding1
  //       scores1(pone1) should be(Score(0, 29))
  //     }
  //   }

  //   /** ## Miscellaneous
  //     *
  //     * The following list includes many of the hands that may give the beginner some difficulty in
  //     * counting. Note that no hand can make a count of 19, 25, 26, or 27. (In the chart below J
  //     * stands for His Nobs, the jack of the same suit as the starter.
  //     *
  //     * ### Muggins (optional) - not implemented.
  //     *
  //     * Each player must count his hand (and crib) aloud and announce the total. If he overlooks any
  //     * score, the opponent may say "Muggins" and then score the overlooked points for himself. For
  //     * experienced players, the Muggins rule is always in effect and adds even more suspense to the
  //     * game.
  //     *
  //     * ## Game
  //     *
  //     * Game may be fixed at either 121 points or 61 points. The play ends the moment either player
  //     * reaches the agreed total, whether by pegging or counting one's hand. If the non-dealer "goes
  //     * out" by the count of his hand, the game immediately ends and the dealer may not score either
  //     * his hand or the crib.
  //     *
  //     * If a player wins the game before the loser has passed the halfway mark (did not reach 31 in
  //     * a game of 61, or 61 in a game of 121), the loser is "lurched," and the winner scores two
  //     * games instead of one. A popular variation of games played to 121, is a "skunk" (double game)
  //     * for the winner if the losing player fails to pass the three-quarter mark - 91 points or more
  //     * \- and it is a "double skunk" (quadruple game) if the loser fails to pass the halfway mark
  //     * (61 or more points).
  //     *
  //     * ## The Cribbage Board
  //     *
  //     * The Cribbage board (see illustration) has four rows of 30 holes each, divided into two pairs
  //     * of rows by a central panel. There are usually four (or two) additional holes near one end,
  //     * called "game holes." With the board come four pegs, usually in two contrasting colors. Note:
  //     * There are also continuous track Cribbage boards available which, as the name implies, have
  //     * one continuous line of 121 holes for each player.
  //     *
  //     * The board is placed to one side between the two players, and each player takes two pegs of
  //     * the same color. (The pegs are placed in the game holes until the game begins.) Each time a
  //     * player scores, he advances a peg along a row on his side of the board, counting one hole per
  //     * point. Two pegs are used, and the rearmost peg jumps over the first peg to show the first
  //     * increment in score. After another increase in score, the peg behind jumps over the peg in
  //     * front to the appropriate hole to show the player's new score, and so on (see diagram next
  //     * page). The custom is to "go down" (away from the game holes) on the outer rows and "come up"
  //     * on the inner rows. A game of 61 is "once around" and a game of 121 is "twice around." As
  //     * noted previously, continuous line Cribbage boards are available.
  //     *
  //     * If a Cribbage board is not available, each player may use a piece of paper or cardboard,
  //     * marked thus:
  //     *
  //     *   - Units 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
  //     *   - Tens 10, 20, 30, 40, 50, 60
  //     *
  //     * Two small markers, such as small coins or buttons, can substitute for pegs for counting in
  //     * each row.
  //     *
  //     * ## Strategy
  //     *
  //     * ### The Crib.
  //     *
  //     * If the dealer is discarding for the crib, he should “salt” it with the best possible cards,
  //     * but at the same time retain good cards in his hand that can be used for high scoring.
  //     * Conversely, for the non-dealer, it is best to lay out cards that will be the least
  //     * advantageous for the dealer. Laying out a five would be the worst choice, for the dealer
  //     * could use it to make 15 with any one of the ten-cards (10, J, Q, K). Laying out a pair is
  //     * usually a poor choice too, and the same goes for sequential cards, such as putting both a
  //     * six and seven in the crib. The ace and king tend to be good cards to put in the crib because
  //     * it is harder to use them in a run.
  //     *
  //     * ### The Play
  //     *
  //     * As expected, the five makes for the worst lead in that there are so many ten-cards that the
  //     * opponent can use to make a 15. Leading from a pair is a good idea, for even if the opponent
  //     * makes a pair, the leader can play the other matching card from his hand and collect for a
  //     * pair royal. Leading an ace or deuce is not a good idea, for these cards should be saved
  //     * until later to help make a 15, a Go, or a 31. The safest lead is a four because this card
  //     * cannot be used to make a 15 at the opponent’s very next turn. Finally, when the opponent
  //     * leads a card that can either be paired or make 15, the latter choice is preferred.
  //     *
  //     * During the play, it is advisable not to try to make a count of 21, for the opponent can then
  //     * play one of the many 10-cards and make 31 to gain two points.
  //     */
  // }

}