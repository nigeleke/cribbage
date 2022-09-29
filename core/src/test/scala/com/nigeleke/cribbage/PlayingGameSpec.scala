package com.nigeleke.cribbage

import effects.*
import model.*
import model.Card.{Face, Suit}
import Face.*, Suit.*

import cats.data.NonEmptyList
import cats.data.Validated.*
import cats.data.ValidatedNel
import cats.syntax.validated.*

import org.scalatest.*
import org.scalatest.matchers.should.*
import org.scalatest.wordspec.*

import java.util.UUID

extension (game: PlayingGame)
  def card(face: Face, suit: Suit) =
    val availableCards = game.hands(game.dealer) ++ game.hands(game.pone)
    availableCards.filter(card => card.face == face && card.suit == suit).head

class PlayingGameSpec extends AnyWordSpec with Matchers:

  def playingGame(test: PlayingGame => Unit) =
    val dealer = Player.newId
    val pone = Player.newId
    val game = PlayingGame(
      UUID.randomUUID().asInstanceOf[GameId],
      Map(
        dealer -> Score.zero,
        pone -> Score.zero
      ),
      Map(
        dealer -> Hand(Seq(Card(Ten, Diamonds), Card(Ten, Spades), Card(Five, Hearts), Card(Four, Clubs))),
        pone -> Hand(Seq(Card(King, Diamonds), Card(King, Spades), Card(Eight, Diamonds), Card(Seven, Spades)))
      ),
      dealer,
      pone,
      Crib(Seq(Card(Ten, Hearts), Card(Ten, Clubs), Card(King, Hearts), Card(King, Clubs))),
      Card(Ace, Spades),
      Plays(pone)
    )
    test(game)

  def fullyPlayedGame(test: PlayingGame => Unit) =
    val dealer = Player.newId
    val pone = Player.newId
    val game = PlayingGame(
      UUID.randomUUID().asInstanceOf[GameId],
      Map(
        dealer -> Score.zero,
        pone -> Score.zero
      ),
      Map(
        dealer -> Hand(),
        pone -> Hand()
      ),
      dealer,
      pone,
      Crib(Seq(Card(Ten, Hearts), Card(Ten, Clubs), Card(King, Hearts), Card(King, Clubs))),
      Card(Ace, Spades),
      Plays(
        pone,
        Seq(
          Plays.Laid(pone, Card(Seven, Spades))
        ),
        Set.empty,
        Seq(
          Plays.Laid(pone, Card(King, Diamonds)),
          Plays.Laid(dealer, Card(Ten, Diamonds)),
          Plays.Laid(pone, Card(Eight, Diamonds)),
          Plays.Pass(dealer),
          Plays.Pass(pone),
          Plays.Laid(dealer, Card(Five, Hearts)),
          Plays.Laid(pone, Card(King, Spades)),
          Plays.Laid(dealer, Card(Ten, Spades)),
          Plays.Pass(pone),
          Plays.Laid(dealer, Card(Four, Clubs)),
          Plays.Pass(dealer)
        )
      )
    )
    test(game)

  "A PlayingGame" should {

    def asPlayingGame(game: Game): ValidatedNel[String, PlayingGame] = game match
      case p: PlayingGame => p.validNel
      case _              => s"PlayingGame expected; have $game".invalidNel

    def asWonGame(game: Game): ValidatedNel[String, WonGame] = game match
      case w: WonGame => w.validNel
      case _          => s"WonGame expected; have $game".invalidNel

    "allow the next Player to Play" when {
      "they have at least one valid cardId for the CurrentPlay" in playingGame { game =>
        val dealer = game.dealer
        val pone = game.pone
        val card = game.hands(pone).head
        game.validNel andThen
          playCard(pone, card) andThen asPlayingGame match
          case Valid(game) =>
            game.hands(pone) should not contain (card)
            game.plays.current.head should be(Plays.Laid(pone, card))
            game.plays.nextToPlay should be(dealer)
          case Invalid(errors) => failWith(errors)
      }
    }

    "not allow the next Player to Play" when {
      "it's not their turn" in playingGame { game =>
        val dealer = game.dealer
        val card = game.hands(dealer).head
        game.validNel andThen
          playCard(dealer, card) match
          case v @ Valid(_)    => fail(s"Incorrectly allowed wrong player to Play")
          case Invalid(errors) => passIfMatched(errors, List(s"It is not Player ${dealer}'s turn to play"))
      }

      "they have no valid cards for the current play" in playingGame { game =>
        val dealer = game.dealer
        val pone = game.pone
        val kingDiamonds = game.card(King, Diamonds)
        val tenDiamonds = game.card(Ten, Diamonds)
        val kingSpades = game.card(King, Spades)
        val fiveHearts = game.card(Five, Hearts)
        game.validNel andThen
          playCard(pone, kingDiamonds) andThen asPlayingGame andThen
          playCard(dealer, tenDiamonds) andThen asPlayingGame andThen
          playCard(pone, kingSpades) andThen asPlayingGame andThen
          playCard(dealer, fiveHearts) match
          case Valid(_)        => fail("Incorrectly allowed Play that would break 31")
          case Invalid(errors) => passIfMatched(errors, List(s"Playing $fiveHearts exceeds 31; current play total is 30"))
      }
    }

    "allow the next Player to Pass" when {
      "they have no valid cards for the CurrentPlay" in playingGame { game =>
        val dealer = game.dealer
        val pone = game.pone
        val kingDiamonds = game.card(King, Diamonds)
        val tenDiamonds = game.card(Ten, Diamonds)
        val kingSpades = game.card(King, Spades)
        game.validNel andThen
          playCard(pone, kingDiamonds) andThen asPlayingGame andThen
          playCard(dealer, tenDiamonds) andThen asPlayingGame andThen
          playCard(pone, kingSpades) andThen asPlayingGame andThen
          pass(dealer) andThen asPlayingGame match
          case Valid(game) =>
            game.plays should be(
              Plays(
                pone,
                Seq(
                  Plays.Laid(pone, kingDiamonds),
                  Plays.Laid(dealer, tenDiamonds),
                  Plays.Laid(pone, kingSpades),
                  Plays.Pass(dealer)
                ),
                Set(dealer),
                Seq.empty
              )
            )
          case Invalid(errors) => failWith(errors)
      }
    }

    "not allow the next Player to Pass" when {
      "they have at least one valid Card for the current Play" in playingGame { game =>
        val pone = game.pone
        game.validNel andThen
          pass(pone) match
          case Valid(game) => fail(s"Incorrectly allowed pass for $pone in $game")
          case Invalid(errors) =>
            passIfMatched(errors, List(s"Player $pone cannot pass; they have one or more valid cards to play"))
      }
    }

    "score the Play" when {
      "a Card is laid" in playingGame { game =>
        val dealer = game.dealer
        val pone = game.pone
        val kingDiamonds = game.card(King, Diamonds)
        val fiveHearts = game.card(Five, Hearts)
        val initialDealerPoints = game.scores(dealer).points
        game.validNel andThen
          playCard(pone, kingDiamonds) andThen asPlayingGame andThen
          playCard(dealer, fiveHearts) andThen asPlayingGame match
          case Valid(game) =>
            game.scores(dealer).back should be(initialDealerPoints)
            game.scores(dealer).front should be(initialDealerPoints + 2)
          case Invalid(errors) => failWith(errors)
      }
    }

    "score the end of Play" when {
      "play finishes with runningTotal less than 31" in playingGame { game =>
        val dealer = game.dealer
        val pone = game.pone
        val kingDiamonds = game.card(King, Diamonds)
        val tenDiamonds = game.card(Ten, Diamonds)
        val kingSpades = game.card(King, Spades)
        val initialPonePoints = game.scores(dealer).points
        game.validNel andThen
          playCard(pone, kingDiamonds) andThen asPlayingGame andThen
          playCard(dealer, tenDiamonds) andThen asPlayingGame andThen
          playCard(pone, kingSpades) andThen asPlayingGame andThen
          pass(dealer) andThen asPlayingGame andThen
          pass(pone) andThen asPlayingGame match
          case Valid(game) =>
            game.scores(pone).back should be(initialPonePoints)
            game.scores(pone).front should be(initialPonePoints + 1)
          case Invalid(errors) => failWith(errors)
      }

      "play finishes with runningTotal exactly 31" in playingGame { game =>
        val dealer = game.dealer
        val pone = game.pone
        val kingDiamonds = game.card(King, Diamonds)
        val tenDiamonds = game.card(Ten, Diamonds)
        val sevenSpades = game.card(Seven, Spades)
        val fourClubs = game.card(Four, Clubs)
        val initialDealerPoints = game.scores(dealer).points
        game.validNel andThen
          playCard(pone, kingDiamonds) andThen asPlayingGame andThen
          playCard(dealer, tenDiamonds) andThen asPlayingGame andThen
          playCard(pone, sevenSpades) andThen asPlayingGame andThen
          playCard(dealer, fourClubs) andThen asPlayingGame andThen
          pass(pone) andThen asPlayingGame andThen
          pass(dealer) andThen asPlayingGame match
          case Valid(game) =>
            game.scores(dealer).back should be(initialDealerPoints)
            game.scores(dealer).front should be(initialDealerPoints + 2)
          case Invalid(errors) => failWith(errors)
      }
    }

    "start the next Play" when {
      "both Players have Passed" in playingGame { game =>
        val dealer = game.dealer
        val pone = game.pone
        val kingDiamonds = game.card(King, Diamonds)
        val tenDiamonds = game.card(Ten, Diamonds)
        val kingSpades = game.card(King, Spades)
        game.validNel andThen
          playCard(pone, kingDiamonds) andThen asPlayingGame andThen
          playCard(dealer, tenDiamonds) andThen asPlayingGame andThen
          playCard(pone, kingSpades) andThen asPlayingGame andThen
          pass(dealer) andThen asPlayingGame andThen
          pass(pone) andThen asPlayingGame match
          case Valid(game) =>
            val plays = game.plays
            plays.passCount should be(0)
            plays.nextToPlay should be(dealer)
            plays.current should be(empty)
            plays.runningTotal should be(0)
            plays.previous should contain theSameElementsInOrderAs (
              Seq(
                Plays.Laid(pone, kingDiamonds),
                Plays.Laid(dealer, tenDiamonds),
                Plays.Laid(pone, kingSpades),
                Plays.Pass(dealer),
                Plays.Pass(pone)
              )
            )
          case Invalid(errors) => failWith(errors)
      }

      "current Play finished on 31" in playingGame { game =>
        val dealer = game.dealer
        val pone = game.pone
        val kingDiamonds = game.card(King, Diamonds)
        val tenDiamonds = game.card(Ten, Diamonds)
        val sevenSpades = game.card(Seven, Spades)
        val fourClubs = game.card(Four, Clubs)
        game.validNel andThen
          playCard(pone, kingDiamonds) andThen asPlayingGame andThen
          playCard(dealer, tenDiamonds) andThen asPlayingGame andThen
          playCard(pone, sevenSpades) andThen asPlayingGame andThen
          playCard(dealer, fourClubs) andThen asPlayingGame andThen
          pass(pone) andThen asPlayingGame andThen
          pass(dealer) andThen asPlayingGame match
          case Valid(game) =>
            val plays = game.plays
            plays.passCount should be(0)
            plays.nextToPlay should be(pone)
            plays.current should be(empty)
            plays.runningTotal should be(0)
            plays.previous should contain theSameElementsInOrderAs (
              Seq(
                Plays.Laid(pone, kingDiamonds),
                Plays.Laid(dealer, tenDiamonds),
                Plays.Laid(pone, sevenSpades),
                Plays.Laid(dealer, fourClubs),
                Plays.Pass(pone),
                Plays.Pass(dealer)
              )
            )
          case Invalid(errors) => failWith(errors)
      }
    }

    "be a WonGame" when {
      "winning point(s) scored in Play" in playingGame { game =>
        val dealer = game.dealer
        val pone = game.pone
        val kingDiamonds = game.card(King, Diamonds)
        val tenDiamonds = game.card(Ten, Diamonds)
        val sevenSpades = game.card(Seven, Spades)
        val fourClubs = game.card(Four, Clubs)

        def withScores(dealerScore: Score, poneScore: Score)(game: PlayingGame) =
          game.copy(scores = Map(game.dealer -> dealerScore, game.pone -> poneScore)).validNel

        game.validNel andThen
          withScores(Score(0, 120), Score(0, 120)) andThen
          playCard(pone, kingDiamonds) andThen asPlayingGame andThen
          playCard(dealer, tenDiamonds) andThen asPlayingGame andThen
          playCard(pone, sevenSpades) andThen asPlayingGame andThen
          playCard(dealer, fourClubs) andThen asPlayingGame andThen
          pass(pone) andThen asPlayingGame andThen
          pass(dealer) andThen asWonGame match
          case Valid(game) =>
            game.scores(dealer) should be(Score(120, 122))
            game.scores(pone) should be(Score(0, 120))
          case Invalid(errors) => failWith(errors)
      }
    }

    "regather Plays" when {
      "all Plays completed" in fullyPlayedGame { game =>
        val dealer = game.dealer
        val pone = game.pone
        game.validNel andThen
          regatherPlays match
          case Valid(scoringGame) =>
            scoringGame.id should be(game.id)
            scoringGame.scores should be(game.scores)
            scoringGame.hands(dealer).size should be(4)
            scoringGame.hands(pone).size should be(4)
            scoringGame.dealer should be(game.dealer)
            scoringGame.pone should be(game.pone)
            scoringGame.crib should be(game.crib)
            scoringGame.cut should be(game.cut)
          case Invalid(errors) => failWith(errors)
      }
    }

    "not regather Plays" when {
      "some Cards left to Play" in playingGame { game =>
        game.validNel andThen
          regatherPlays match
          case Valid(game)     => fail(s"Incorrectly regathered Plays with Cards left to Play")
          case Invalid(errors) => passIfMatched(errors, List(s"There are still cards left to play in ${game.id}"))
      }
    }

  }
