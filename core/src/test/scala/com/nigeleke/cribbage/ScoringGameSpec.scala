package com.nigeleke.cribbage

import model.*
import model.Card.{Face, Suit}
import Face.*, Suit.*
import model.GameState.*

import org.scalatest.*
import org.scalatest.matchers.should.*
import org.scalatest.wordspec.*

class ScoringGameSpec extends AnyWordSpec with Matchers:

  "A ScoringGame" should {

    // format: off
    def scoringGame(
                     dealerScore: Score = Score.zero,
                     poneScore: Score = Score.zero
                   )(
      test: Game => Unit
    ) =
      val (dealer, pone) = (Player.createPlayer, Player.createPlayer)
      val state          = Scoring(
        Map(dealer -> dealerScore, pone -> poneScore),
        Map(
          dealer -> Seq(Card(Ten, Diamonds), Card(Ten, Spades), Card(Five, Hearts), Card(Four, Clubs)),
          pone   -> Seq(Card(King, Diamonds), Card(King, Spades), Card(Eight, Diamonds), Card(Seven, Spades))
        ),
        dealer,
        pone,
        Seq(Card(Ten, Hearts), Card(Ten, Clubs), Card(King, Hearts), Card(King, Clubs)),
        Card(Ace, Spades)
      )
      test(Game(state))
      // format: on

    "score the Pone's Hand" when {
      "non-winning points" in scoringGame() { game =>
        game.state match
          case Scoring(_, _, _, pone, _, _) =>
            game.scorePoneHand match
              case Right(game) =>
                game.state match
                  case Scoring(scores, _, _, _, _, _) => scores(pone).points should be(4)
                  case _                              => fail(s"Unexpected state $game")
              case Left(error) => fail(error)
          case _                            => fail(s"Unexpected state $game")
      }

      "winning points" in scoringGame(Score(0, 120), Score(0, 117)) { game =>
        game.state match
          case Scoring(_, _, _, pone, _, _) =>
            game.scorePoneHand match
              case Right(game) =>
                game.state match
                  case Finished(scores) => scores(pone).points should be(121)
                  case _                => fail(s"Unexpected state $game")
              case Left(error) => fail(error)
          case _                            => fail(s"Unexpected state $game")
      }
    }

    "score the Dealer's Hand" when {
      "non-winning points" in scoringGame() { game =>
        game.state match
          case Scoring(_, _, dealer, _, _, _) =>
            game.scoreDealerHand match
              case Right(game) =>
                game.state match
                  case Scoring(scores, _, _, _, _, _) => scores(dealer).points should be(10)
                  case _                              => fail(s"Unexpected state $game")
              case Left(error) => fail(error)
          case _                              => fail(s"Unexpected state $game")
      }

      "winning points" in scoringGame(Score(0, 111), Score(0, 120)) { game =>
        game.state match
          case Scoring(_, _, dealer, _, _, _) =>
            game.scoreDealerHand match
              case Right(game) =>
                game.state match
                  case Finished(scores) => scores(dealer).points should be(121)
                  case _                => fail(s"Unexpected state $game")
              case Left(error) => fail(error)
          case _                              => fail(s"Unexpected state $game")
      }
    }

    "score the Crib" when {
      "non-winning points" in scoringGame() { game =>
        game.state match
          case Scoring(_, _, dealer, _, _, _) =>
            game.scoreCrib match
              case Right(game) =>
                game.state match
                  case Scoring(scores, _, _, _, _, _) => scores(dealer).points should be(4)
                  case _                              => fail(s"Unexpected state $game")
              case Left(error) => fail(error)
          case _                              => fail(s"Unexpected state $game")
      }

      "winning points" in scoringGame(Score(0, 117), Score(0, 120)) { game =>
        game.state match
          case Scoring(_, _, dealer, _, _, _) =>
            game.scoreCrib match
              case Right(game) =>
                game.state match
                  case Finished(scores) => scores(dealer).points should be(121)
                  case _                => fail(s"Unexpected state $game")
              case Left(error) => fail(error)
          case _                              => fail(s"Unexpected state $game")
      }
    }

    "swap the Dealer" in scoringGame() { game0 =>
      game0.state match
        case Scoring(_, _, dealer0, pone0, _, _) =>
          game0.swapDealer match
            case Right(game1) =>
              game1.state match
                case Discarding(_, _, _, dealer1, pone1, _) =>
                  dealer1 should be(pone0)
                  pone1 should be(dealer0)
                case _                                      => fail(s"Unexpected state $game0")
            case Left(error)  => fail(error)
        case _                                   => fail(s"Unexpected state $game0")
    }

    "deal new Hands" in scoringGame() { game =>
      game.state match
        case Scoring(_, _, dealer0, pone0, _, _) =>
          game.swapDealer match
            case Right(game) =>
              game.state match
                case Discarding(_, _, hands, _, _, crib) =>
                  hands.values.map(_.size) should be(Seq(6, 6))
                  crib.size should be(0)
                case _                                   => fail(s"Unexpected state $game")
            case Left(error) => fail(error)
        case _                                   => fail(s"Unexpected state $game")
    }

  }
