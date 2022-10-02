package com.nigeleke.cribbage

import model.*
import model.Card.Face.*
import GameState.*

import org.scalatest.*
import org.scalatest.matchers.should.*
import org.scalatest.wordspec.*

import scala.util.Random

extension (state: Discarding)
  def usingPlayers =
    val shuffled = Random.shuffle(Seq(state.dealer, state.pone))
    (shuffled.head, shuffled.last)

class DiscardingGameSpec extends AnyWordSpec with Matchers:

  def discardingState(
      dealerScore: Score = Score.zero,
      poneScore: Score = Score.zero
  )(
      test: Discarding => Unit
  ) =
    val players        = Set(Player.createPlayer, Player.createPlayer)
    val cut            = Random.shuffle(players)
    val (dealer, pone) = (cut.head, cut.last)
    val scores         = Map(dealer -> dealerScore, pone -> poneScore)
    val (deck, hands)  = Deck.shuffledDeck.deal(players.size, cardsPerHand)
    val dealtHands     = players.zip(hands).toMap
    val crib           = Crib.empty
    test(Discarding(deck, scores, dealtHands, dealer, pone, crib))

  "A DiscardingGame" should {

    "allow a player to discard cards into the crib" in discardingState() { state =>
      val (player, _) = state.usingPlayers
      val discards    = state.hands(player).take(2)
      Game(state).discardCribCards(player, discards) match
        case Right(Game(Discarding(_, _, hands, _, _, crib))) =>
          hands(player) should not contain allElementsOf(discards)
          crib should contain allElementsOf (discards)
        case other                                            => fail(s"Unexpected state $other")
    }

    "not allow a discard" when {

      "the discard contains cards not owned by the player" in discardingState() { state =>
        val (player1, player2) = state.usingPlayers
        val discards           = state.hands(player1).take(2)
        Game(state).discardCribCards(player2, discards) match
          case Left(error) => error should be("Cannot discard cards")
          case other       => fail(s"Unexpected state $other")
      }

      "the discard contains too many cards" in discardingState() { state =>
        val (player, _) = state.usingPlayers
        val discards    = state.hands(player).take(3)
        Game(state).discardCribCards(player, discards) match
          case Left(error) => error should be("Cannot discard cards")
          case other       => fail(s"Unexpected state $other")
      }
    }

    "allow the Play to start" when {

      "both Players have discarded" in discardingState() {
        case state @ Discarding(_, _, hands0, dealer0, pone0, _) =>
          val dealerDiscards = hands0(dealer0).take(2)
          val poneDiscards   = hands0(pone0).take(2)
          Game(state)
            .discardCribCards(dealer0, dealerDiscards)
            .flatMap(_.discardCribCards(pone0, poneDiscards))
            .flatMap(_.startPlay(pone0)) match
            case Right(Game(Playing(scores, hands1, dealer1, pone1, crib, cut, plays))) =>
              dealer1 should be(dealer0)
              pone1 should be(pone0)
              hands1(dealer1) ++ dealerDiscards should contain allElementsOf (hands0(dealer0))
              hands1(pone1) ++ poneDiscards should contain allElementsOf (hands0(pone0))
              plays.nextPlayer should be(state.pone)
              plays.inPlay should be(empty)
              plays.passCount should be(0)
              plays.played should be(empty)
              crib should contain allElementsOf (dealerDiscards ++ poneDiscards)
              if cut.face == Jack
              then scores(dealer1) should be(Score(0, 2))
              else scores(dealer1) should be(Score(0, 0))
            case other                                                                  => fail(s"Unexpected state $other")
      }
    }

    "be a WonGame" when {
      "winning point(s) scored for his Heels" in discardingState(Score(0, 120), Score(0, 120)) {
        case state @ Discarding(deck, scores, hands, dealer, pone, crib) =>
          // This test won't always test the condition as the cut is random.
          val discards1 = hands(dealer).take(2)
          val discards2 = hands(pone).take(2)
          Game(state)
            .discardCribCards(dealer, discards1)
            .flatMap(_.discardCribCards(pone, discards2))
            .flatMap(_.startPlay(pone)) match
            case Right(Game(Playing(_, _, _, _, _, cut, _))) => cut.face should not be (Jack)
            case Right(Game(Finished(scores)))               => scores(dealer) should be(Score(120, 122))
            case other                                       => fail(s"Unexpected state $other")
      }
    }
  }
