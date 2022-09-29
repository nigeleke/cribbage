package com.nigeleke.cribbage

import com.nigeleke.cribbage.domain.*
import com.nigeleke.cribbage.domain.Card.*
import Face.*
import Suit.*
import com.nigeleke.cribbage.effects.*

import cats.data.NonEmptyList
import cats.data.Validated.*
import cats.data.ValidatedNel
import cats.syntax.validated.*

import org.scalatest.*
import org.scalatest.matchers.should.*
import org.scalatest.wordspec.*

import java.util.UUID

class ScoringGameSpec extends AnyWordSpec with Matchers:

  "A ScoringGame" should {

    def scoringGame(test: ScoringGame => Unit) =
      val dealer = Player.newId
      val pone = Player.newId
      val game = ScoringGame(
        UUID.randomUUID().asInstanceOf[Game.Id],
        Map(
          dealer -> Score.zero,
          pone -> Score.zero
        ),
        Map(
          dealer -> Seq(Card(Ten, Diamonds), Card(Ten, Spades), Card(Five, Hearts), Card(Four, Clubs)),
          pone -> Seq(Card(King, Diamonds), Card(King, Spades), Card(Eight, Diamonds), Card(Seven, Spades))
        ),
        dealer,
        pone,
        Seq(Card(Ten, Hearts), Card(Ten, Clubs), Card(King, Hearts), Card(King, Clubs)),
        Card(Ace, Spades)
      )
      test(game)

    def scoringGameWithScores(dealerScore: Score, poneScore: Score)(test: ScoringGame => Unit) = scoringGame { game =>
      test(game.copy(scores = Map(game.dealer -> dealerScore, game.pone -> poneScore)))
    }

    def asScoringGame(game: Game): ValidatedNel[String, ScoringGame] = game match
      case s: ScoringGame => s.validNel
      case _              => s"ScoringGame expected; have $game".invalidNel

    def asWonGame(game: Game): ValidatedNel[String, WonGame] = game match
      case w: WonGame => w.validNel
      case _          => s"WonGame expected; have $game".invalidNel

    "score the Pone's Hand" when {
      "non-winning points" in scoringGame { game =>
        val pone = game.pone
        game.validNel andThen
          scorePoneHand andThen asScoringGame match
          case Valid(game)     => game.scores(pone).points should be(4)
          case Invalid(errors) => failWith(errors)
      }

      "winning points" in scoringGameWithScores(Score(0, 120), Score(0, 117)) { game =>
        val pone = game.pone
        game.validNel andThen
          scorePoneHand andThen asWonGame match
          case Valid(game)     => game.scores(pone).points should be(121)
          case Invalid(errors) => failWith(errors)
      }
    }

    "score the Dealer's Hand" when {
      "non-winning points" in scoringGame { game =>
        val dealer = game.dealer
        game.validNel andThen
          scoreDealerHand andThen asScoringGame match
          case Valid(game)     => game.scores(dealer).points should be(10)
          case Invalid(errors) => failWith(errors)
      }

      "winning points" in scoringGameWithScores(Score(0, 111), Score(0, 120)) { game =>
        val dealer = game.dealer
        game.validNel andThen
          scoreDealerHand andThen asWonGame match
          case Valid(game)     => game.scores(dealer).points should be(121)
          case Invalid(errors) => failWith(errors)
      }
    }

    "score the Crib" when {
      "non-winning points" in scoringGame { game =>
        val dealer = game.dealer
        game.validNel andThen
          scoreCrib andThen asScoringGame match
          case Valid(game)     => game.scores(dealer).points should be(4)
          case Invalid(errors) => failWith(errors)
      }

      "winning points" in scoringGameWithScores(Score(0, 117), Score(0, 120)) { game =>
        val dealer = game.dealer
        game.validNel andThen
          scoreCrib andThen asWonGame match
          case Valid(game)     => game.scores(dealer).points should be(121)
          case Invalid(errors) => failWith(errors)
      }
    }

    "swap the Dealer" in scoringGame { game1 =>
      game1.validNel andThen
        swapDealer match
        case Valid(game2) =>
          game2.dealer should be(game1.pone)
          game2.pone should be(game1.dealer)
        case Invalid(errors) => failWith(errors)
    }

    "deal new Hands" in scoringGame { game =>
      game.validNel andThen
        swapDealer match
        case Valid(game) =>
          game.hands.values.map(_.size) should be(Seq(6, 6))
          game.crib.size should be(0)
        case Invalid(errors) => failWith(errors)
    }

  }
