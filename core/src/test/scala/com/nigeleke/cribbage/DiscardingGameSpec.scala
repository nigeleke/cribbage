package com.nigeleke.cribbage

import model.*

import org.scalatest.*
import org.scalatest.matchers.should.*
import org.scalatest.wordspec.*

import scala.util.Random

extension (game: Game)
  def usingPlayers =
    val shuffled = Random.shuffle(game.players)
    (shuffled.head, shuffled.last)

class DiscardingGameSpec extends AnyWordSpec with Matchers:

  def discardingGame(
      dealerScore: Score = Score.zero,
      poneScore: Score = Score.zero
  )(
      test: Game => Unit
  ) =
    val players        = Set(Player.createPlayer, Player.createPlayer)
    val cut            = Random.shuffle(players)
    val (dealer, pone) = (cut.head, cut.last)
    val scores         = Map(dealer -> dealerScore, pone -> poneScore)
    val (deck, hands)  = Deck.shuffledDeck.deal(players.size, cardsPerHand)
    val dealtHands     = players.zip(hands).toMap
    val crib           = Crib.empty
    val state          = GameState.Discarding(deck, scores, dealtHands, dealer, pone, crib)
    test(Game(state))

  "A DiscardingGame" should {
    "initially start with zero scores for players" in discardingGame() { game =>
      game.state match
        case state: GameState.Discarding =>
          state.scores(state.dealer) should be(Score(0, 0))
          state.scores(state.pone) should be(Score(0, 0))
        case other                       => fail(s"Unexpected state $game")
    }

    "start with an empty crib" in discardingGame() { game =>
      game.state match
        case state: GameState.Discarding => state.crib should be(empty)
        case other                       => fail(s"Unexpected state $game")
    }

    "start with dealt hands" in discardingGame() { game =>
      game.state match
        case state: GameState.Discarding =>
          val dealerHand = state.hands(state.dealer)
          val poneHand   = state.hands(state.pone)
          val deck       = state.deck
          dealerHand.size should be(cardsPerHand)
          poneHand.size should be(cardsPerHand)
          deck.size should be(Deck.fullDeck.size - maxPlayers * cardsPerHand)
          dealerHand.toSet intersect deck.toSet should be(empty)
          poneHand.toSet intersect deck.toSet should be(empty)
          dealerHand.toSet intersect poneHand.toSet should be(empty)
        case other                       => fail(s"Unexpected state $game")
    }

    "allow a player to discard cards into the crib" in discardingGame() { game =>
      game.state match
        case state: GameState.Discarding =>
          val (player, _) = game.usingPlayers
          val discards    = state.hands(player).take(2)
          game.discardCribCards(player, discards) match
            case Right(game) =>
              val state = game.state.asInstanceOf[GameState.Discarding]
              state.hands(player) should not contain allElementsOf(discards)
              state.crib should contain allElementsOf (discards)
            case Left(error) => fail(error)
        case other                       => fail(s"Unexpected state $game")
    }

    "not allow a discard" when {

      "the discard contains cards not owned by the player" in discardingGame() { game =>
        game.state match
          case state: GameState.Discarding =>
            val (player1, player2) = game.usingPlayers
            val discards           = state.hands(player1).take(2)
            game.discardCribCards(player2, discards) match
              case Right(game) => fail(s"Incorrectly discarded $discards for $player2 in $game")
              case Left(error) => error should be("Cannot discard cards")
          case other                       => fail(s"Unexpected state $game")
      }

      "the discard contains too many cards" in discardingGame() { game =>
        game.state match
          case state: GameState.Discarding =>
            val (player, _) = game.usingPlayers
            val discards    = state.hands(player).take(3)
            game.discardCribCards(player, discards) match
              case Right(game) => fail(s"Incorrectly discarded $discards for $player in $game")
              case Left(error) => error should be("Cannot discard cards")
          case other                       => fail(s"Unexpected state $game")
      }

    }

    "allow the Play to start" when {

      "both Players have discarded" in discardingGame() { game =>
        extension (card: Card) def isJack = card.face == Card.Face.Jack
        game.state match
          case state: GameState.Discarding =>
            val (dealer, pone) = (state.dealer, state.pone)
            val discards1      = state.hands(dealer).take(2)
            val discards2      = state.hands(pone).take(2)
            game
              .discardCribCards(dealer, discards1)
              .flatMap(_.discardCribCards(pone, discards2))
              .flatMap(_.startPlay(pone)) match
              case Right(game) =>
                val state = game.state.asInstanceOf[GameState.Playing]
                val plays = state.plays
                plays.nextPlayer should be(state.pone)
                plays.inPlay should be(empty)
                plays.passCount should be(0)
                plays.played should be(empty)
                state.crib should contain allElementsOf (discards1 ++ discards2)
                (state.cut, state.dealer) match
                  case (card, dealer) if card.isJack => state.scores(dealer) should be(Score(0, 2))
                  case (_, dealer)                   => state.scores(dealer) should be(Score(0, 0))
              case Left(error) => fail(error)
          case other                       => fail(s"Unexpected state $game")
      }
    }

    "be a WonGame" when {
      "winning point(s) scored for his Heels" in
        discardingGame(Score(0, 120), Score(0, 120)) { game =>
          game.state match
            case d: GameState.Discarding =>
              // This test won't always test the condition as the cut is random.
              val (dealer, pone) = (d.dealer, d.pone)
              val discards1      = d.hands(dealer).take(2)
              val discards2      = d.hands(pone).take(2)
              game
                .discardCribCards(dealer, discards1)
                .flatMap(_.discardCribCards(pone, discards2))
                .flatMap(_.startPlay(pone)) match
                case Right(game) =>
                  game.state match
                    case p: GameState.Playing  => p.cut.face should not be (Card.Face.Jack)
                    case f: GameState.Finished => f.scores(dealer) should be(Score(120, 122))
                    case _                     => fail(s"Unexpected state $game")
                case Left(error) => fail(error)
            case other                   => fail(s"Unexpected state $game")
        }
    }
  }
