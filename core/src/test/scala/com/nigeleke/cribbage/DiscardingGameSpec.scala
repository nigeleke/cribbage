package com.nigeleke.cribbage

import model.*
import model.Card.Face.*
import model.Card.Suit.*
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

    "not allow new players" in discardingState() { state =>
      val player = Player.createPlayer
      Game(state).addPlayer(player) match
        case Left(error) => error should be(s"Cannot add player $player")
        case other       => fail(s"Unexpected state $other")
    }

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

    "report scores for existing players" in discardingState(Score(0, 10), Score(0, 20)) {
      case state @ Discarding(_, _, _, dealer, pone, _) =>
        Game(state).scores(dealer) should be(Score(0, 10))
        Game(state).scores(pone) should be(Score(0, 20))
    }

    "report scores for non-existing players as zero" in discardingState(
      Score(0, 10),
      Score(0, 20)
    ) { case state: Discarding =>
      val player = Player.createPlayer
      Game(state).scores(player) should be(Score.zero)
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

    "not allow the Play to start" when {
      "not all cards have been discarded" in discardingState() {
        case state @ Discarding(_, _, hands, _, pone, _) =>
          val discards = hands(pone).take(2)
          Game(state)
            .discardCribCards(pone, discards)
            .flatMap(_.startPlay(pone)) match
            case Left(error) => error should be("Cannot start play")
            case other       => fail(s"Unexpected state $other")
      }
    }

    "be a WonGame" when {
      "winning point(s) scored for his Heels" in discardingState(Score(0, 120), Score(0, 120)) {
        case state @ Discarding(_, _, hands, dealer, pone, _) =>
          extension (game: Game)
            def forceJackCut: Game = game.state match
              case d: Discarding => Game(d.copy(deck = Seq(Card(Jack, Hearts))))
              case other         => Game(other)
          val discards1            = hands(dealer).take(2)
          val discards2            = hands(pone).take(2)
          Game(state)
            .discardCribCards(dealer, discards1)
            .flatMap(_.discardCribCards(pone, discards2))
            .flatMap(_.forceJackCut.startPlay(pone)) match
            case Right(Game(Finished(scores))) => scores(dealer) should be(Score(120, 122))
            case other                         => fail(s"Unexpected state $other")
      }
    }

    "disallow Play actions" when {
      "Player plays a card" in discardingState() {
        case state @ Discarding(_, _, hands, _, pone, _) =>
          Game(state)
            .playCard(pone, hands(pone).head) match
            case Left(error) => error should be("Cannot play card")
            case other       => fail(s"Unexpected state $other")
      }

      "Player passes" in discardingState() { case state @ Discarding(_, _, _, _, pone, _) =>
        Game(state)
          .pass(pone) match
          case Left(error) => error should be("Cannot pass")
          case other       => fail(s"Unexpected state $other")
      }

      "Plays to be regathered" in discardingState() { case state: Discarding =>
        Game(state).regatherPlays match
          case Left(error) => error should be("Cannot regather")
          case other       => fail(s"Unexpected state $other")
      }
    }
  }
