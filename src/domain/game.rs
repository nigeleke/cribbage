use crate::domain::prelude::*;
use crate::domain::result::Result;

use leptos::*;
use serde::{Serialize, Deserialize};

use std::collections::{HashMap, HashSet};
use std::fmt::Display;

/// The game state, waiting for opponent, discarding, playing, scoring, finished.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Game {
    Starting(HashMap<Player, Card>, Deck),
    Discarding(HashMap<Player, Score>, Player, HashMap<Player, Hand>, Crib, Deck),
//     Playing(MyState, OpponentState, Play, Cut, Crib),
//     ScoringPoneCards(MyState, OpponentState, Cut, Crib),
//     ScoringDealerCards(MyState, OpponentState, Cut, Crib),
//     ScoringCrib(MyState, OpponentState, Cut, Crib),
//     Finished(MyScore, OpponentScore),
}

impl Game {
    pub fn new(players: &HashSet<Player>) -> Result<Game> {
        if players.len() > super::NUMBER_OF_PLAYERS_IN_GAME {
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
        }
    }

    pub fn deck(&self) -> Deck {
        match self {
            Game::Starting(_, deck) => deck.clone(),
            Game::Discarding(_, _, _, _, deck) => deck.clone(),
        }
    }

    pub fn start(&self) -> Result<Self> {
        match self {
            Game::Starting(cuts, _) => {
                let players = self.players();
                verify::players(&players)?;
                verify::different_cuts(cuts)?;
                let scores: HashMap<Player, Score> = HashMap::from_iter(players.iter().map(|&p| (p, Score::new())));
                let mut cuts = cuts.iter();
                let Some((player1, cut1)) = cuts.next() else { unreachable!() };
                let Some((player2, cut2)) = cuts.next() else { unreachable!() };
                let dealer = if cut1.rank() < cut2.rank() { player1 } else { player2 };
                let deck = Deck::shuffled_pack();
                let (hands, deck) = deck.deal(&players);
                let crib = Crib::new();
                Ok(Game::Discarding(scores, dealer.clone(), hands, crib, deck))
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
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Game::Starting(cuts, deck) => write!(f, "Starting(Cuts({}), {})", format_hashmap(cuts), deck),
            _ => write!(f, "{:?}", self),
        }
    }
}

fn format_hashmap<T: Display>(map: &HashMap<Player, T>) -> String {
    map.iter().map(|(k, v)| format!("{} -> {}", k, v)).collect::<Vec<_>>().join(", ")
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

}


/// # [Cribbage Rules](https://www.officialgamerules.org/cribbage)
#[cfg(test)]
mod test {
    use super::*;

    /// ## Number of Players
    ///
    /// Two or three people can play. Or four people can play two against two as partners. But
    /// Cribbage is basically best played by two people, and the rules that follow are for that
    /// number.
    ///
    #[test]
    fn play_with_zero_one_or_two_players() {
    let players: HashSet<Player> = HashSet::from_iter(vec![Player::new(), Player::new()].into_iter());
    for n in 0..=2 {
        let players = HashSet::from_iter(players.clone().into_iter().take(n));
        let game = Game::new(&players).ok().unwrap();
        assert_eq!(game.players().len(), n);
        for player in players.into_iter() {
            assert!(game.players().contains(&player))
        }
    }
    }

    #[test]
    fn fail_to_play_with_more_than_two_players() {
        let players: HashSet<Player> = HashSet::from_iter(vec![Player::new(), Player::new(), Player::new()].into_iter());
        let error = Game::new(&players).err().unwrap();
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
        let game = Game::new(&HashSet::new()).ok().unwrap();
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
        let player1 = Player::new();
        let player2 = Player::new();
        let players: HashSet<Player> = HashSet::from_iter(vec![player1, player2].into_iter());

        loop {
            let game = Game::new(&players).ok().unwrap();
            let Game::Starting(ref cuts, _) = game else { panic!("Unexpected state") };
            let card1 = cuts.get(&player1).unwrap();
            let card2 = cuts.get(&player2).unwrap();

            if card1.rank() != card2.rank() {
                let game = game.start().ok().unwrap();
                let Game::Discarding(_, dealer, _, _, _) = game else { panic!("Unexpected state") };
                if card1.rank() < card2.rank() {
                    assert_eq!(dealer, player1)
                } else {
                    assert_eq!(dealer, player2)
                }
                break;
            };
        }

        assert!(true)
    }

