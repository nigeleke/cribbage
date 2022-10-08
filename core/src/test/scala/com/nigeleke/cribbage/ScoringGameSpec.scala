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
    def scoringState(
                     dealerScore: Score = Score.zero,
                     poneScore: Score = Score.zero
                   )(
      test: Scoring => Unit
    ) =
      val (dealer, pone) = (Player.createPlayer, Player.createPlayer)
      val state          =
        Scoring(
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
      test(state)
      // format: on

    "score the Pone's Hand" when {
      "non-winning points" in scoringState() { case state @ Scoring(_, _, _, pone, _, _) =>
        Game(state).scorePoneHand match
          case Right(Game(Scoring(scores, _, _, _, _, _))) => scores(pone).points should be(4)
          case other                                       => fail(s"Unexpected state $other")
      }

      "winning points" in scoringState(Score(0, 120), Score(0, 117)) {
        case state @ Scoring(_, _, _, pone, _, _) =>
          Game(state).scorePoneHand match
            case Right(Game(Finished(scores))) => scores(pone).points should be(121)
            case other                         => fail(s"Unexpected state $other")
      }
    }

    "score the Dealer's Hand" when {
      "non-winning points" in scoringState() { case state @ Scoring(_, _, dealer, _, _, _) =>
        Game(state).scoreDealerHand match
          case Right(Game(Scoring(scores, _, _, _, _, _))) => scores(dealer).points should be(10)
          case other                                       => fail(s"Unexpected state $other")
      }

      "winning points" in scoringState(Score(0, 111), Score(0, 120)) {
        case state @ Scoring(_, _, dealer, _, _, _) =>
          Game(state).scoreDealerHand match
            case Right(Game(Finished(scores))) => scores(dealer).points should be(121)
            case other                         => fail(s"Unexpected state $other")
      }
    }

    "score the Crib" when {
      "non-winning points" in scoringState() { case state @ Scoring(_, _, dealer, _, _, _) =>
        Game(state).scoreCrib match
          case Right(Game(Scoring(scores, _, _, _, _, _))) => scores(dealer).points should be(4)
          case other                                       => fail(s"Unexpected state $other")
      }

      "winning points" in scoringState(Score(0, 117), Score(0, 120)) {
        case state @ Scoring(_, _, dealer, _, _, _) =>
          Game(state).scoreCrib match
            case Right(Game(Finished(scores))) => scores(dealer).points should be(121)
            case other                         => fail(s"Unexpected state $other")
      }
    }

    "report scores for existing players" in scoringState(Score(0, 10), Score(0, 20)) {
      case state @ Scoring(_, _, dealer, pone, _, _) =>
        Game(state).scores(dealer) should be(Score(0, 10))
        Game(state).scores(pone) should be(Score(0, 20))
    }

    "report scores for non-existing players as zero" in scoringState(Score(0, 10), Score(0, 20)) {
      case state: Scoring =>
        val player = Player.createPlayer
        Game(state).scores(player) should be(Score.zero)
    }

    "swap the Dealer" in scoringState() { case state @ Scoring(_, _, dealer0, pone0, _, _) =>
      Game(state).swapDealer match
        case Right(Game(Discarding(_, _, _, dealer1, pone1, _))) =>
          dealer1 should be(pone0)
          pone1 should be(dealer0)
        case other                                               => fail(s"Unexpected state $other")
    }

    "deal new Hands" in scoringState() { case state @ Scoring(_, _, dealer0, pone0, _, _) =>
      Game(state).swapDealer match
        case Right(Game(Discarding(_, _, hands, _, _, crib))) =>
          hands.values.map(_.size) should be(Seq(6, 6))
          crib.size should be(0)
        case other                                            => fail(s"Unexpected state $other")
    }

    "will not declare winner in an unfinished game" in scoringState() { state =>
      Game(state).winner should be(None)
      Game(state).loser should be(None)
    }

  }
