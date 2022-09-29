package com.nigeleke.cribbage

import com.nigeleke.cribbage.domain.*
import com.nigeleke.cribbage.domain.Card.Face.*
import com.nigeleke.cribbage.effects.*

import cats.data.NonEmptyList
import cats.data.Validated.*
import cats.data.ValidatedNel
import cats.syntax.validated.*

import org.scalatest.*
import org.scalatest.matchers.should.*
import org.scalatest.wordspec.*

import java.util.UUID
import scala.util.Random

extension (game: DiscardingGame)
  def randomPlayer =
    val players = Seq(game.dealer, game.pone)
    val index = Random.nextInt(Game.maxPlayers)
    val otherIndex = Game.maxPlayers - index - 1
    (players(index), players(otherIndex))

class DiscardingGameSpec extends AnyWordSpec with Matchers:

  def discardingGame(test: DiscardingGame => Unit) =
    val id1 = Player.newId
    val id2 = Player.newId
    StartingGame().validNel andThen
      addPlayer(id1) andThen
      addPlayer(id2) andThen
      start match
      case Valid(game)     => test(game)
      case Invalid(errors) => failWith(errors)

  "A DiscardingGame" should {
    "initially start with zero scores for players" in discardingGame { game =>
      game.scores(game.dealer) should be(Score(0, 0))
      game.scores(game.pone) should be(Score(0, 0))
    }

    "start with an empty Crib" in discardingGame { game =>
      game.crib should be(empty)
    }

    "start with dealt hands" in discardingGame { game =>
      val dealerHand = game.hands(game.dealer)
      val poneHand = game.hands(game.pone)
      val deck = game.deck
      dealerHand.size should be(Deck.cardsPerHandToDeal)
      poneHand.size should be(Deck.cardsPerHandToDeal)
      deck.size should be(Deck.shuffledDeck.size - Game.maxPlayers * Deck.cardsPerHandToDeal)
      dealerHand.toSet intersect deck.toSet should be(empty)
      poneHand.toSet intersect deck.toSet should be(empty)
      dealerHand.toSet intersect poneHand.toSet should be(empty)
    }

    "allow a player to discard cards into the crib" in discardingGame { game =>
      val (player, _) = game.randomPlayer
      val discards = game.hands(player).take(2)
      game.validNel andThen
        discardCribCards(player, discards) match
        case Valid(game) =>
          game.hands(player) should not contain allElementsOf(discards)
          game.crib should contain allElementsOf (discards)
        case Invalid(errors) => failWith(errors)
    }

    "not allow a discard" when {

      "the discard contains cards not owned by the player" in discardingGame { game =>
        val (player1, player2) = game.randomPlayer
        val discards = game.hands(player1).take(2)

        game.validNel andThen
          discardCribCards(player2, discards) match
          case Valid(game)     => fail(s"Incorrectly discarded $discards for $player2 in $game")
          case Invalid(errors) => passIfMatched(errors, List(s"Player ${player2} does not own card(s) (${discards.toString})"))
      }

      "the discard contains too few cards" in discardingGame { game =>
        val (player, _) = game.randomPlayer
        val discards = game.hands(player).take(1)
        game.validNel andThen
          discardCribCards(player, discards) match
          case Valid(game)     => fail(s"Incorrectly allowed single card discard $discards for $player in $game")
          case Invalid(errors) => passIfMatched(errors, List(s"Game ${game.id} expecting 2 cards discarded; have 1"))
      }

      "the discard contains too many cards" in discardingGame { game =>
        val (player, _) = game.randomPlayer
        val discards = game.hands(player).take(3)
        game.validNel andThen
          discardCribCards(player, discards) match
          case Valid(game)     => fail(s"Incorrectly allowed three card discard $discards for $player in $game")
          case Invalid(errors) => passIfMatched(errors, List(s"Game ${game.id} expecting 2 cards discarded; have 3"))
      }

    }

    "allow the Play to start" when {

      "both Players have discarded" in discardingGame { game =>
        val (dealer, pone) = game.randomPlayer
        val discards1 = game.hands(dealer).take(2)
        val discards2 = game.hands(pone).take(2)

        game.validNel andThen
          discardCribCards(dealer, discards1) andThen
          discardCribCards(pone, discards2) andThen
          startPlay(pone) match
          case Valid(game) =>
            game match
              case game: PlayingGame =>
                val plays = game.plays
                plays.nextToPlay should be(game.pone)
                plays.current should be(empty)
                plays.passCount should be(0)
                plays.previous should be(empty)
                game.crib should contain allElementsOf (discards1 ++ discards2)
                (game.cut, game.dealer) match
                  case (Card(_, Jack, _), dealer) => game.scores(dealer) should be(Score(0, 2))
                  case (_, dealer)                => game.scores(dealer) should be(Score(0, 0))
              case any => any should be(a[PlayingGame])
          case Invalid(errors) => failWith(errors)
      }
    }

    "be a WonGame" when {
      "winning point(s) scored for his Heels" in discardingGame { game =>
        // This test won't always test the condition as the cut is random.
        val (dealer, pone) = game.randomPlayer
        val discards1 = game.hands(dealer).take(2)
        val discards2 = game.hands(pone).take(2)

        def withScores(dealerScore: Score, poneScore: Score)(game: DiscardingGame) =
          game.copy(scores = Map(game.dealer -> dealerScore, game.pone -> poneScore)).validNel

        game.validNel andThen
          withScores(Score(0, 120), Score(0, 120)) andThen
          discardCribCards(dealer, discards1) andThen
          discardCribCards(pone, discards2) andThen
          startPlay(pone) match
          case Valid(game) =>
            game match
              case p: PlayingGame => p.cut.face should not be (Jack)
              case w: WonGame     => w.scores(pone) should be(Score(120, 122))
          case Invalid(errors) => failWith(errors)
      }
    }
  }