    #[test]
    fn fail_to_start_game_if_cuts_are_the_same_value() {
        let player1 = Player::new();
        let player2 = Player::new();
        let players: HashSet<Player> = HashSet::from_iter(vec![player1, player2].into_iter());

        loop {
            let game = Game::new(&players).ok().unwrap();
            let Game::Starting(ref cuts, _) = game else { panic!("Unexpected state") };
            let card1 = cuts.get(&player1).unwrap();
            let card2 = cuts.get(&player2).unwrap();

            if card1.rank() == card2.rank() {
                let error = game.start().err().unwrap();
                assert_eq!(error, Error::CutForStartUndecided);
                break;
            };
        }

        assert!(true)
    }

    #[test]
    fn fail_to_start_game_if_not_enough_players() {
        let player1 = Player::new();
        let players: HashSet<Player> = HashSet::from_iter(vec![player1].into_iter());

        let game = Game::new(&players).ok().unwrap();
        let error = game.start().err().unwrap();
        assert_eq!(error, Error::NotEnoughPlayers);
    }

    #[test]
    fn redraw_if_cuts_are_same_value() {
        let player1 = Player::new();
        let player2 = Player::new();
        let players: HashSet<Player> = HashSet::from_iter(vec![player1, player2].into_iter());

        loop {
            let game = Game::new(&players).ok().unwrap();
            let Game::Starting(ref cuts, _) = game else { panic!("Unexpected state") };
            let card1 = cuts.get(&player1).unwrap();
            let card2 = cuts.get(&player2).unwrap();

            if card1.rank() == card2.rank() {
                let game = game.redraw().ok().unwrap();
                let Game::Starting(ref _cuts, _) = game else { panic!("Unexpected state") };
                break;
            };
        }

        assert!(true)
    }

    #[test]
    fn fail_to_redraw_if_cuts_are_not_the_same_value() {
        let player1 = Player::new();
        let player2 = Player::new();
        let players: HashSet<Player> = HashSet::from_iter(vec![player1, player2].into_iter());

        loop {
            let game = Game::new(&players).ok().unwrap();
            let Game::Starting(ref cuts, _) = game else { panic!("Unexpected state") };
            let card1 = cuts.get(&player1).unwrap();
            let card2 = cuts.get(&player2).unwrap();

            if card1.rank() != card2.rank() {
                let error = game.redraw().err().unwrap();
                assert_eq!(error, Error::CutForStartDecided);
                break;
            };
        }

        assert!(true)
    }

  //   /** ## The Deal
  //     *
  //     * The dealer distributes six cards face down to his opponent and himself, beginning with the
  //     * opponent.
  //     */
  //   def dummyDiscarding(
  //       dealerScore: Score = Score.zero,
  //       poneScore: Score = Score.zero,
  //       maybeCut: Option[Card] = None
  //   )(f: Discarding => Unit) =
  //     val players       = (1 to 2).map(_ => Player.newPlayer).toSet
  //     val draws         = players.map((_, Card(Jack, Hearts))).toMap
  //     val draw          = Decided(draws, players.head, players.last)
  //     val discarding    = Cribbage.dealFirstHand(draw)
  //     val dealer        = discarding.dealer
  //     val pone          = discarding.pone
  //     val updatedScores = Map(dealer -> dealerScore, pone -> poneScore)
  //     val updatedDeck   = maybeCut.map(c => Cards.deckOf(Seq(c))).getOrElse(discarding.deck)
  //     f(discarding.copy(deck = updatedDeck, scores = updatedScores))

  //   "deal" when {
  //     "draw decided" in dummyDiscarding() {
  //       case discarding @ Discarding(deck, scores, hands, dealer, pone, crib) =>
  //         val players = Set(dealer, pone)
  //         deck.size should be(40)
  //         scores.keySet should be(players)
  //         scores.foreach(_._2 should be(Score.zero))
  //         hands.keySet should be(players)
  //         hands.foreach(_._2.size should be(CardsDealtPerHand))
  //         crib.toSeq should be(empty)
  //         deck.toSeq ++
  //           hands(dealer).toSeq ++
  //           hands(pone).toSeq should contain theSameElementsAs (Cards.fullDeck.toSeq)
  //     }

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

  //   /** ## Object of the Game
  //     *
  //     * The goal is to be the first player to score 121 points. (Some games are to 61 points.)
  //     * Players earn points during play and for making various card combinations.
  //     */

  //   /** ## The Crib
  //     *
  //     * Each player looks at his six cards and "lays away" (discards) two of them face down to
  //     * reduce the hand to four. The four cards laid away together constitute "the crib". The crib
  //     * belongs to the dealer, but these cards are not exposed or used until after the hands have
  //     * been played.
  //     */
  //   def doDiscardsReturning[A](player: Player, discards: Seq[Card])(discarding: Discarding): A =
  //     Cribbage.discardToCrib(player, discards)(discarding).asInstanceOf[A]

