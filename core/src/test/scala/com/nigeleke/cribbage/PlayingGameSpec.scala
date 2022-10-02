package com.nigeleke.cribbage

import model.*
import model.Card.*
import Face.*, Suit.*
import GameState.*

import org.scalatest.*
import org.scalatest.matchers.should.*
import org.scalatest.wordspec.*

class PlayingGameSpec extends AnyWordSpec with Matchers:

  def playingGame(
      dealerScore: Score = Score.zero,
      poneScore: Score = Score.zero
  )(
      test: Game => Unit
  ) =
    // format: off
    val (dealer, pone) = (Player.createPlayer, Player.createPlayer)
    val dealerHand = Seq(Card(Ten, Diamonds), Card(Ten, Spades), Card(Five, Hearts), Card(Four, Clubs))
    val poneHand   = Seq(Card(King, Diamonds), Card(King, Spades), Card(Eight, Diamonds), Card(Seven, Spades))
    val crib  = Seq(Card(Ten, Hearts), Card(Ten, Clubs), Card(King, Hearts), Card(King, Clubs))
    val cut   = Card(Ace, Spades)
    val state = Playing(
      Map(dealer -> dealerScore, pone -> poneScore),
      Map(dealer -> dealerHand, pone -> poneHand),
      dealer,
      pone,
      crib,
      cut,
      Plays(pone)
    )
    val game = Game(state)
    test(game)
    // format: on

  def fullyPlayedGame(test: Game => Unit) =
    // format: off
    val (dealer, pone) = (Player.createPlayer, Player.createPlayer)
    val crib  = Seq(Card(Ten, Hearts), Card(Ten, Clubs), Card(King, Hearts), Card(King, Clubs))
    val cut   = Card(Ace, Spades)
    val plays = Plays(
      pone,
      Seq(Plays.Laid(pone, Card(Seven, Spades))),
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
    val state = Playing(
      Map(dealer -> Score.zero, pone -> Score.zero),
      Map(dealer -> Hand.empty, pone -> Hand.empty),
      dealer,
      pone,
      crib,
      cut,
      plays
    )
    val game = Game(state)
    test(game)
    // format: on

  "A PlayingGame" should {
    "allow the next Player to Play" when {
      "they have at least one valid cardId for the CurrentPlay" in playingGame() { game =>
        game.state match
          case Playing(_, hands, dealer, pone, _, _, _) =>
            val card = hands(pone).head
            game.playCard(pone, card) match
              case Right(game) =>
                game.state match
                  case Playing(_, hands, _, _, _, _, plays) =>
                    hands(pone) should not contain (card)
                    plays.inPlay.head should be(Plays.Laid(pone, card))
                    plays.nextPlayer should be(dealer)
                  case _                                    => fail(s"Unexpected state $game")
              case Left(error) => fail(error)
          case _                                        => fail(s"Unexpected state $game")
      }
    }

    "not allow the next Player to Play" when {
      "it's not their turn" in playingGame() { game =>
        game.state match
          case Playing(_, hands, dealer, pone, _, _, _) =>
            val card = hands(dealer).head
            game.playCard(dealer, card) match
              case Right(game) => fail(s"Incorrectly allowed Play($dealer, $card)")
              case Left(error) => error should be(s"Cannot play")
          case _                                        => fail(s"Unexpected state $game")
      }

      "they have no valid cards for the current play" in playingGame() { game =>
        game.state match
          case Playing(_, hands, dealer, pone, _, _, _) =>
            game
              .playCard(pone, Card(King, Diamonds))
              .flatMap(_.playCard(dealer, Card(Ten, Diamonds)))
              .flatMap(_.playCard(pone, Card(King, Spades)))
              .flatMap(_.playCard(dealer, Card(Five, Hearts))) match
              case Right(game) => fail("Incorrectly allowed Play that would break 31")
              case Left(error) => error should be(s"Cannot play")
          case _                                        => fail(s"Unexpected state $game")
      }
    }

    "allow the next Player to Pass" when {
      "they have no valid cards for the CurrentPlay" in playingGame() { game =>
        game.state match
          case Playing(_, _, dealer, pone, _, _, _) =>
            game
              .playCard(pone, Card(King, Diamonds))
              .flatMap(_.playCard(dealer, Card(Ten, Diamonds)))
              .flatMap(_.playCard(pone, Card(King, Spades)))
              .flatMap(_.pass(dealer)) match
              case Right(game) =>
                game.state match
                  case Playing(_, _, _, _, _, _, plays) =>
                    val expectedPlays =
                      Plays(
                        pone,
                        Seq(
                          Plays.Laid(pone, Card(King, Diamonds)),
                          Plays.Laid(dealer, Card(Ten, Diamonds)),
                          Plays.Laid(pone, Card(King, Spades)),
                          Plays.Pass(dealer)
                        ),
                        Seq.empty
                      )
                    plays should be(expectedPlays)
                    plays.passCount should be(1)
                    plays.passedPlayers should contain(dealer)
                  case _                                => fail(s"Unexpected state $game")
              case Left(error) => fail(error)
          case _                                    => fail(s"Unexpected state $game")
      }
    }

    "not allow the next Player to Pass" when {
      "they have at least one valid Card for the current Play" in playingGame() { game =>
        game.state match
          case Playing(scores, hands, dealer, pone, crib, cut, plays) =>
            game.pass(pone) match
              case Right(game) => fail(s"Incorrectly allowed pass for $pone in $game")
              case Left(error) => error should be("Cannot pass")
          case _                                                      => fail(s"Unexpected state $game")
      }
    }

    "score the Play" when {
      "a Card is laid" in playingGame() { game =>
        game.state match
          case Playing(scores, _, dealer, pone, _, _, _) =>
            val initialDealerPoints = scores(dealer).points
            game
              .playCard(pone, Card(King, Diamonds))
              .flatMap(_.playCard(dealer, Card(Five, Hearts))) match
              case Right(game) =>
                game.scores(dealer).back should be(initialDealerPoints)
                game.scores(dealer).front should be(initialDealerPoints + 2)
              case Left(error) => fail(error)
          case _                                         => fail(s"Unexpected state $game")
      }
    }

    "score the end of Play" when {
      "play finishes with runningTotal less than 31" in playingGame() { game =>
        game.state match
          case Playing(scores, hands, dealer, pone, crib, cut, plays) =>
            val initialPonePoints = game.scores(dealer).points
            game
              .playCard(pone, Card(King, Diamonds))
              .flatMap(_.playCard(dealer, Card(Ten, Diamonds)))
              .flatMap(_.playCard(pone, Card(King, Spades)))
              .flatMap(_.pass(dealer))
              .flatMap(_.pass(pone)) match
              case Right(game) =>
                game.scores(pone).back should be(initialPonePoints)
                game.scores(pone).front should be(initialPonePoints + 1)
              case Left(error) => fail(error)
          case _                                                      => fail(s"Unexpected state $game")
      }

      "play finishes with runningTotal exactly 31" in playingGame() { game =>
        game.state match
          case Playing(scores, _, dealer, pone, _, _, _) =>
            val initialDealerPoints = scores(dealer).points
            game
              .playCard(pone, Card(King, Diamonds))
              .flatMap(_.playCard(dealer, Card(Ten, Diamonds)))
              .flatMap(_.playCard(pone, Card(Seven, Spades)))
              .flatMap(_.playCard(dealer, Card(Four, Clubs)))
              .flatMap(_.pass(pone))
              .flatMap(_.pass(dealer)) match
              case Right(game) =>
                game.scores(dealer).back should be(initialDealerPoints)
                game.scores(dealer).front should be(initialDealerPoints + 2)
              case Left(error) => fail(error)
          case _                                         => fail(s"Unexpected state $game")
      }
    }

    "start the next Play" when {
      "both Players have Passed" in playingGame() { game =>
        game.state match
          case Playing(scores, hands, dealer, pone, crib, cut, plays) =>
            game
              .playCard(pone, Card(King, Diamonds))
              .flatMap(_.playCard(dealer, Card(Ten, Diamonds)))
              .flatMap(_.playCard(pone, Card(King, Spades)))
              .flatMap(_.pass(dealer))
              .flatMap(_.pass(pone)) match
              case Right(game) =>
                game.state match
                  case Playing(_, _, dealer, pone, _, _, plays) =>
                    plays.passCount should be(0)
                    plays.nextPlayer should be(dealer)
                    plays.inPlay should be(empty)
                    plays.runningTotal should be(0)
                    plays.played should contain theSameElementsInOrderAs (
                      Seq(
                        Plays.Laid(pone, Card(King, Diamonds)),
                        Plays.Laid(dealer, Card(Ten, Diamonds)),
                        Plays.Laid(pone, Card(King, Spades)),
                        Plays.Pass(dealer),
                        Plays.Pass(pone)
                      )
                    )
                  case _                                        => fail(s"Unexpected state $game")
              case Left(error) => fail(error)
          case _                                                      => fail(s"Unexpected state $game")
      }

      "inPlay Play finished on 31" in playingGame() { game =>
        game.state match
          case Playing(scores, hands, dealer, pone, crib, cut, plays) =>
            game
              .playCard(pone, Card(King, Diamonds))
              .flatMap(_.playCard(dealer, Card(Ten, Diamonds)))
              .flatMap(_.playCard(pone, Card(Seven, Spades)))
              .flatMap(_.playCard(dealer, Card(Four, Clubs)))
              .flatMap(_.pass(pone))
              .flatMap(_.pass(dealer)) match
              case Right(game) =>
                game.state match
                  case Playing(scores, hands, dealer, pone, crib, cut, plays) =>
                    plays.passCount should be(0)
                    plays.nextPlayer should be(pone)
                    plays.inPlay should be(empty)
                    plays.runningTotal should be(0)
                    plays.played should contain theSameElementsInOrderAs (
                      Seq(
                        Plays.Laid(pone, Card(King, Diamonds)),
                        Plays.Laid(dealer, Card(Ten, Diamonds)),
                        Plays.Laid(pone, Card(Seven, Spades)),
                        Plays.Laid(dealer, Card(Four, Clubs)),
                        Plays.Pass(pone),
                        Plays.Pass(dealer)
                      )
                    )
                  case _                                                      => fail(s"Unexpected state $game")
              case Left(error) => fail(error)
          case _                                                      => fail(s"Unexpected state $game")
      }
    }

    "be a WonGame" when {
      "winning point(s) scored in Play" in playingGame(Score(0, 120), Score(0, 120)) { game0 =>
        game0.state match
          case Playing(_, _, dealer, pone, _, _, _) =>
            game0
              .playCard(pone, Card(King, Diamonds))
              .flatMap(_.playCard(dealer, Card(Ten, Diamonds)))
              .flatMap(_.playCard(pone, Card(Seven, Spades)))
              .flatMap(_.playCard(dealer, Card(Four, Clubs)))
              .flatMap(_.pass(pone))
              .flatMap(_.pass(dealer)) match
              case Right(game1) =>
                game1.state match
                  case Finished(scores) =>
                    scores(dealer) should be(Score(120, 122))
                    scores(pone) should be(Score(0, 120))
                  case _                => fail(s"Unexpected state $game1")
              case Left(error)  => fail(error)
          case _                                    => fail(s"Unexpected state $game0")
      }
    }

    "regather Plays" when {
      "all Plays completed" in fullyPlayedGame { game0 =>
        game0.state match
          case Playing(scores0, _, dealer0, pone0, crib0, cut0, _) =>
            game0.regatherPlays match
              case Right(game1) =>
                game1.state match
                  case Scoring(scores1, hands1, dealer1, pone1, crib1, cut1) =>
                    scores1 should be(scores0)
                    hands1(dealer1).size should be(4)
                    hands1(pone1).size should be(4)
                    dealer1 should be(dealer0)
                    pone1 should be(pone0)
                    crib1 should be(crib0)
                    cut1 should be(cut0)
                  case _                                                     => fail(s"Unexpected state $game1")
              case Left(error)  => fail(error)
          case _                                                   => fail(s"Unexpected state $game0")
      }
    }

    "not regather Plays" when {
      "some Cards left to Play" in playingGame() { game =>
        game.regatherPlays match
          case Right(game) => fail(s"Incorrectly regathered Plays with Cards left to Play")
          case Left(error) => error should be("Cannot regather")
      }
    }

  }
