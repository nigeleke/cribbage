package com.nigeleke.cribbage

import model.*
import model.Card.*
import Face.*, Suit.*
import GameState.*

import org.scalatest.*
import org.scalatest.matchers.should.*
import org.scalatest.wordspec.*

class PlayingGameSpec extends AnyWordSpec with Matchers:

  def playingState(
      dealerScore: Score = Score.zero,
      poneScore: Score = Score.zero
  )(
      test: Playing => Unit
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
    test(state)
    // format: on

  def playingStateFinished(test: Playing => Unit) =
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
    test(state)
    // format: on

  "A PlayingGame" should {
    "allow the next Player to Play" when {
      "they have at least one valid cardId for the CurrentPlay" in playingState() {
        case state @ Playing(_, hands, dealer, pone, _, _, _) =>
          val card = hands(pone).head
          Game(state).playCard(pone, card) match
            case Right(Game(Playing(_, hands, _, _, _, _, plays))) =>
              hands(pone) should not contain (card)
              plays.inPlay.head should be(Plays.Laid(pone, card))
              plays.nextPlayer should be(dealer)
            case other                                             => fail(s"Unexpected state $other")
      }
    }

    "not allow the next Player to Play" when {
      "it's not their turn" in playingState() {
        case state @ Playing(_, hands, dealer, _, _, _, _) =>
          val card = hands(dealer).head
          Game(state).playCard(dealer, card) match
            case Left(error) => error should be(s"Cannot play card")
            case other       => fail(s"Unexpected state $other")
      }

      "they have no valid cards for the current play" in playingState() {
        case state @ Playing(_, hands, dealer, pone, _, _, _) =>
          Game(state)
            .playCard(pone, Card(King, Diamonds))
            .flatMap(_.playCard(dealer, Card(Ten, Diamonds)))
            .flatMap(_.playCard(pone, Card(King, Spades)))
            .flatMap(_.playCard(dealer, Card(Five, Hearts))) match
            case Left(error) => error should be(s"Cannot play card")
            case other       => fail(s"Unexpected state $other")
      }
    }

    "allow the next Player to Pass" when {
      "they have no valid cards for the CurrentPlay" in playingState() {
        case state @ Playing(_, _, dealer, pone, _, _, _) =>
          Game(state)
            .playCard(pone, Card(King, Diamonds))
            .flatMap(_.playCard(dealer, Card(Ten, Diamonds)))
            .flatMap(_.playCard(pone, Card(King, Spades)))
            .flatMap(_.pass(dealer)) match
            case Right(Game(Playing(_, _, _, _, _, _, plays))) =>
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
            case other                                         => fail(s"Unexpected state $other")
      }
    }

    "not allow the next Player to Pass" when {
      "they have at least one valid Card for the current Play" in playingState() {
        case state @ Playing(scores, hands, dealer, pone, crib, cut, plays) =>
          Game(state).pass(pone) match
            case Left(error) => error should be("Cannot pass")
            case other       => fail(s"Unexpected state $other")
      }
    }

    "disallow discarding more cards to the crib" in playingState() {
      case state @ Playing(_, hands, _, pone, _, _, _) =>
        Game(state).discardCribCards(pone, hands(pone).take(2)) match
          case Left(error) => error should be("Cannot discard cards")
          case other       => fail(s"Unexpected state $other")
    }

    "disallow re-starting play" in playingState() { case state @ Playing(_, _, _, pone, _, _, _) =>
      Game(state).startPlay(pone) match
        case Left(error) => error should be("Cannot start play")
        case other       => fail(s"Unexpected state $other")
    }

    "score the Play" when {
      "a Card is laid" in playingState() { case state @ Playing(scores, _, dealer, pone, _, _, _) =>
        val initialDealerPoints = scores(dealer).points
        Game(state)
          .playCard(pone, Card(King, Diamonds))
          .flatMap(_.playCard(dealer, Card(Five, Hearts))) match
          case Right(game) =>
            game.scores(dealer).back should be(initialDealerPoints)
            game.scores(dealer).front should be(initialDealerPoints + 2)
          case other       => fail(s"Unexpected state $other")
      }
    }

    "score the end of Play" when {
      "play finishes with runningTotal less than 31" in playingState() {
        case state @ Playing(scores, hands, dealer, pone, crib, cut, plays) =>
          val initialPonePoints = scores(dealer).points
          Game(state)
            .playCard(pone, Card(King, Diamonds))
            .flatMap(_.playCard(dealer, Card(Ten, Diamonds)))
            .flatMap(_.playCard(pone, Card(King, Spades)))
            .flatMap(_.pass(dealer))
            .flatMap(_.pass(pone)) match
            case Right(game) =>
              game.scores(pone).back should be(initialPonePoints)
              game.scores(pone).front should be(initialPonePoints + 1)
            case other       => fail(s"Unexpected state $other")
      }

      "play finishes with runningTotal exactly 31" in playingState() {
        case state @ Playing(scores, _, dealer, pone, _, _, _) =>
          val initialDealerPoints = scores(dealer).points
          Game(state)
            .playCard(pone, Card(King, Diamonds))
            .flatMap(_.playCard(dealer, Card(Ten, Diamonds)))
            .flatMap(_.playCard(pone, Card(Seven, Spades)))
            .flatMap(_.playCard(dealer, Card(Four, Clubs)))
            .flatMap(_.pass(pone))
            .flatMap(_.pass(dealer)) match
            case Right(game) =>
              game.scores(dealer).back should be(initialDealerPoints)
              game.scores(dealer).front should be(initialDealerPoints + 2)
            case other       => fail(s"Unexpected state $other")
      }
    }

    "start the next Play" when {
      "both Players have Passed" in playingState() {
        case state @ Playing(scores, hands, dealer, pone, crib, cut, plays) =>
          Game(state)
            .playCard(pone, Card(King, Diamonds))
            .flatMap(_.playCard(dealer, Card(Ten, Diamonds)))
            .flatMap(_.playCard(pone, Card(King, Spades)))
            .flatMap(_.pass(dealer))
            .flatMap(_.pass(pone)) match
            case Right(Game(Playing(_, _, dealer, pone, _, _, plays))) =>
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
            case other                                                 => fail(s"Unexpected state $other")
      }

      "inPlay Play finished on 31" in playingState() {
        case state @ Playing(scores, hands, dealer, pone, crib, cut, plays) =>
          Game(state)
            .playCard(pone, Card(King, Diamonds))
            .flatMap(_.playCard(dealer, Card(Ten, Diamonds)))
            .flatMap(_.playCard(pone, Card(Seven, Spades)))
            .flatMap(_.playCard(dealer, Card(Four, Clubs)))
            .flatMap(_.pass(pone))
            .flatMap(_.pass(dealer)) match
            case Right(Game(Playing(scores, hands, dealer, pone, crib, cut, plays))) =>
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
            case other                                                               => fail(s"Unexpected state $other")
      }
    }

    "be a WonGame" when {
      "winning point(s) scored at end of Play" in playingState(Score(0, 120), Score(0, 120)) {
        case state @ Playing(_, _, dealer, pone, _, _, _) =>
          Game(state)
            .playCard(pone, Card(King, Diamonds))
            .flatMap(_.playCard(dealer, Card(Ten, Diamonds)))
            .flatMap(_.playCard(pone, Card(Seven, Spades)))
            .flatMap(_.playCard(dealer, Card(Four, Clubs)))
            .flatMap(_.pass(pone))
            .flatMap(_.pass(dealer)) match
            case Right(Game(Finished(scores))) =>
              scores(dealer) should be(Score(120, 122))
              scores(pone) should be(Score(0, 120))
            case other                         => fail(s"Unexpected state $other")
      }

      "winning point(s) scored during Play" in playingState(Score(0, 120), Score(0, 120)) {
        case state@Playing(_, _, dealer, pone, _, _, _) =>
          Game(state)
            .playCard(pone, Card(King, Diamonds))
            .flatMap(_.playCard(dealer, Card(Five, Hearts))) match
            case Right(Game(Finished(scores))) =>
              scores(dealer) should be(Score(120, 122))
              scores(pone) should be(Score(0, 120))
            case other => fail(s"Unexpected state $other")
      }

    }

    "regather Plays" when {
      "all Plays completed" in playingStateFinished {
        case state @ Playing(scores0, _, dealer0, pone0, crib0, cut0, _) =>
          Game(state).regatherPlays match
            case Right(Game(Scoring(scores1, hands1, dealer1, pone1, crib1, cut1))) =>
              scores1 should be(scores0)
              hands1(dealer1).size should be(4)
              hands1(pone1).size should be(4)
              dealer1 should be(dealer0)
              pone1 should be(pone0)
              crib1 should be(crib0)
              cut1 should be(cut0)
            case other                                                              => fail(s"Unexpected state $other")
      }
    }

    "not regather Plays" when {
      "some Cards left to Play" in playingState() { state =>
        Game(state).regatherPlays match
          case Left(error) => error should be("Cannot regather")
          case other       => fail(s"Unexpected state $other")
      }
    }

    "disallow Score action" when {
      "for pone hand" in playingState() { state =>
        Game(state).scorePoneHand match
          case Left(error) => error should be("Cannot score pone hand")
          case other       => fail(s"Unexpected state $other")
      }

      "for dealer hand" in playingState() { state =>
        Game(state).scoreDealerHand match
          case Left(error) => error should be("Cannot score dealer hand")
          case other       => fail(s"Unexpected state $other")
      }

      "for crib" in playingState() { state =>
        Game(state).scoreCrib match
          case Left(error) => error should be("Cannot score crib")
          case other       => fail(s"Unexpected state $other")
      }

      "swapping dealer" in playingState() { state =>
        Game(state).swapDealer match
          case Left(error) => error should be("Cannot swap dealer")
          case other       => fail(s"Unexpected state $other")
      }
    }
  }