  //   "accept discards" when {
  //     "player discards" in dummyDiscarding() {
  //       case discarding0 @ Discarding(deck0, scores0, hands0, dealer0, pone0, crib0) =>
  //         val dealerDiscards = hands0(dealer0).toSeq.take(2)
  //         val discarding1    = doDiscardsReturning[Discarding](dealer0, dealerDiscards)(discarding0)

  //         val Discarding(deck1, scores1, hands1, dealer1, pone1, crib1) = discarding1
  //         deck1 should be(deck0)
  //         scores1 should be(scores0)
  //         dealer1 should be(dealer0)
  //         pone1 should be(pone0)
  //         hands1(dealer1).toSeq should not contain allElementsOf(dealerDiscards.toSeq)
  //         hands1(pone1) should be(hands0(pone0))
  //         crib1.toSeq should contain allElementsOf (dealerDiscards.toSeq)
  //     }
  //   }

  //   "not accept discards" when {
  //     "non-participating player" in dummyDiscarding() {
  //       case discarding @ Discarding(_, _, hands, dealer, _, _) =>
  //         val bogusPlayer    = Player.newPlayer
  //         val dealerDiscards = hands(dealer).toSeq.take(2)
  //         val caught         = intercept[IllegalArgumentException] {
  //           doDiscardsReturning[Discarding](bogusPlayer, dealerDiscards)(discarding)
  //         }
  //         caught should be(a[IllegalArgumentException])
  //         val expectedRule   = NonParticipatingPlayer(bogusPlayer)
  //         caught.getMessage should be(s"requirement failed: $expectedRule")
  //     }

  //     "unheld card" in dummyDiscarding() {
  //       case discarding @ Discarding(_, _, hands, dealer, pone, _) =>
  //         val dealerDiscards = hands(pone).toSeq.take(2)
  //         val caught         = intercept[IllegalArgumentException] {
  //           doDiscardsReturning[Discarding](dealer, dealerDiscards)(discarding)
  //         }
  //         caught should be(a[IllegalArgumentException])
  //         val expectedRule   = UnheldCards(dealer, dealerDiscards)
  //         caught.getMessage should be(s"requirement failed: $expectedRule")
  //     }

  //     "exceeded discard" in dummyDiscarding() {
  //       case discarding @ Discarding(_, _, hands, dealer, _, _) =>
  //         val dealerDiscards = hands(dealer).toSeq.take(3)
  //         val caught         = intercept[IllegalArgumentException] {
  //           doDiscardsReturning[Discarding](dealer, dealerDiscards)(discarding)
  //         }
  //         caught should be(a[IllegalArgumentException])
  //         val expectedRule   = TooManyDiscards(dealer, dealerDiscards)
  //         caught.getMessage should be(s"requirement failed: $expectedRule")
  //     }
  //   }

  //   /** ## Before the Play
  //     *
  //     * After the crib is laid away, the non-dealer cuts the pack. The dealer turns up the top card
  //     * of the lower packet and places it face up on top of the pack. This card is the "starter." If
  //     * the starter is a jack, it is called "His Heels," and the dealer pegs (scores) 2 points at
  //     * once. The starter is not used in the play phase of Cribbage , but is used later for making
  //     * various card combinations that score points.
  //     */
  //   "start the play" when {
  //     "both players fully discarded" in dummyDiscarding() {
  //       case discarding0 @ Discarding(deck0, scores0, hands0, dealer0, pone0, _) =>
  //         val dealerDiscards = hands0(dealer0).toSeq.take(2)
  //         val poneDiscards   = hands0(pone0).toSeq.take(2)

  //         val discarding1 = doDiscardsReturning[Discarding](dealer0, dealerDiscards)(discarding0)
  //         val playing2    = doDiscardsReturning[Playing](pone0, poneDiscards)(discarding1)

  //         val Playing(scores2, hands2, dealer2, pone2, crib2, cut2, plays2) = playing2
  //         dealer2 should be(dealer0)
  //         pone2 should be(pone0)
  //         hands2(dealer2).toSeq should not contain allElementsOf(dealerDiscards.toSeq)
  //         hands2(pone2).toSeq should not contain allElementsOf(poneDiscards.toSeq)
  //         crib2.size should be(CardsRequiredInCrib)
  //         crib2.toSeq should contain allElementsOf (dealerDiscards.toSeq ++ poneDiscards.toSeq)
  //         hands2(dealer2).toSeq ++ hands2(pone2).toSeq ++ crib2.toSeq should not contain (cut2)
  //         plays2.nextPlayer should be(pone2)
  //     }
  //   }

