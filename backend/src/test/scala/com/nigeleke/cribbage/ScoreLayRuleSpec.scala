package com.nigeleke.cribbage

import com.nigeleke.cribbage.model.{ Cards, Status }
import com.nigeleke.cribbage.model.Face
import com.nigeleke.cribbage.model.Face._
import com.nigeleke.cribbage.model.Suit
import com.nigeleke.cribbage.model.Suit._
import com.nigeleke.cribbage.TestModel._
import com.nigeleke.cribbage.actors.Game.{ PlayCompleted, PointsScored }
import com.nigeleke.cribbage.actors.handlers.CommandHandler
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

class ScoreLayRuleSpec extends AnyWordSpec with Matchers {

  def checkPlays(plays: Seq[(Seq[(Face, Suit)], Int)]) = plays.foreach { play =>
    val faceSuits = play._1
    val expectedScore = play._2
    assertScore(faceSuits, expectedScore)
  }

  def assertScore(cards: Seq[(Face, Suit)], expectedScore: Int) = {

    def takeAlternate(cards: Cards): Cards = cards.toList match {
      case Nil => Nil
      case card1 :: Nil => Seq(card1)
      case card1 :: _ :: rest => card1 +: takeAlternate(rest)
    }

    val playerIds = Seq(player1Id, player2Id)
    val initialCards = cardsOf(cards)
    val initialGame = Status(randomId)
      .withPlayer(player1Id)
      .withPlayer(player2Id)
      .withDealer(player1Id)
      .withZeroScores()
      .withDeal(
        Map(
          (playerIds.head, takeAlternate(initialCards)),
          (playerIds.last, takeAlternate(initialCards.drop(1)))),
        initialCards)
      .withNextToLay(player2Id)
    val lays = initialCards.zip(Iterator.continually(playerIds).flatten)

    val game = lays.foldLeft(initialGame)((g, lay) => g.withLay(lay._2, lay._1))
    val endOfPlay = game.play.runningTotal == 31

    CommandHandler.scoreLay(game) should be {
      (expectedScore, endOfPlay) match {
        case (0, _) => Seq.empty
        case (score, false) => Seq(PointsScored(lays.last._2, score))
        case (score, true) => Seq(PointsScored(lays.last._2, score), PlayCompleted)
      }
    }
  }

  "The ScoreLayRule" should {

    "pegs totals of fifteen in a play" in {
      val plays = Seq(
        Seq((Ten, Clubs), (Five, Clubs)) -> 2,
        Seq((Jack, Clubs), (Five, Clubs)) -> 2,
        Seq((Nine, Clubs), (Six, Clubs)) -> 2,
        Seq((Eight, Clubs), (Seven, Clubs)) -> 2,
        Seq((Ace, Clubs), (Four, Clubs), (Ten, Clubs)) -> 2,
        Seq((Two, Clubs), (Three, Clubs), (Ten, Clubs)) -> 2,
        Seq((Two, Clubs), (Three, Clubs), (Ace, Clubs), (Three, Spades), (Ace, Spades), (Five, Hearts)) -> 2,
        Seq((Two, Clubs), (Three, Clubs), (Ace, Clubs), (Three, Spades), (Ace, Spades), (Six, Hearts)) -> 0)
      checkPlays(plays)
    }

    "pegs pairs in a play" in {
      val plays = Seq(
        Seq((Ten, Clubs), (Ten, Hearts)) -> 2,
        Seq((Ace, Clubs), (Ace, Hearts)) -> 2,
        Seq((Ten, Spades), (Jack, Spades)) -> 0,
        Seq((Ace, Clubs), (Ten, Clubs), (Ten, Hearts)) -> 2)
      checkPlays(plays)
    }

    "pegs repeated pairs in a play" in {
      val plays = Seq(
        Seq((Ace, Clubs), (Ace, Hearts)) -> 2,
        Seq((Ace, Clubs), (Ace, Hearts), (Ace, Diamonds)) -> 6,
        Seq((Ace, Clubs), (Ace, Hearts), (Ace, Diamonds), (Ace, Spades)) -> 12,
        Seq((Two, Diamonds), (Ace, Clubs), (Ace, Hearts)) -> 2,
        Seq((Two, Diamonds), (Ace, Clubs), (Ace, Hearts), (Ace, Diamonds)) -> 6,
        Seq((Two, Diamonds), (Ace, Clubs), (Ace, Hearts), (Ace, Diamonds), (Ace, Spades)) -> 12,
        Seq((Two, Diamonds), (Ace, Clubs), (Two, Hearts), (Ace, Hearts)) -> 0,
        Seq((Two, Diamonds), (Ace, Clubs), (Two, Hearts), (Ace, Hearts), (Ace, Diamonds)) -> 2,
        Seq((Two, Diamonds), (Ace, Clubs), (Ace, Hearts), (Two, Hearts), (Ace, Diamonds)) -> 0,
        Seq((Two, Diamonds), (Ace, Clubs), (Two, Hearts), (Ace, Hearts), (Ace, Diamonds), (Ace, Spades)) -> 6,
        Seq((Two, Diamonds), (Ace, Clubs), (Ace, Hearts), (Two, Hearts), (Ace, Diamonds), (Ace, Spades)) -> 2,
        Seq((Two, Diamonds), (Ace, Clubs), (Ace, Hearts), (Ace, Diamonds), (Two, Hearts), (Ace, Spades)) -> 0)
      checkPlays(plays)
    }

    "pegs runs formed during a play" in {
      val plays = Seq(
        Seq((Ace, Clubs), (Two, Hearts), (Three, Diamonds)) -> 3,
        Seq((Ace, Clubs), (Three, Diamonds), (Two, Hearts)) -> 3,
        Seq((Four, Spades), (Ace, Clubs), (Three, Diamonds), (Two, Hearts)) -> 4,
        Seq((Four, Spades), (Ace, Clubs), (Three, Diamonds), (Five, Hearts)) -> 0,
        Seq((Five, Spades), (Two, Clubs), (Four, Diamonds), (Six, Hearts), (Three, Clubs)) -> 5)
      checkPlays(plays)
    }

  }

}
