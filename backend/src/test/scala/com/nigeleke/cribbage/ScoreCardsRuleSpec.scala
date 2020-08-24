package com.nigeleke.cribbage

import com.nigeleke.cribbage.model.Face._
import com.nigeleke.cribbage.model.Suit._
import com.nigeleke.cribbage.TestModel._
import com.nigeleke.cribbage.actors.Game.{ CribScored, DealerScored, DealerSwapped, PoneScored }
import com.nigeleke.cribbage.actors.handlers.CommandHandler
import com.nigeleke.cribbage.model.{ Attributes, Points }
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

class ScoreCardsRuleSpec extends AnyWordSpec with Matchers {

  val zeroPoints = Points(flushes = 5) // Empty hand and single cut will give this result...

  def checkCards(hands: Seq[(Seq[FaceSuit], Points)], cut: FaceSuit) = hands.foreach { hand =>
    val faceSuits = hand._1
    val expectedPoints = hand._2
    assertScore(faceSuits, cut, expectedPoints)
  }

  def assertScore(cards: Seq[FaceSuit], cut: FaceSuit, expectedPoints: Points) = {
    val attributes = Attributes()
      .withPlayer(player1Id)
      .withPlayer(player2Id)
      .withDealer(player2Id)
      .withZeroScores()
      .withDeal(Map((player1Id, cardsOf(cards)), (player2Id, Seq.empty)), deck)
      .withCut(cardOf(cut))

    CommandHandler.scoreHands(attributes) should be(Seq(
      PoneScored(player1Id, expectedPoints),
      DealerScored(player2Id, zeroPoints),
      CribScored(player2Id, zeroPoints),
      DealerSwapped))
  }

  "The ScoreCardsRule" should {

    "peg cards with fifteens" in {
      val cut = (Queen, Hearts)
      val cards = Seq(
        Seq((Five, Clubs), (Nine, Hearts), (Four, Diamonds), (Two, Spades)) -> Points(fifteens = 4),
        Seq((Ace, Clubs), (Four, Clubs), (Ten, Clubs), (King, Hearts)) -> Points(fifteens = 6))
      checkCards(cards, cut)
    }

    "pegs cards with pairs" in {
      val cut = (Ace, Hearts)
      val cards = Seq(
        Seq((Five, Clubs), (Five, Hearts), (Ace, Diamonds), (Two, Spades)) -> Points(pairs = 4),
        Seq((Queen, Clubs), (Three, Clubs), (Three, Diamonds), (King, Hearts)) -> Points(pairs = 2),
        Seq((Four, Hearts), (Four, Clubs), (Four, Diamonds), (Five, Hearts)) -> Points(pairs = 6),
        Seq((Four, Hearts), (Four, Clubs), (Four, Diamonds), (Four, Spades)) -> Points(pairs = 12))
      checkCards(cards, cut)
    }

    "pegs cards with runs" in {
      val cut = (Ace, Diamonds)
      val cards = Seq(
        Seq((Two, Clubs), (Jack, Hearts), (Queen, Diamonds), (King, Spades)) -> Points(runs = 3),
        Seq((Jack, Clubs), (Jack, Hearts), (Queen, Diamonds), (King, Spades)) -> Points(pairs = 2, runs = 6),
        Seq((Ten, Clubs), (Jack, Hearts), (Queen, Diamonds), (King, Spades)) -> Points(runs = 4),
        Seq((Two, Hearts), (Two, Clubs), (Three, Diamonds), (Four, Spades)) -> Points(pairs = 2, runs = 8),
        Seq((Two, Hearts), (Five, Clubs), (Three, Diamonds), (Four, Spades)) -> Points(runs = 5, fifteens = 2))
      checkCards(cards, cut)
    }

    "pegs cards with one for his heels" in {
      val cut = (Ace, Hearts)
      val cards = Seq(
        Seq((Two, Clubs), (Jack, Hearts), (Ten, Diamonds), (Six, Spades)) -> Points(heels = 1),
        Seq((Two, Clubs), (Jack, Diamonds), (Ten, Diamonds), (Six, Spades)) -> Points())
      checkCards(cards, cut)
    }

    "pegs cards with flushes" in {
      val cut = (Ace, Diamonds)
      val cards = Seq(
        Seq((Two, Diamonds), (Seven, Diamonds), (Queen, Diamonds), (King, Diamonds)) -> Points(flushes = 5),
        Seq((Two, Hearts), (Seven, Hearts), (Queen, Hearts), (King, Hearts)) -> Points(flushes = 4),
        Seq((Two, Diamonds), (Seven, Diamonds), (Queen, Diamonds), (King, Clubs)) -> Points())
      checkCards(cards, cut)
    }
  }

}