  //   "score his heels" when {
  //     "both players fully discarded - playing" in dummyDiscarding(
  //       maybeCut = Some(Card(Jack, Hearts))
  //     ) { case discarding0 @ Discarding(_, scores0, hands0, dealer0, pone0, crib0) =>
  //       val dealerDiscards = hands0(dealer0).toSeq.take(2)
  //       val poneDiscards   = hands0(pone0).toSeq.take(2)

  //       val discarding1 = doDiscardsReturning[Discarding](dealer0, dealerDiscards)(discarding0)
  //       val playing2    = doDiscardsReturning[Playing](pone0, poneDiscards)(discarding1)

  //       val Playing(scores2, hands2, dealer2, pone2, _, _, _) = playing2
  //       dealer2 should be(dealer0)
  //       pone2 should be(pone0)
  //       scores2(dealer2) should be(Score(0, 2))
  //       scores2(pone2) should be(Score.zero)
  //     }

  //     "both players fully discarded - finished" in dummyDiscarding(
  //       dealerScore = Score(0, 120),
  //       maybeCut = Some(Card(Jack, Hearts))
  //     ) { case discarding0 @ Discarding(_, _, hands0, dealer0, pone0, _) =>
  //       val dealerDiscards = hands0(dealer0).toSeq.take(2)
  //       val poneDiscards   = hands0(pone0).toSeq.take(2)

  //       val discarding1 = doDiscardsReturning[Discarding](dealer0, dealerDiscards)(discarding0)
  //       val finished2   = doDiscardsReturning[Finished](pone0, poneDiscards)(discarding1)

  //       val Finished(scores2) = finished2
  //       scores2(dealer0) should be(Score(120, 122))
  //       scores2(pone0) should be(Score.zero)
  //     }
  //   }

  //   /** ## The Play
  //     *
  //     * After the starter is turned, the non-dealer lays one of his cards face up on the table. The
  //     * dealer similarly exposes a card, then non-dealer again, and so on - the hands are exposed
  //     * card by card, alternately except for a "Go," (Pass) as noted below. Each player keeps his
  //     * cards separate from those of his opponent.
  //     */
  //   def dummyPlaying(
  //       dealerScore: Score = Score.zero,
  //       poneScore: Score = Score.zero,
  //       dealerCards: Seq[Card] = Seq.empty,
  //       poneCards: Seq[Card] = Seq.empty,
  //       cribCards: Seq[Card] = Seq.empty,
  //       maybeCut: Option[Card] = None,
  //       inPlays: Seq[(Int, Card)] = Seq.empty,
  //       playeds: Seq[(Int, Card)] = Seq.empty
  //   )(f: Playing => Unit) =
  //     val dealer = Player.newPlayer
  //     val pone   = Player.newPlayer

  //     def intToPlayer(i: Int) = if i == 0 then dealer else pone

  //     def mapToPlays(ics: Seq[(Int, Card)]) = ics.map((i, c) => Laid(intToPlayer(i), c))

  //     val scores = Map(dealer -> dealerScore, pone -> poneScore)
  //     val hands  = Map(dealer -> Cards.handOf(dealerCards), pone -> Cards.handOf(poneCards))
  //     val crib   = Cards.cribOf(cribCards)
  //     val cut    = maybeCut.getOrElse(Card(Ace, Spades))
  //     val plays  = Plays(pone).copy(inPlay = mapToPlays(inPlays), played = mapToPlays(playeds))

  //     f(Playing(scores, hands, dealer, pone, crib, cut, plays))

  //   def doPlayFor[A](player: Player, card: Card)(playing: Playing): A =
  //     Cribbage.play(player, card)(playing).asInstanceOf[A]

  //   def doPassFor[A](player: Player)(playing: Playing): A =
  //     Cribbage.pass(player)(playing).asInstanceOf[A]

  //   /** As each person plays, he announces a running total of pips reached by the addition of the
  //     * last card to all those previously played. (Example: The non-dealer begins with a four,
  //     * saying "Four." The dealer plays a nine, saying "Thirteen".) The kings, queens and jacks
  //     * count 10 each; every other card counts its pip value (the ace counts one).
  //     */
  //   "accept play" in dummyPlaying(
  //     dealerCards = Seq(Card(Nine, Spades)),
  //     poneCards = Seq(Card(Four, Spades))
  //   ) { case playing0 @ Playing(_, hands0, dealer0, pone0, _, _, plays0) =>
  //     val playing1 = doPlayFor[Playing](pone0, Card(Four, Spades))(playing0)

  //     val Playing(_, hands1, dealer1, pone1, _, _, plays1) = playing1
  //     dealer1 should be(dealer0)
  //     pone1 should be(pone0)
  //     hands1(dealer1) should be(hands0(dealer0))
  //     hands1(pone1).toSeq should not contain (Card(Four, Spades))
  //     plays1.nextPlayer should be(dealer1)
  //     plays1.runningTotal should be(4)
  //     plays1.inPlay should contain theSameElementsInOrderAs (Seq(Laid(pone1, Card(Four, Spades))))

