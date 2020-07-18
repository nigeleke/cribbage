package com.nigeleke.cribbage

import com.nigeleke.cribbage.actors.Game.{PegScore, Playing}
import com.nigeleke.cribbage.actors.rules.ScorePlayRule
import com.nigeleke.cribbage.model.{Card, Cards, Game}
import com.nigeleke.cribbage.suit.Face
import com.nigeleke.cribbage.suit.Face._
import com.nigeleke.cribbage.suit.Suit
import com.nigeleke.cribbage.suit.Suit._
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

class ScorePlayRuleSpec extends AnyWordSpec with Matchers {

  "The ScorePlayRule" should {

    def checkPlays(plays: Seq[(Seq[(Face, Suit)], Int)]) = plays.foreach { play =>
      val faceSuits = play._1
      val expectedScore = play._2
      scorePlay(faceSuits, expectedScore)
    }

    def scorePlay(cards: Seq[(Face, Suit)], expectedScore: Int) = {

      def takeAlternate(cards: Cards) : Cards = cards match {
        case Nil => Nil
        case card1 :: Nil => Seq(card1)
        case card1 :: _ :: rest => card1 +: takeAlternate(rest)
      }

      val playerIds = Seq(randomId, randomId)
      val initialCards = cards.map(card => Card(randomId, card._1, card._2))
      val initialGame = Game(randomId)
        .withDeck(initialCards)
        .withPlayer(playerIds.head).withHand(playerIds.head, takeAlternate(initialCards))
        .withPlayer(playerIds.last).withHand(playerIds.last, takeAlternate(initialCards.drop(1)))
      val lays = initialCards.zip(Iterator.continually(playerIds).flatten)

      val game = lays.foldLeft(initialGame)((g, lay) => g.withPlay(lay._2, lay._1))

      ScorePlayRule.commands(Playing(game)) should be(
        if (expectedScore != 0) Seq(PegScore(lays.last._2, expectedScore))
        else Seq.empty)
    }

    "pegs totals of fifteen in a play" in {
      val plays = Seq(
        Seq((Ten, Clubs), (Five, Clubs)) -> 2,
        Seq((Jack, Clubs), (Five, Clubs)) -> 2,
        Seq((Nine, Clubs), (Six, Clubs)) -> 2,
        Seq((Eight, Clubs), (Seven, Clubs)) -> 2,
        Seq((Ace, Clubs), (Four, Clubs), (Ten, Clubs)) -> 2,
        Seq((Two, Clubs), (Three, Clubs), (Ten, Clubs)) -> 2,
        Seq((Two, Clubs), (Three, Clubs), (Ace, Clubs), (Three, Spades), (Ace, Spades), (Five, Hearts)) -> 2,
        Seq((Two, Clubs), (Three, Clubs), (Ace, Clubs), (Three, Spades), (Ace, Spades), (Six, Hearts)) -> 0
      )
      checkPlays(plays)
    }

    "pegs pairs in a play" in {
      val plays = Seq(
        Seq((Ten, Clubs), (Ten, Hearts)) -> 2,
        Seq((Ace, Clubs), (Ace, Hearts)) -> 2,
        Seq((Ten, Spades), (Jack, Spades)) -> 0,
        Seq((Ace, Clubs), (Ten, Clubs), (Ten, Hearts)) -> 2
      )
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
        Seq((Two, Diamonds), (Ace, Clubs), (Ace, Hearts), (Ace, Diamonds), (Two, Hearts), (Ace, Spades)) -> 0
      )
      checkPlays(plays)
    }

    "pegs runs formed during a play" in {
      val plays = Seq(
        Seq((Ace, Clubs), (Two, Hearts), (Three, Diamonds)) -> 3,
        Seq((Ace, Clubs), (Three, Diamonds), (Two, Hearts)) -> 3,
        Seq((Four, Spades), (Ace, Clubs), (Three, Diamonds), (Two, Hearts)) -> 4,
        Seq((Four, Spades), (Ace, Clubs), (Three, Diamonds), (Five, Hearts)) -> 0,
        Seq((Five, Spades), (Two, Clubs), (Four, Diamonds), (Six, Hearts), (Three, Clubs)) -> 5
      )
      checkPlays(plays)
    }

    "pegs running total of 31" in {
      val plays = Seq(
        Seq((Ten, Hearts), (King, Clubs), (Ten, Diamonds), (Ace, Spades)) -> 2,
        Seq((Ten, Hearts), (King, Clubs), (Ten, Diamonds)) -> 0
      )
      checkPlays(plays)
    }

  }

}
