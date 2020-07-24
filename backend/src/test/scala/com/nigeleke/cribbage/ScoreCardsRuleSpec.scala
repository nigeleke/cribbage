package com.nigeleke.cribbage

import com.nigeleke.cribbage.actors.Game.PegScore
import com.nigeleke.cribbage.actors.rules.Rules._
import com.nigeleke.cribbage.suit.Face._
import com.nigeleke.cribbage.suit.Suit._
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

class ScoreCardsRuleSpec extends AnyWordSpec with Matchers {

  "The ScoreCardsRule" should {

    import TestEvents._

    def checkCards(hands: Seq[(Seq[FaceSuit], Int)], cut: FaceSuit) = hands.foreach { hand =>
      val faceSuits = hand._1
      val expectedScore = hand._2
      assertScore(faceSuits, cut, expectedScore)
    }

    def assertScore(cards: Seq[FaceSuit], cut: FaceSuit, expectedScore: Int) = {
      val game = model.Game(randomId)
        .withDeck(deck)
        .withPlayer(player1Id)
        .withPlayer(player2Id)
        .withDealer(player2Id)
        .withHand(player1Id, cardsOf(cards))
        .withCut(cardOf(cut))
      scorePone(game) should be {
        if (expectedScore != 0) Seq(PegScore(player1Id, expectedScore))
        else Seq.empty
      }
    }

    "peg cards with fifteens" in {
      val cut = (Queen, Hearts)
      val cards = Seq(
        Seq((Five, Clubs), (Nine, Hearts), (Four, Diamonds), (Two, Spades)) -> 4,
        Seq((Ace, Clubs), (Four, Clubs), (Ten, Clubs), (King, Hearts)) -> 6
      )
      checkCards(cards, cut)
    }

    "pegs cards with pairs" in {
      val cut = (Ace, Hearts)
      val cards = Seq(
        Seq((Five, Clubs), (Five, Hearts), (Ace, Diamonds), (Two, Spades)) -> 4,
        Seq((Queen, Clubs), (Three, Clubs), (Three, Diamonds), (King, Hearts)) -> 2,
        Seq((Four, Hearts), (Four, Clubs), (Four, Diamonds), (Five, Hearts)) -> 6,
        Seq((Four, Hearts), (Four, Clubs), (Four, Diamonds), (Four, Spades)) -> 12
      )
      checkCards(cards, cut)
    }

    "pegs cards with runs" in {
      val cut = (Ace, Diamonds)
      val cards = Seq(
        Seq((Two, Clubs), (Jack, Hearts), (Queen, Diamonds), (King, Spades)) -> 3,
        Seq((Jack, Clubs), (Jack, Hearts), (Queen, Diamonds), (King, Spades)) -> 8,
        Seq((Ten, Clubs), (Jack, Hearts), (Queen, Diamonds), (King, Spades)) -> 4,
        Seq((Two, Hearts), (Two, Clubs), (Three, Diamonds), (Four, Spades)) -> 10,
        Seq((Two, Hearts), (Five, Clubs), (Three, Diamonds), (Four, Spades)) -> (5 + 2) // +2 for fifteen
      )
      checkCards(cards, cut)
    }

    "pegs cards with one for his heels" in {
      val cut = (Ace, Hearts)
      val cards = Seq(
        Seq((Two, Clubs), (Jack, Hearts), (Ten, Diamonds), (Six, Spades)) -> 1,
        Seq((Two, Clubs), (Jack, Diamonds), (Ten, Diamonds), (Six, Spades)) -> 0
      )
      checkCards(cards, cut)
    }

    "pegs cards with flush" in {
      val cut = (Ace, Diamonds)
      val cards = Seq(
        Seq((Two, Diamonds), (Seven, Diamonds), (Queen, Diamonds), (King, Diamonds)) -> 5,
        Seq((Two, Hearts), (Seven, Hearts), (Queen, Hearts), (King, Hearts)) -> 4,
        Seq((Two, Diamonds), (Seven, Diamonds), (Queen, Diamonds), (King, Clubs)) -> 0
      )
      checkCards(cards, cut)
    }
  }

}