  //     val playing2 = doPlayFor[Playing](playing1.dealer, Card(Nine, Spades))(playing1)

  //     val Playing(_, hands2, dealer2, pone2, _, _, plays2) = playing2
  //     dealer2 should be(dealer1)
  //     pone2 should be(pone1)
  //     hands2(dealer2).toSeq should not contain (Card(Nine, Spades))
  //     hands2(pone2).toSeq should not contain (Card(Four, Spades))
  //     plays2.nextPlayer should be(pone2)
  //     plays2.runningTotal should be(13)
  //     plays2.inPlay should contain theSameElementsInOrderAs (Seq(
  //       Laid(pone2, Card(Four, Spades)),
  //       Laid(dealer2, Card(Nine, Spades))
  //     ))
  //   }

  //   "not accept play" when {
  //     "non-participating player" in dummyPlaying(poneCards = Seq(Card(Two, Spades))) {
  //       case playing @ Playing(_, hands, _, pone, _, _, _) =>
  //         val bogusPlayer  = Player.newPlayer
  //         val poneCard     = hands(pone).toSeq.head
  //         val caught       = intercept[IllegalArgumentException] {
  //           doPlayFor[Playing](bogusPlayer, poneCard)(playing)
  //         }
  //         caught should be(a[IllegalArgumentException])
  //         val expectedRule = NonParticipatingPlayer(bogusPlayer)
  //         caught.getMessage should be(s"requirement failed: $expectedRule")
  //     }

  //     "unheld card" in dummyPlaying(poneCards = Seq(Card(Two, Spades))) {
  //       case playing @ Playing(_, hands, _, pone, _, _, _) =>
  //         val poneCard     = Card(Two, Hearts)
  //         val caught       = intercept[IllegalArgumentException] {
  //           doPlayFor[Playing](pone, poneCard)(playing)
  //         }
  //         caught should be(a[IllegalArgumentException])
  //         val expectedRule = UnheldCard(pone, poneCard)
  //         caught.getMessage should be(s"requirement failed: $expectedRule")
  //     }

  //     "not their turn" in dummyPlaying(
  //       dealerCards = Seq(Card(Two, Hearts)),
  //       poneCards = Seq(Card(Two, Spades))
  //     ) { case playing @ Playing(_, hands, dealer, _, _, _, _) =>
  //       val dealerCard   = hands(dealer).toSeq.head
  //       val caught       = intercept[IllegalArgumentException] {
  //         doPlayFor[Playing](dealer, dealerCard)(playing)
  //       }
  //       caught should be(a[IllegalArgumentException])
  //       val expectedRule = NotYourTurn(dealer)
  //       caught.getMessage should be(s"requirement failed: $expectedRule")
  //     }

  //     "play exceeds target" in dummyPlaying(
  //       poneCards = Seq(Card(Two, Hearts)),
  //       inPlays = Seq(0 -> Card(King, Hearts), 0 -> Card(King, Clubs), 0 -> Card(King, Diamonds))
  //     ) { case playing @ Playing(_, hands, _, pone, _, _, _) =>
  //       val poneCard     = hands(pone).toSeq.head
  //       val caught       = intercept[IllegalArgumentException] {
  //         doPlayFor[Playing](pone, poneCard)(playing)
  //       }
  //       caught should be(a[IllegalArgumentException])
  //       val expectedRule = PlayTargetExceeded(pone, poneCard)
  //       caught.getMessage should be(s"requirement failed: $expectedRule")
  //     }
  //   }

  //   "score play - state" when {
  //     "play target not reached - playing" in dummyPlaying(
  //       poneCards = Seq(Card(Five, Hearts)),
  //       inPlays = Seq(0 -> Card(Ten, Hearts))
  //     ) { case playing0: Playing =>
  //       val playing1 = doPlayFor[Playing](playing0.pone, Card(Five, Hearts))(playing0)

  //       val Playing(scores1, _, _, pone1, _, _, _) = playing1
  //       scores1(pone1) should be(Score(0, 2))
  //     }

  //     "play target not reached - discarding" in dummyPlaying(
  //       poneCards = Seq(Card(Ace, Hearts)),
  //       inPlays = Seq(0 -> Card(Jack, Hearts), 0 -> Card(Queen, Hearts)),
  //       playeds = Seq(
  //         0 -> Card(Nine, Hearts),
  //         0 -> Card(Seven, Clubs),
  //         1 -> Card(Six, Spades),
  //         1 -> Card(Two, Spades),
  //         1 -> Card(King, Spades)
  //       ),
  //       maybeCut = Some(Card(Queen, Clubs))
  //     ) { case playing0 @ Playing(_, _, dealer0, pone0, _, _, _) =>
  //       val discarding1 = doPlayFor[Discarding](pone0, Card(Ace, Hearts))(playing0)

