package com.nigeleke.cribbage

import model.*
import Card.Face.*
import Card.Suit.*

import cats.data.NonEmptyList
import cats.data.Validated.*
import cats.data.ValidatedNel
import cats.syntax.validated.*

import org.scalatest.*
import org.scalatest.matchers.should.*
import org.scalatest.wordspec.*

import java.util.UUID

// Methods to help test descriptions...
//
extension (cards: Deck | Crib | Hand)
  def take(n: Int): Seq[Card] = cards.toCardSeq.take(n)
  def size: Int = cards.toCardSeq.size
  def ++(otherCards: Seq[Card]): Seq[Card] = cards.toCardSeq ++ otherCards

// Methods get game into preset, and potentially invalid, state...
// Use for test purposes only...
//
extension (game: Game)
  def withPlayer(hand: Hand = Seq.empty, score: Score = Score(0, 0)): Game =
    val playerId = Player.newId
    val updatedPlayers = game.players :+ playerId
    val updatedScores = game.scores.updated(playerId, score)
    val updatedHands = game.hands.updated(playerId, hand)
    game.copy(players = updatedPlayers, scores = updatedScores, hands = updatedHands)
  def withDealer(index: Int): Game =
    val playerId = game.players(index)
    game.copy(optDealer = Some(playerId))
  def withNextToPlay(index: Int): Game =
    val playerId = game.players(index)
    val updatedPlays = game.plays.copy(optNextToPlay = Some(playerId))
    game.copy(plays = updatedPlays)
  def withCrib(crib: Crib): Game = game.copy(crib = crib)
  def withCut(cut: Card): Game = game.copy(optCut = Some(cut))
  def card(face: Face, suit: Suit): Card =
    val hands = game.hands.values
    val crib = game.crib
    val cut = game.optCut.get
    (hands.head.toCardSeq ++ hands.last.toCardSeq ++ crib.toCardSeq :+ cut)
      .filter(c => c.face == face && c.suit == suit)
      .head

