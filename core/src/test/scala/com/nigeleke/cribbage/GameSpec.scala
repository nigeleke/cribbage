//package com.nigeleke.cribbage
//
//import model.*
//
//import org.scalatest.*
//import org.scalatest.matchers.should.*
//import org.scalatest.wordspec.*
//
////// Methods to help test descriptions...
//////
////extension (cards: Deck | Crib | Hand)
////  def take(n: Int): Seq[Card] = cards.toCardSeq.take(n)
////  def size: Int = cards.toCardSeq.size
////  def ++(otherCards: Seq[Card]): Seq[Card] = cards.toCardSeq ++ otherCards
////
////// Methods get state into preset, and potentially invalid, state...
////// Use for test purposes only...
//////
////extension (state: Game)
////  def withPlayer(hand: Hand = Seq.empty, score: Score = Score(0, 0)): Game =
////    val player = Player.newId
////    val updatedPlayers = state.players :+ player
////    val updatedScores = state.scores.updated(player, score)
////    val updatedHands = state.hands.updated(player, hand)
////    state.copy(players = updatedPlayers, scores = updatedScores, hands = updatedHands)
////  def withDealer(index: Int): Game =
////    val player = state.players(index)
////    state.copy(optDealer = Some(player))
////  def withNextToPlay(index: Int): Game =
////    val player = state.players(index)
////    val updatedPlays = state.plays.copy(optNextToPlay = Some(player))
////    state.copy(plays = updatedPlays)
////  def withCrib(crib: Crib): Game = state.copy(crib = crib)
////  def withCut(cut: Card): Game = state.copy(optCut = Some(cut))
////  def card(face: Face, suit: Suit): Card =
////    val hands = state.hands.values
////    val crib = state.crib
////    val cut = state.optCut.get
////    (hands.head.toCardSeq ++ hands.last.toCardSeq ++ crib.toCardSeq :+ cut)
////      .filter(c => c.face == face && c.suit == suit)
////      .head
////
////class GameSpec extends AnyWordSpec with Matchers:
////
////  "A Game" should {
////    val state = Game()
////    "have a full shuffled deck of cards" in {
////      state.deck.size should be(52)
////    }
////    "have no players" in {
////      state.players should be(empty)
////    }
////    "not have selected a dealer" in {
////      state.optDealer should be(empty)
////    }
////    "have an empty crib" in {
////      state.crib.toCardSeq should be(empty)
////    }
////    "not have cut a card" in {
////      state.optCut should be(empty)
////    }
////    "not have had any plays made" in {
////      val plays = state.plays
////      plays.optNextToPlay should be(empty)
////      plays.inPlay should be(empty)
////      plays.passCount should be(0)
////      plays.played should be(empty)
////    }
////  }
//
////    "starting" should {
////      "allow a new player to join" in {
////        val id = Player.newId
////        Game().validNel andThen
////          addPlayer(id) match
////          case Valid(state)    =>
////            state.players.size should be(1)
////          case Invalid(errors) => fail(errors.toList.mkString(", "))
////      }
////
////      "allow two new players to join" in {
////        val id1 = Player.newId
////        val id2 = Player.newId
////        Game().validNel andThen
////          addPlayer(id1) andThen
////          addPlayer(id2) match
////          case Valid(state)    =>
////            state.players.size should be(2)
////          case Invalid(errors) => fail(errors.toList.mkString(", "))
////      }
////
////      "not allow three players to join" in {
////        val id1   = Player.newId
////        val id2   = Player.newId
////        val id3   = Player.newId
////        val state = Game()
////        state.validNel andThen
////          addPlayer(id1) andThen
////          addPlayer(id2) andThen
////          addPlayer(id3) match
////          case Valid(state)    => fail(s"Incorrectly added Player ${id3} to Game ${state.id}")
////          case Invalid(errors) =>
////            errors.toList should be(List(s"Game ${state.id} does not need any further players"))
////      }
////
////      "not allow the same player to join twice" in {
////        val id    = Player.newId
////        val state = Game()
////        state.validNel andThen
////          addPlayer(id) andThen
////          addPlayer(id) match
////          case Valid(state)    => fail(s"Incorrectly added Player ${id} twice to Game ${state.id}")
////          case Invalid(errors) =>
////            errors.toList should be(List(s"Player ${id} has already joined Game ${state.id}"))
////      }
////    }
////
////    "started" should {
////      "deal hands" in {
////        val id1 = Player.newId
////        val id2 = Player.newId
////        Game().validNel andThen
////          addPlayer(id1) andThen
////          addPlayer(id2) andThen
////          start match
////          case Valid(state)    =>
////            state.players.size should be(2)
////            val hand1 = state.hands(id1)
////            val hand2 = state.hands(id2)
////            val deck  = state.deck
////            hand1.size should be(6)
////            hand2.size should be(6)
////            deck.size should be(40)
////            hand1.toCardSeq.toSet intersect deck.toCardSeq.toSet should be(empty)
////            hand2.toCardSeq.toSet intersect deck.toCardSeq.toSet should be(empty)
////            hand1.toCardSeq.toSet intersect hand2.toCardSeq.toSet should be(empty)
////          case Invalid(errors) => fail(errors.toList.mkString(", "))
////      }
////
////      "not start if not enough players" in {
////        val id    = Player.newId
////        val state = Game()
////        state.validNel andThen
////          addPlayer(id) andThen
////          start match
////          case Valid(state)    => fail(s"Game ${state.id} allowed to start without enough Players")
////          case Invalid(errors) =>
////            errors.toList should be(List(s"Game ${state.id} requires more Players"))
////      }
////
////      "not start if already started" in {
////        val id1   = Player.newId
////        val id2   = Player.newId
////        val state = Game()
////        state.validNel andThen
////          addPlayer(id1) andThen
////          addPlayer(id2) andThen
////          start andThen
////          start match
////          case Valid(state)    => fail(s"Game ${state.id} allowed to start when already started")
////          case Invalid(errors) =>
////            errors.toList should be(List(s"Game ${state.id} has already been started"))
////      }
////    }
////
////    def discardingState(test: Game => Unit) =
////      val id1 = Player.newId
////      val id2 = Player.newId
////      Game().validNel andThen
////        addPlayer(id1) andThen
////        addPlayer(id2) andThen
////        start match
////        case Valid(state)   => test(state)
////        case Invalid(error) => fail(s"Failed to create discardingState for test: $error")
////
////    "discarding" should {
////
////      "allow a player to discard cards into the crib" in discardingState { state =>
////        val player   = state.players.head
////        val discards = state.hands(player).take(2)
////        state.validNel andThen
////          discardCribCards(player, discards) match
////          case Valid(state)    =>
////            state.hands(player).toCardSeq should not contain allElementsOf(discards)
////            state.crib.toCardSeq should contain allElementsOf (discards)
////          case Invalid(errors) => fail(errors.toList.mkString(", "))
////      }
////
////      "not allow a discard" when {
////
////        "the discard contains cards not owned by the player" in discardingState { state =>
////          val player1Id = state.players.head
////          val discards  = state.hands(player1Id).take(2)
////          val player2Id = state.players.last
////
////          state.validNel andThen
////            discardCribCards(player2Id, discards) match
////            case Valid(state)    => fail(s"Incorrectly discarded $discards for $player2Id in $state")
////            case Invalid(errors) =>
////              errors.toList should be(
////                List(s"Player ${player2Id} does not own card(s) (${discards.toString})")
////              )
////        }
////
////        "the discard contains too few cards" in discardingState { state =>
////          val player   = state.players.head
////          val discards = state.hands(player).take(1)
////          state.validNel andThen
////            discardCribCards(player, discards) match
////            case Valid(state)    =>
////              fail(s"Incorrectly allowed single card discard $discards for $player in $state")
////            case Invalid(errors) =>
////              errors.toList should be(List(s"Game ${state.id} expecting 2 cards discarded; have 1"))
////        }
////
////        "the discard contains too many cards" in discardingState { state =>
////          val player   = state.players.head
////          val discards = state.hands(player).take(3)
////          state.validNel andThen
////            discardCribCards(player, discards) match
////            case Valid(state)    =>
////              fail(s"Incorrectly allowed three card discard $discards for $player in $state")
////            case Invalid(errors) =>
////              errors.toList should be(List(s"Game ${state.id} expecting 2 cards discarded; have 3"))
////        }
////
////      }
////
////      "start the Play" when {
////
////        "both Players have discarded" in discardingState { state =>
////          val player1Id = state.players.head
////          val discards1 = state.hands(player1Id).take(2)
////
////          val player2Id = state.players.last
////          val discards2 = state.hands(player2Id).take(2)
////
////          state.validNel andThen
////            discardCribCards(player1Id, discards1) andThen
////            discardCribCards(player2Id, discards2) match
////            case Valid(state)    =>
////              val plays = state.plays
////              plays.optNextToPlay should not be (empty)
////              plays.optNextToPlay should be(state.optPone)
////              plays.inPlay should be(empty)
////              plays.passCount should be(0)
////              plays.played should be(empty)
////              state.crib.toCardSeq should contain allElementsOf (discards1 ++ discards2)
////              (state.optCut, state.optDealer) match
////                case (Some(Card(_, Card.Face.Jack, _)), Some(dealerId)) =>
////                  state.scores(dealerId) should be(Score(0, 2))
////                case (Some(_), Some(dealerId))                          => state.scores(dealerId) should be(Score(0, 0))
////                case (optCard, optDealer)                               =>
////                  fail(s"Failed to initialise Plays for $optCard, $optDealer")
////            case Invalid(errors) => fail(errors.toList.mkString(", "))
////        }
////
////      }
////
////    }
////
////    def playingState(test: Game => Unit) =
////      val state = Game()
////        .withPlayer(
////          Seq(Card(Ten, Diamonds), Card(Ten, Spades), Card(Five, Hearts), Card(Four, Clubs))
////        )
////        .withPlayer(
////          Seq(Card(King, Diamonds), Card(King, Spades), Card(Eight, Diamonds), Card(Seven, Spades))
////        )
////        .withCrib(Seq(Card(Ten, Hearts), Card(Ten, Clubs), Card(King, Hearts), Card(King, Clubs)))
////        .withDealer(0)
////        .withNextToPlay(1)
////        .withCut(Card(Ace, Spades))
////      test(state)
////
////    def playingStateFinished(test: Game => Unit) =
////      playingState { state =>
////        val dealerId           = state.optDealer.head
////        val poneId             = state.optPone.head
////        val originalDealerHand = state.hands(dealerId)
////        val originalPoneHand   = state.hands(poneId)
////        val kingDiamonds       = state.card(King, Diamonds)
////        val tenDiamonds        = state.card(Ten, Diamonds)
////        val eightDiamonds      = state.card(Eight, Diamonds)
////        val fiveHearts         = state.card(Five, Hearts)
////        val kingSpades         = state.card(King, Spades)
////        val tenSpades          = state.card(Ten, Spades)
////        val fourClubs          = state.card(Four, Clubs)
////        val sevenSpades        = state.card(Seven, Spades)
////        state.validNel andThen
////          playCard(poneId, kingDiamonds) andThen
////          playCard(dealerId, tenDiamonds) andThen
////          playCard(poneId, eightDiamonds) andThen
////          pass(dealerId) andThen
////          pass(poneId) andThen
////          playCard(dealerId, fiveHearts) andThen
////          playCard(poneId, kingSpades) andThen
////          playCard(dealerId, tenSpades) andThen
////          pass(poneId) andThen
////          playCard(dealerId, fourClubs) andThen
////          pass(dealerId) andThen
////          playCard(poneId, sevenSpades) andThen
////          regatherPlays match
////          case Valid(state)    => test(state)
////          case Invalid(errors) => fail(errors.toList.mkString(", "))
////      }
////
////    "playing" should {
////
////      "allow the next Player to Play" when {
////        "they have at least one valid cardId for the CurrentPlay" in playingState { state =>
////          val dealerId = state.optDealer.head
////          val poneId   = state.optPone.head
////          val card     = state.hands(poneId).toCardSeq.head
////          state.validNel andThen
////            playCard(poneId, card) match
////            case Valid(state)    =>
////              state.hands(poneId).toCardSeq should not contain (card)
////              state.plays.inPlay.head should be(Plays.Play.Laid(poneId, card))
////              state.plays.optNextToPlay should be(Some(dealerId))
////            case Invalid(errors) => fail(errors.toList.mkString(", "))
////        }
////      }
////
////      "not allow the next Player to Play" when {
////        "it's not their turn" in playingState { state =>
////          val dealerId = state.optDealer.head
////          val card     = state.hands(dealerId).toCardSeq.head
////          state.validNel andThen
////            playCard(dealerId, card) match
////            case v @ Valid(_)    => fail(s"Incorrectly allowed wrong player to Play")
////            case Invalid(errors) =>
////              errors.toList should be(List(s"It is not Player ${dealerId}'s turn to play"))
////        }
////
////        "they have no valid cards for the inPlay play" in playingState { state =>
////          val dealerId     = state.optDealer.head
////          val poneId       = state.optPone.head
////          val kingDiamonds = state.card(King, Diamonds)
////          val tenDiamonds  = state.card(Ten, Diamonds)
////          val kingSpades   = state.card(King, Spades)
////          val fiveHearts   = state.card(Five, Hearts)
////          state.validNel andThen
////            playCard(poneId, kingDiamonds) andThen
////            playCard(dealerId, tenDiamonds) andThen
////            playCard(poneId, kingSpades) andThen
////            playCard(dealerId, fiveHearts) match
////            case Valid(state)    => fail("Incorrectly allowed Play that would break 31")
////            case Invalid(errors) =>
////              errors.toList should be(
////                List(s"Playing $fiveHearts exceeds 31; inPlay play total is 30")
////              )
////        }
////      }
////
////      "allow the next Player to Pass" when {
////        "they have no valid cards for the CurrentPlay" in playingState { state =>
////          val dealerId     = state.optDealer.head
////          val poneId       = state.optPone.head
////          val kingDiamonds = state.card(King, Diamonds)
////          val tenDiamonds  = state.card(Ten, Diamonds)
////          val kingSpades   = state.card(King, Spades)
////          state.validNel andThen
////            playCard(poneId, kingDiamonds) andThen
////            playCard(dealerId, tenDiamonds) andThen
////            playCard(poneId, kingSpades) andThen
////            pass(dealerId) match
////            case Valid(state)    =>
////              state.plays should be(
////                Plays(
////                  Some(poneId),
////                  Seq(
////                    Plays.Play.Laid(poneId, kingDiamonds),
////                    Plays.Play.Laid(dealerId, tenDiamonds),
////                    Plays.Play.Laid(poneId, kingSpades),
////                    Plays.Play.Pass(dealerId)
////                  ),
////                  Set(dealerId),
////                  Seq.empty
////                )
////              )
////            case Invalid(errors) => fail(errors.toList.mkString(", "))
////        }
////      }
////
////      "not allow the next Player to Pass" when {
////        "they have at least one valid Card for the inPlay Play" in playingState { state =>
////          val poneId = state.optPone.head
////          state.validNel andThen
////            pass(poneId) match
////            case Valid(state)    => fail(s"Incorrectly allowed pass for $poneId in $state")
////            case Invalid(errors) =>
////              errors.toList should be(
////                List(s"Player $poneId cannot pass; they have one or more valid cards to play")
////              )
////        }
////      }
////
////      "score the Play" when {
////        "a Card is laid" in playingState { state =>
////          val dealerId            = state.optDealer.head
////          val poneId              = state.optPone.head
////          val kingDiamonds        = state.card(King, Diamonds)
////          val fiveHearts          = state.card(Five, Hearts)
////          val initialDealerPoints = state.scores(dealerId).points
////          state.validNel andThen
////            playCard(poneId, kingDiamonds) andThen
////            playCard(dealerId, fiveHearts) match
////            case Valid(state)    =>
////              state.scores(dealerId).back should be(initialDealerPoints)
////              state.scores(dealerId).front should be(initialDealerPoints + 2)
////            case Invalid(errors) => fail(errors.toList.mkString(", "))
////        }
////      }
////
////      "score the end of Play" when {
////        "play finishes with runningTotal less than 31" in playingState { state =>
////          val dealerId          = state.optDealer.head
////          val poneId            = state.optPone.head
////          val kingDiamonds      = state.card(King, Diamonds)
////          val tenDiamonds       = state.card(Ten, Diamonds)
////          val kingSpades        = state.card(King, Spades)
////          val initialPonePoints = state.scores(dealerId).points
////          state.validNel andThen
////            playCard(poneId, kingDiamonds) andThen
////            playCard(dealerId, tenDiamonds) andThen
////            playCard(poneId, kingSpades) andThen
////            pass(dealerId) andThen
////            pass(poneId) match
////            case Valid(state)    =>
////              state.scores(poneId).back should be(initialPonePoints)
////              state.scores(poneId).front should be(initialPonePoints + 1)
////            case Invalid(errors) => fail(errors.toList.mkString(", "))
////        }
////
////        "play finishes with runningTotal exactly 31" in playingState { state =>
////          val dealerId            = state.optDealer.head
////          val poneId              = state.optPone.head
////          val kingDiamonds        = state.card(King, Diamonds)
////          val tenDiamonds         = state.card(Ten, Diamonds)
////          val sevenSpades         = state.card(Seven, Spades)
////          val fourClubs           = state.card(Four, Clubs)
////          val initialDealerPoints = state.scores(dealerId).points
////          state.validNel andThen
////            playCard(poneId, kingDiamonds) andThen
////            playCard(dealerId, tenDiamonds) andThen
////            playCard(poneId, sevenSpades) andThen
////            playCard(dealerId, fourClubs) andThen
////            pass(poneId) andThen
////            pass(dealerId) match
////            case Valid(state)    =>
////              state.scores(dealerId).back should be(initialDealerPoints)
////              state.scores(dealerId).front should be(initialDealerPoints + 2)
////            case Invalid(errors) => fail(errors.toList.mkString(", "))
////        }
////      }
////
////      "start the next Play" when {
////        "both Players have Passed" in playingState { state =>
////          val dealerId     = state.optDealer.head
////          val poneId       = state.optPone.head
////          val kingDiamonds = state.card(King, Diamonds)
////          val tenDiamonds  = state.card(Ten, Diamonds)
////          val kingSpades   = state.card(King, Spades)
////          state.validNel andThen
////            playCard(poneId, kingDiamonds) andThen
////            playCard(dealerId, tenDiamonds) andThen
////            playCard(poneId, kingSpades) andThen
////            pass(dealerId) andThen
////            pass(poneId) match
////            case Valid(state)    =>
////              val plays = state.plays
////              plays.passCount should be(0)
////              plays.optNextToPlay should be(Some(dealerId))
////              plays.inPlay should be(empty)
////              plays.runningTotal should be(0)
////              plays.played should contain theSameElementsInOrderAs (
////                Seq(
////                  Plays.Play.Laid(poneId, kingDiamonds),
////                  Plays.Play.Laid(dealerId, tenDiamonds),
////                  Plays.Play.Laid(poneId, kingSpades),
////                  Plays.Play.Pass(dealerId),
////                  Plays.Play.Pass(poneId)
////                )
////              )
////            case Invalid(errors) => fail(errors.toList.mkString(", "))
////        }
////
////        "inPlay Play finished on 31" in playingState { state =>
////          val dealerId     = state.optDealer.head
////          val poneId       = state.optPone.head
////          val kingDiamonds = state.card(King, Diamonds)
////          val tenDiamonds  = state.card(Ten, Diamonds)
////          val sevenSpades  = state.card(Seven, Spades)
////          val fourClubs    = state.card(Four, Clubs)
////          state.validNel andThen
////            playCard(poneId, kingDiamonds) andThen
////            playCard(dealerId, tenDiamonds) andThen
////            playCard(poneId, sevenSpades) andThen
////            playCard(dealerId, fourClubs) andThen
////            pass(poneId) andThen
////            pass(dealerId) match
////            case Valid(state)    =>
////              val plays = state.plays
////              plays.passCount should be(0)
////              plays.optNextToPlay should be(Some(poneId))
////              plays.inPlay should be(empty)
////              plays.runningTotal should be(0)
////              plays.played should contain theSameElementsInOrderAs (
////                Seq(
////                  Plays.Play.Laid(poneId, kingDiamonds),
////                  Plays.Play.Laid(dealerId, tenDiamonds),
////                  Plays.Play.Laid(poneId, sevenSpades),
////                  Plays.Play.Laid(dealerId, fourClubs),
////                  Plays.Play.Pass(poneId),
////                  Plays.Play.Pass(dealerId)
////                )
////              )
////            case Invalid(errors) => fail(errors.toList.mkString(", "))
////        }
////      }
////
////      "perform Scoring" when {
////        "all Plays completed" in playingStateFinished { state =>
////          val dealerId = state.optDealer.get
////          val poneId   = state.optPone.get
////
////          state.hands(dealerId).size should be(4)
////          state.scores(dealerId).points should be(1)
////
////          state.hands(poneId).size should be(4)
////          state.scores(poneId).points should be(1 + 2 + 1)
////
////          val poneScoredGame = state.validNel andThen scorePoneHand
////          poneScoredGame match
////            case Valid(state)    => state.scores(poneId).points should be(4 + 4)
////            case Invalid(errors) => fail(errors.toList.mkString(", "))
////
////          val dealerScoredGame = poneScoredGame andThen scoreDealerHand
////          dealerScoredGame match
////            case Valid(state)    => state.scores(dealerId).points should be(1 + 10)
////            case Invalid(errors) => fail(errors.toList.mkString(", "))
////
////          val cribScoredGame = dealerScoredGame andThen scoreCrib
////          cribScoredGame match
////            case Valid(state)    => state.scores(dealerId).points should be(11 + 4)
////            case Invalid(errors) => fail(errors.toList.mkString(", "))
////        }
////      }
////    }
////
////    "scored" should {
////      "swap the Dealer" in playingStateFinished { game1 =>
////        game1.validNel andThen
////          swapDealer match
////          case Valid(game2)    =>
////            game2.optDealer should be(game1.optPone)
////            game2.optPone should be(game1.optDealer)
////          case Invalid(errors) => fail(errors.toList.mkString(", "))
////      }
////
////      "deal new Hands" in playingStateFinished { state =>
////        state.validNel andThen
////          swapDealer match
////          case Valid(state)    =>
////            state.hands.values.map(_.size) should be(Seq(6, 6))
////            state.crib.size should be(0)
////            state.plays should be(Plays(state.optPone))
////          case Invalid(errors) => fail(errors.toList.mkString(", "))
////      }
////    }
////  }