  //       val Discarding(_, scores1, _, dealer1, _, _) = discarding1
  //       scores1(dealer1) should be(Score(0, 1))
  //     }

  //     "play target not reached - finished" in dummyPlaying(
  //       poneCards = Seq(Card(Five, Hearts)),
  //       inPlays = Seq(0 -> Card(Ten, Hearts)),
  //       poneScore = Score(0, 120)
  //     ) { case playing0: Playing =>
  //       val finished1 = doPlayFor[Finished](playing0.pone, Card(Five, Hearts))(playing0)
  //       finished1.scores(playing0.pone) should be(Score(120, 122))
  //     }

  //     "play target reached - playing" in dummyPlaying(
  //       poneCards = Seq(Card(Ace, Hearts)),
  //       inPlays = Seq(0 -> Card(Ten, Hearts), 0 -> Card(Jack, Hearts), 0 -> Card(Queen, Hearts))
  //     ) { case playing0: Playing =>
  //       val playing1 = doPlayFor[Playing](playing0.pone, Card(Ace, Hearts))(playing0)

  //       val Playing(scores1, _, _, pone1, _, _, _) = playing1
  //       scores1(pone1) should be(Score(0, 2))
  //     }

  //     "play target reached - discarding" in dummyPlaying(
  //       poneCards = Seq(Card(Ace, Hearts)),
  //       inPlays = Seq(0 -> Card(Ten, Hearts), 0 -> Card(Jack, Hearts), 0 -> Card(Queen, Hearts)),
  //       playeds = Seq(
  //         0 -> Card(Nine, Hearts),
  //         1 -> Card(Two, Spades),
  //         1 -> Card(Queen, Spades),
  //         1 -> Card(Six, Spades)
  //       ),
  //       maybeCut = Some(Card(King, Clubs))
  //     ) { case playing0 @ Playing(_, _, dealer0, pone0, _, _, _) =>
  //       val discarding1 = doPlayFor[Discarding](pone0, Card(Ace, Hearts))(playing0)

  //       val Discarding(_, scores1, _, dealer1, pone1, _) = discarding1
  //       scores1(dealer1) should be(Score(0, 2))
  //     }

  //     "play target reached - finished" in dummyPlaying(
  //       poneCards = Seq(Card(Ace, Hearts)),
  //       poneScore = Score(0, 120),
  //       inPlays = Seq(0 -> Card(Ten, Hearts), 0 -> Card(Jack, Hearts), 0 -> Card(Queen, Hearts)),
  //       playeds = Seq(
  //         0 -> Card(Nine, Hearts),
  //         0 -> Card(Four, Spades),
  //         0 -> Card(Five, Spades),
  //         0 -> Card(Six, Spades)
  //       )
  //     ) { case playing0: Playing =>
  //       val finished1 = doPlayFor[Finished](playing0.pone, Card(Ace, Hearts))(playing0)
  //       finished1.scores(playing0.pone) should be(Score(120, 122))
  //     }

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

  //   /** ## The Go
  //     *
  //     * During play, the running total of cards may never be carried beyond 31. If a player cannot
  //     * add another card without exceeding 31, he or she says "Go" and the opponent pegs 1. After
  //     * gaining the Go, the opponent must first lay down any additional cards he can without
  //     * exceeding 31. Besides the point for Go, he may then score any additional points that can be
  //     * made through pairs and runs (described later). If a player reaches exactly 31, he pegs two
  //     * instead of one for Go.
  //     *
  //     * The player who called Go leads for the next series of plays, with the count starting at
  //     * zero. The lead may not be combined with any cards previously played to form a scoring
  //     * combination; the Go has interrupted the sequence.
  //     *
  //     * The person who plays the last card pegs one for Go, plus one extra if the card brings the
  //     * count to exactly 31. The dealer is sure to peg at least one point in every hand, for he will
  //     * have a Go on the last card if not earlier.
  //     */
  //   "accept pass" when {
  //     "pone has no valid card" in dummyPlaying() {
  //       case playing0 @ Playing(_, _, _, pone0, _, _, plays0) =>
  //         val playing1 = doPassFor[Playing](pone0)(playing0)

  //         val Playing(_, _, dealer1, _, _, _, plays1) = playing1

  //         plays1.nextPlayer should be(dealer1)
  //         plays1.runningTotal should be(plays0.runningTotal)
  //         plays1.inPlay should contain theSameElementsInOrderAs (Seq(Pass(pone0)))
  //     }