class GameSpec extends AnyWordSpec with Matchers:

  "A Game" when {
    "initialised" should {
      val game = Game()
      "have a full shuffled deck of cards" in {
        game.deck.size should be(52)
      }
      "have no players" in {
        game.players should be(empty)
      }
      "not have selected a dealer" in {
        game.optDealer should be(empty)
      }
      "have an empty crib" in {
        game.crib.toCardSeq should be(empty)
      }
      "not have cut a card" in {
        game.optCut should be(empty)
      }
      "not have had any plays made" in {
        val plays = game.plays
        plays.optNextToPlay should be(empty)
        plays.current should be(empty)
        plays.passCount should be(0)
        plays.previous should be(empty)
      }
    }

    "starting" should {
      "allow a new player to join" in {
        val id = Player.newId
        Game().validNel andThen
          addPlayer(id) match
          case Valid(game) =>
            game.players.size should be(1)
          case Invalid(errors) => fail(errors.toList.mkString(", "))
      }

      "allow two new players to join" in {
        val id1 = Player.newId
        val id2 = Player.newId
        Game().validNel andThen
          addPlayer(id1) andThen
          addPlayer(id2) match
          case Valid(game) =>
            game.players.size should be(2)
          case Invalid(errors) => fail(errors.toList.mkString(", "))
      }

      "not allow three players to join" in {
        val id1 = Player.newId
        val id2 = Player.newId
        val id3 = Player.newId
        val game = Game()
        game.validNel andThen
          addPlayer(id1) andThen
          addPlayer(id2) andThen
          addPlayer(id3) match
          case Valid(game)     => fail(s"Incorrectly added Player ${id3} to Game ${game.id}")
          case Invalid(errors) => errors.toList should be(List(s"Game ${game.id} does not need any further players"))
      }

      "not allow the same player to join twice" in {
        val id = Player.newId
        val game = Game()
        game.validNel andThen
          addPlayer(id) andThen
          addPlayer(id) match
          case Valid(game)     => fail(s"Incorrectly added Player ${id} twice to Game ${game.id}")
          case Invalid(errors) => errors.toList should be(List(s"Player ${id} has already joined Game ${game.id}"))
      }
    }

    "started" should {
      "deal hands" in {
        val id1 = Player.newId
        val id2 = Player.newId
        Game().validNel andThen
          addPlayer(id1) andThen
          addPlayer(id2) andThen
          start match
          case Valid(game) =>
            game.players.size should be(2)
            val hand1 = game.hands(id1)
            val hand2 = game.hands(id2)
            val deck = game.deck
            hand1.size should be(6)
            hand2.size should be(6)
            deck.size should be(40)
            hand1.toCardSeq.toSet intersect deck.toCardSeq.toSet should be(empty)
            hand2.toCardSeq.toSet intersect deck.toCardSeq.toSet should be(empty)
            hand1.toCardSeq.toSet intersect hand2.toCardSeq.toSet should be(empty)
          case Invalid(errors) => fail(errors.toList.mkString(", "))
      }

      "not start if not enough players" in {
        val id = Player.newId
        val game = Game()
        game.validNel andThen
          addPlayer(id) andThen
          start match
          case Valid(game)     => fail(s"Game ${game.id} allowed to start without enough Players")
          case Invalid(errors) => errors.toList should be(List(s"Game ${game.id} requires more Players"))
      }

      "not start if already started" in {
        val id1 = Player.newId
        val id2 = Player.newId
        val game = Game()
        game.validNel andThen
          addPlayer(id1) andThen
          addPlayer(id2) andThen
          start andThen
          start match
          case Valid(game)     => fail(s"Game ${game.id} allowed to start when already started")
          case Invalid(errors) => errors.toList should be(List(s"Game ${game.id} has already been started"))
      }
    }

    def discardingGame(test: Game => Unit) =
      val id1 = Player.newId
      val id2 = Player.newId
      Game().validNel andThen
        addPlayer(id1) andThen
        addPlayer(id2) andThen
        start match
        case Valid(game)    => test(game)
        case Invalid(error) => fail(s"Failed to create discardingGame for test: $error")

    "discarding" should {

      "allow a player to discard cards into the crib" in discardingGame { game =>
        val playerId = game.players.head
        val discards = game.hands(playerId).take(2)
        game.validNel andThen
          discardCribCards(playerId, discards) match
          case Valid(game) =>
            game.hands(playerId).toCardSeq should not contain allElementsOf(discards)
            game.crib.toCardSeq should contain allElementsOf (discards)
          case Invalid(errors) => fail(errors.toList.mkString(", "))
      }

      "not allow a discard" when {

        "the discard contains cards not owned by the player" in discardingGame { game =>
          val player1Id = game.players.head
          val discards = game.hands(player1Id).take(2)
          val player2Id = game.players.last

          game.validNel andThen
            discardCribCards(player2Id, discards) match
            case Valid(game) => fail(s"Incorrectly discarded $discards for $player2Id in $game")
            case Invalid(errors) =>
              errors.toList should be(List(s"Player ${player2Id} does not own card(s) (${discards.toString})"))
        }

        "the discard contains too few cards" in discardingGame { game =>
          val playerId = game.players.head
          val discards = game.hands(playerId).take(1)
          game.validNel andThen
            discardCribCards(playerId, discards) match
            case Valid(game)     => fail(s"Incorrectly allowed single card discard $discards for $playerId in $game")
            case Invalid(errors) => errors.toList should be(List(s"Game ${game.id} expecting 2 cards discarded; have 1"))
        }

        "the discard contains too many cards" in discardingGame { game =>
          val playerId = game.players.head
          val discards = game.hands(playerId).take(3)
          game.validNel andThen
            discardCribCards(playerId, discards) match
            case Valid(game)     => fail(s"Incorrectly allowed three card discard $discards for $playerId in $game")
            case Invalid(errors) => errors.toList should be(List(s"Game ${game.id} expecting 2 cards discarded; have 3"))
        }

      }

      "start the Play" when {

        "both Players have discarded" in discardingGame { game =>
          val player1Id = game.players.head
          val discards1 = game.hands(player1Id).take(2)

          val player2Id = game.players.last
          val discards2 = game.hands(player2Id).take(2)

          game.validNel andThen
            discardCribCards(player1Id, discards1) andThen
            discardCribCards(player2Id, discards2) match
            case Valid(game) =>
              val plays = game.plays
              plays.optNextToPlay should not be (empty)
              plays.optNextToPlay should be(game.optPone)
              plays.current should be(empty)
              plays.passCount should be(0)
              plays.previous should be(empty)
              game.crib.toCardSeq should contain allElementsOf (discards1 ++ discards2)
              (game.optCut, game.optDealer) match
                case (Some(Card(_, Card.Face.Jack, _)), Some(dealerId)) => game.scores(dealerId) should be(Score(0, 2))
                case (Some(_), Some(dealerId))                          => game.scores(dealerId) should be(Score(0, 0))
                case (optCard, optDealer) => fail(s"Failed to initialise Plays for $optCard, $optDealer")
            case Invalid(errors) => fail(errors.toList.mkString(", "))
        }

      }

    }

    def playingGame(test: Game => Unit) =
      val game = Game()
        .withPlayer(Seq(Card(Ten, Diamonds), Card(Ten, Spades), Card(Five, Hearts), Card(Four, Clubs)))
        .withPlayer(Seq(Card(King, Diamonds), Card(King, Spades), Card(Eight, Diamonds), Card(Seven, Spades)))
        .withCrib(Seq(Card(Ten, Hearts), Card(Ten, Clubs), Card(King, Hearts), Card(King, Clubs)))
        .withDealer(0)
        .withNextToPlay(1)
        .withCut(Card(Ace, Spades))
      test(game)

    def fullyPlayedGame(test: Game => Unit) =
      playingGame { game =>
        val dealerId = game.optDealer.head
        val poneId = game.optPone.head
        val originalDealerHand = game.hands(dealerId)
        val originalPoneHand = game.hands(poneId)
        val kingDiamonds = game.card(King, Diamonds)
        val tenDiamonds = game.card(Ten, Diamonds)
        val eightDiamonds = game.card(Eight, Diamonds)
        val fiveHearts = game.card(Five, Hearts)
        val kingSpades = game.card(King, Spades)
        val tenSpades = game.card(Ten, Spades)
        val fourClubs = game.card(Four, Clubs)
        val sevenSpades = game.card(Seven, Spades)
        game.validNel andThen
          playCard(poneId, kingDiamonds) andThen
          playCard(dealerId, tenDiamonds) andThen
          playCard(poneId, eightDiamonds) andThen
          pass(dealerId) andThen
          pass(poneId) andThen
          playCard(dealerId, fiveHearts) andThen
          playCard(poneId, kingSpades) andThen
          playCard(dealerId, tenSpades) andThen
          pass(poneId) andThen
          playCard(dealerId, fourClubs) andThen
          pass(dealerId) andThen
          playCard(poneId, sevenSpades) andThen
          regatherPlays match
          case Valid(game)     => test(game)
          case Invalid(errors) => fail(errors.toList.mkString(", "))
      }

    "playing" should {

      "allow the next Player to Play" when {
        "they have at least one valid cardId for the CurrentPlay" in playingGame { game =>
          val dealerId = game.optDealer.head
          val poneId = game.optPone.head
          val card = game.hands(poneId).toCardSeq.head
          game.validNel andThen
            playCard(poneId, card) match
            case Valid(game) =>
              game.hands(poneId).toCardSeq should not contain (card)
              game.plays.current.head should be(Plays.Play.Laid(poneId, card))
              game.plays.optNextToPlay should be(Some(dealerId))
            case Invalid(errors) => fail(errors.toList.mkString(", "))
        }
      }

      "not allow the next Player to Play" when {
        "it's not their turn" in playingGame { game =>
          val dealerId = game.optDealer.head
          val card = game.hands(dealerId).toCardSeq.head
          game.validNel andThen
            playCard(dealerId, card) match
            case v @ Valid(_)    => fail(s"Incorrectly allowed wrong player to Play")
            case Invalid(errors) => errors.toList should be(List(s"It is not Player ${dealerId}'s turn to play"))
        }

        "they have no valid cards for the current play" in playingGame { game =>
          val dealerId = game.optDealer.head
          val poneId = game.optPone.head
          val kingDiamonds = game.card(King, Diamonds)
          val tenDiamonds = game.card(Ten, Diamonds)
          val kingSpades = game.card(King, Spades)
          val fiveHearts = game.card(Five, Hearts)
          game.validNel andThen
            playCard(poneId, kingDiamonds) andThen
            playCard(dealerId, tenDiamonds) andThen
            playCard(poneId, kingSpades) andThen
            playCard(dealerId, fiveHearts) match
            case Valid(game)     => fail("Incorrectly allowed Play that would break 31")
            case Invalid(errors) => errors.toList should be(List(s"Playing $fiveHearts exceeds 31; current play total is 30"))
        }
      }

      "allow the next Player to Pass" when {
        "they have no valid cards for the CurrentPlay" in playingGame { game =>
          val dealerId = game.optDealer.head
          val poneId = game.optPone.head
          val kingDiamonds = game.card(King, Diamonds)
          val tenDiamonds = game.card(Ten, Diamonds)
          val kingSpades = game.card(King, Spades)
          game.validNel andThen
            playCard(poneId, kingDiamonds) andThen
            playCard(dealerId, tenDiamonds) andThen
            playCard(poneId, kingSpades) andThen
            pass(dealerId) match
            case Valid(game) =>
              game.plays should be(
                Plays(
                  Some(poneId),
                  Seq(
                    Plays.Play.Laid(poneId, kingDiamonds),
                    Plays.Play.Laid(dealerId, tenDiamonds),
                    Plays.Play.Laid(poneId, kingSpades),
                    Plays.Play.Pass(dealerId)
                  ),
                  Set(dealerId),
                  Seq.empty
                )
              )
            case Invalid(errors) => fail(errors.toList.mkString(", "))
        }
      }

      "not allow the next Player to Pass" when {
        "they have at least one valid Card for the current Play" in playingGame { game =>
          val poneId = game.optPone.head
          game.validNel andThen
            pass(poneId) match
            case Valid(game) => fail(s"Incorrectly allowed pass for $poneId in $game")
            case Invalid(errors) =>
              errors.toList should be(List(s"Player $poneId cannot pass; they have one or more valid cards to play"))
        }
      }

      "score the Play" when {
        "a Card is laid" in playingGame { game =>
          val dealerId = game.optDealer.head
          val poneId = game.optPone.head
          val kingDiamonds = game.card(King, Diamonds)
          val fiveHearts = game.card(Five, Hearts)
          val initialDealerPoints = game.scores(dealerId).points
          game.validNel andThen
            playCard(poneId, kingDiamonds) andThen
            playCard(dealerId, fiveHearts) match
            case Valid(game) =>
              game.scores(dealerId).back should be(initialDealerPoints)
              game.scores(dealerId).front should be(initialDealerPoints + 2)
            case Invalid(errors) => fail(errors.toList.mkString(", "))
        }
      }

      "score the end of Play" when {
        "play finishes with runningTotal less than 31" in playingGame { game =>
          val dealerId = game.optDealer.head
          val poneId = game.optPone.head
          val kingDiamonds = game.card(King, Diamonds)
          val tenDiamonds = game.card(Ten, Diamonds)
          val kingSpades = game.card(King, Spades)
          val initialPonePoints = game.scores(dealerId).points
          game.validNel andThen
            playCard(poneId, kingDiamonds) andThen
            playCard(dealerId, tenDiamonds) andThen
            playCard(poneId, kingSpades) andThen
            pass(dealerId) andThen
            pass(poneId) match
            case Valid(game) =>
              game.scores(poneId).back should be(initialPonePoints)
              game.scores(poneId).front should be(initialPonePoints + 1)
            case Invalid(errors) => fail(errors.toList.mkString(", "))
        }

        "play finishes with runningTotal exactly 31" in playingGame { game =>
          val dealerId = game.optDealer.head
          val poneId = game.optPone.head
          val kingDiamonds = game.card(King, Diamonds)
          val tenDiamonds = game.card(Ten, Diamonds)
          val sevenSpades = game.card(Seven, Spades)
          val fourClubs = game.card(Four, Clubs)
          val initialDealerPoints = game.scores(dealerId).points
          game.validNel andThen
            playCard(poneId, kingDiamonds) andThen
            playCard(dealerId, tenDiamonds) andThen
            playCard(poneId, sevenSpades) andThen
            playCard(dealerId, fourClubs) andThen
            pass(poneId) andThen
            pass(dealerId) match
            case Valid(game) =>
              game.scores(dealerId).back should be(initialDealerPoints)
              game.scores(dealerId).front should be(initialDealerPoints + 2)
            case Invalid(errors) => fail(errors.toList.mkString(", "))
        }
      }

      "start the next Play" when {
        "both Players have Passed" in playingGame { game =>
          val dealerId = game.optDealer.head
          val poneId = game.optPone.head
          val kingDiamonds = game.card(King, Diamonds)
          val tenDiamonds = game.card(Ten, Diamonds)
          val kingSpades = game.card(King, Spades)
          game.validNel andThen
            playCard(poneId, kingDiamonds) andThen
            playCard(dealerId, tenDiamonds) andThen
            playCard(poneId, kingSpades) andThen
            pass(dealerId) andThen
            pass(poneId) match
            case Valid(game) =>
              val plays = game.plays
              plays.passCount should be(0)
              plays.optNextToPlay should be(Some(dealerId))
              plays.current should be(empty)
              plays.runningTotal should be(0)
              plays.previous should contain theSameElementsInOrderAs (
                Seq(
                  Plays.Play.Laid(poneId, kingDiamonds),
                  Plays.Play.Laid(dealerId, tenDiamonds),
                  Plays.Play.Laid(poneId, kingSpades),
                  Plays.Play.Pass(dealerId),
                  Plays.Play.Pass(poneId)
                )
              )
            case Invalid(errors) => fail(errors.toList.mkString(", "))
        }

        "current Play finished on 31" in playingGame { game =>
          val dealerId = game.optDealer.head
          val poneId = game.optPone.head
          val kingDiamonds = game.card(King, Diamonds)
          val tenDiamonds = game.card(Ten, Diamonds)
          val sevenSpades = game.card(Seven, Spades)
          val fourClubs = game.card(Four, Clubs)
          game.validNel andThen
            playCard(poneId, kingDiamonds) andThen
            playCard(dealerId, tenDiamonds) andThen
            playCard(poneId, sevenSpades) andThen
            playCard(dealerId, fourClubs) andThen
            pass(poneId) andThen
            pass(dealerId) match
            case Valid(game) =>
              val plays = game.plays
              plays.passCount should be(0)
              plays.optNextToPlay should be(Some(poneId))
              plays.current should be(empty)
              plays.runningTotal should be(0)
              plays.previous should contain theSameElementsInOrderAs (
                Seq(
                  Plays.Play.Laid(poneId, kingDiamonds),
                  Plays.Play.Laid(dealerId, tenDiamonds),
                  Plays.Play.Laid(poneId, sevenSpades),
                  Plays.Play.Laid(dealerId, fourClubs),
                  Plays.Play.Pass(poneId),
                  Plays.Play.Pass(dealerId)
                )
              )
            case Invalid(errors) => fail(errors.toList.mkString(", "))
        }
      }

      "perform Scoring" when {
        "all Plays completed" in fullyPlayedGame { game =>
          val dealerId = game.optDealer.get
          val poneId = game.optPone.get

          game.hands(dealerId).size should be(4)
          game.scores(dealerId).points should be(1)

          game.hands(poneId).size should be(4)
          game.scores(poneId).points should be(1 + 2 + 1)

          val poneScoredGame = game.validNel andThen scorePoneHand
          poneScoredGame match
            case Valid(game)     => game.scores(poneId).points should be(4 + 4)
            case Invalid(errors) => fail(errors.toList.mkString(", "))

          val dealerScoredGame = poneScoredGame andThen scoreDealerHand
          dealerScoredGame match
            case Valid(game)     => game.scores(dealerId).points should be(1 + 10)
            case Invalid(errors) => fail(errors.toList.mkString(", "))

          val cribScoredGame = dealerScoredGame andThen scoreCrib
          cribScoredGame match
            case Valid(game)     => game.scores(dealerId).points should be(11 + 4)
            case Invalid(errors) => fail(errors.toList.mkString(", "))
        }
      }
    }

    "scored" should {
      "swap the Dealer" in fullyPlayedGame { game1 =>
        game1.validNel andThen
          swapDealer match
          case Valid(game2) =>
            game2.optDealer should be(game1.optPone)
            game2.optPone should be(game1.optDealer)
          case Invalid(errors) => fail(errors.toList.mkString(", "))
      }

      "deal new Hands" in fullyPlayedGame { game =>
        game.validNel andThen
          swapDealer match
          case Valid(game) =>
            game.hands.values.map(_.size) should be(Seq(6, 6))
            game.crib.size should be(0)
            game.plays should be(Plays(game.optPone))
          case Invalid(errors) => fail(errors.toList.mkString(", "))
      }
    }
  }