  //     "dealer has no valid card" in dummyPlaying(
  //       poneCards = Seq(Card(Ten, Spades))
  //     ) { case playing0 @ Playing(_, _, dealer0, pone0, _, _, plays0) =>
  //       val playing1 = doPlayFor[Playing](pone0, Card(Ten, Spades))(playing0)
  //       val playing2 = doPassFor[Playing](dealer0)(playing1)

  //       val Playing(_, _, dealer2, pone2, _, _, plays2) = playing2

  //       plays2.nextPlayer should be(pone2)
  //       plays2.runningTotal should be(Card(Ten, Spades).value)
  //       plays2.inPlay should contain theSameElementsInOrderAs (Seq(
  //         Laid(pone0, Card(Ten, Spades)),
  //         Pass(dealer0)
  //       ))
  //     }
  //   }

  //   "not accept pass" when {
  //     "non-participating player" in dummyPlaying(poneCards = Seq(Card(Two, Spades))) {
  //       case playing @ Playing(_, hands, _, pone, _, _, _) =>
  //         val bogusPlayer  = Player.newPlayer
  //         val caught       = intercept[IllegalArgumentException] {
  //           doPassFor[Playing](bogusPlayer)(playing)
  //         }
  //         caught should be(a[IllegalArgumentException])
  //         val expectedRule = NonParticipatingPlayer(bogusPlayer)
  //         caught.getMessage should be(s"requirement failed: $expectedRule")
  //     }

  //     "hold card that can be played" in dummyPlaying(poneCards = Seq(Card(Two, Spades))) {
  //       case playing @ Playing(_, hands, _, pone, _, _, _) =>
  //         val caught       = intercept[IllegalArgumentException] {
  //           doPassFor[Playing](pone)(playing)
  //         }
  //         caught should be(a[IllegalArgumentException])
  //         val expectedRule = PlayTargetAttainable(pone)
  //         caught.getMessage should be(s"requirement failed: $expectedRule")
  //     }
  //   }

  //   "score pass" when {
  //     "both players passed - playing" in dummyPlaying(
  //       dealerCards = Seq(Card(Jack, Hearts)),
  //       inPlays = Seq(0 -> Card(Jack, Hearts), 0 -> Card(Jack, Clubs), 0 -> Card(Jack, Diamonds))
  //     ) { case playing0 @ Playing(_, _, dealer0, pone0, _, _, _) =>
  //       val playing1 = doPassFor[Playing](pone0)(playing0)
  //       val playing2 = doPassFor[Playing](dealer0)(playing1)

  //       val Playing(scores2, _, dealer2, pone2, _, _, _) = playing2
  //       dealer2 should be(dealer0)
  //       pone2 should be(pone0)
  //       scores2(dealer2) should be(Score(0, 1))
  //     }

  //     "both players passed - finished" in dummyPlaying(
  //       dealerScore = Score(0, 120)
  //     ) { case playing0 @ Playing(_, _, dealer0, pone0, _, _, _) =>
  //       val playing1  = doPassFor[Playing](pone0)(playing0)
  //       val finished2 = doPassFor[Finished](dealer0)(playing1)

  //       val Finished(scores2) = finished2
  //       scores2(dealer0) should be(Score(120, 121))
  //     }

  //   }

  //   /** ## Pegging
  //     *
  //     * The object in play is to score points by pegging. In addition to a Go, a player may score
  //     * for the following combinations:
  //     *
  //     *   - Fifteen: For adding a card that makes the total 15 Peg 2
  //     *   - Pair: For adding a card of the same rank as the card just played Peg 2 (Note that face
  //     *     cards pair only by actual rank: jack with jack, but not jack with queen.)
  //     *   - Triplet: For adding the third card of the same rank. Peg 6
  //     *   - Four: (also called "Double Pair" or "Double Pair Royal") For adding the fourth card of
  //     *     the same rank Peg 12
  //     *   - Run (Sequence): For adding a card that forms, with those just played:
  //     *     - For a sequence of three Peg 3
  //     *     - For a sequence of four. Peg 4
  //     *     - For a sequence of five. Peg 5
  //     *     - (Peg one point more for each extra card of a sequence. Note that runs are independent
  //     *       of suits, but go strictly by rank; to illustrate: 9, 10, J, or J, 9, 10 is a run but
  //     *       9, 10, Q is not.)
  //     */
  //   "score play - points" when {
  //     "fifteen" in dummyPlaying(
  //       poneCards = Seq(Card(Eight, Diamonds)),
  //       inPlays = Seq(0 -> Card(Seven, Diamonds))
  //     ) { case playing0 @ Playing(_, _, _, pone, _, _, _) =>
  //       val playing1 = doPlayFor[Playing](pone, Card(Eight, Diamonds))(playing0)
  //       playing1.scores(pone) should be(Score(0, 2))
  //     }

  //     "pair" in dummyPlaying(
  //       poneCards = Seq(Card(Ace, Spades)),
  //       inPlays = Seq(0 -> Card(Ace, Diamonds))
  //     ) { case playing0 @ Playing(_, _, _, pone, _, _, _) =>
  //       val playing1 = doPlayFor[Playing](pone, Card(Ace, Spades))(playing0)
  //       playing1.scores(pone) should be(Score(0, 2))
  //     }

  //     "triplet" in dummyPlaying(
  //       poneCards = Seq(Card(Ace, Spades)),
  //       inPlays = Seq(0 -> Card(Ace, Diamonds), 0 -> Card(Ace, Hearts))
  //     ) { case playing0 @ Playing(_, _, _, pone, _, _, _) =>
  //       val playing1 = doPlayFor[Playing](pone, Card(Ace, Spades))(playing0)
  //       playing1.scores(pone) should be(Score(0, 6))
  //     }

  //     "double pair" in dummyPlaying(
  //       poneCards = Seq(Card(Ace, Spades)),
  //       inPlays = Seq(0 -> Card(Ace, Diamonds), 0 -> Card(Ace, Hearts), 0 -> Card(Ace, Clubs))
  //     ) { case playing0 @ Playing(_, _, _, pone, _, _, _) =>
  //       val playing1 = doPlayFor[Playing](pone, Card(Ace, Spades))(playing0)
  //       playing1.scores(pone) should be(Score(0, 12))
  //     }

  //     "run" in dummyPlaying(
  //       poneCards = Seq(Card(Ace, Spades)),
  //       inPlays = Seq(0 -> Card(Two, Diamonds), 0 -> Card(Three, Hearts))
  //     ) { case playing0 @ Playing(_, _, _, pone, _, _, _) =>
  //       val playing1 = doPlayFor[Playing](pone, Card(Ace, Spades))(playing0)
  //       playing1.scores(pone) should be(Score(0, 3))
  //     }

  //     /** It is important to keep track of the order in which cards are played to determine whether
  //       * what looks like a sequence or a run has been interrupted by a "foreign card." Example:
  //       * Cards are played in this order: 8, 7, 7, 6. The dealer pegs 2 for 15, and the opponent
  //       * pegs 2 for pair, but the dealer cannot peg for run because of the extra seven (foreign
  //       * card) that has been played. Example: Cards are played in this order: 9, 6, 8, 7. The
  //       * dealer pegs 2 for fifteen when he plays the six and pegs 4 for run when he plays the seven
  //       * (the 6, 7, 8, 9 sequence). The cards were not played in sequential order, but they form a
  //       * true run with no foreign card.
  //       */
  //     "run edge case 1" in dummyPlaying(
  //       poneCards = Seq(Card(Eight, Spades), Card(Seven, Spades)),
  //       dealerCards = Seq(Card(Seven, Hearts), Card(Six, Hearts))
  //     ) { case playing0 @ Playing(_, _, dealer, pone, _, _, _) =>
  //       val playing1 = doPlayFor[Playing](pone, Card(Eight, Spades))(playing0)
  //       val playing2 = doPlayFor[Playing](dealer, Card(Seven, Hearts))(playing1)
  //       val playing3 = doPlayFor[Playing](pone, Card(Seven, Spades))(playing2)
  //       val playing4 = doPlayFor[Playing](dealer, Card(Six, Hearts))(playing3)
  //       playing4.scores(dealer) should be(Score(0, 2))
  //       playing4.scores(pone) should be(Score(0, 2))
  //     }

  //     "run edge case 2" in dummyPlaying(
  //       poneCards = Seq(Card(Nine, Spades), Card(Eight, Spades)),
  //       dealerCards = Seq(Card(Seven, Hearts), Card(Six, Hearts))
  //     ) { case playing0 @ Playing(_, _, dealer, pone, _, _, _) =>
  //       val playing1 = doPlayFor[Playing](pone, Card(Nine, Spades))(playing0)
  //       val playing2 = doPlayFor[Playing](dealer, Card(Six, Hearts))(playing1)
  //       val playing3 = doPlayFor[Playing](pone, Card(Eight, Spades))(playing2)
  //       val playing4 = doPlayFor[Playing](dealer, Card(Seven, Hearts))(playing3)
  //       playing4.scores(dealer) should be(Score(2, 6))
  //       playing4.scores(pone) should be(Score(0, 0))
  //     }
  //   }

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