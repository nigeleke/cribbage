package com.nigeleke.cribbage

import com.nigeleke.cribbage.actors.Game._
import com.nigeleke.cribbage.actors.rules.Rules._
import com.nigeleke.cribbage.model.Game
import com.nigeleke.cribbage.suit.Face._
import com.nigeleke.cribbage.suit.Suit._
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

class DealRuleSpec extends AnyWordSpec with Matchers {

  "The DealRule" should {

    import TestEvents._

    val game = Game(randomId)

    "deal" when {

      "a dealer has been selected" in {
        val gameUnderTest = game.withPlayer(player1Id).withPlayer(player2Id).withDealer(player1Id)
        deal(gameUnderTest) should be(Seq(DealHands))
      }

      "the scoring completes and the dealer is swapped" in {
        val gameUnderTest = game
          .withDeck(deck)
          .withPlayer(player1Id)
          .withPlayer(player2Id)
          .withDealer(player1Id)
          .withHand(player1Id, cardsOf(Seq((Ace,Hearts), (Ace,Clubs), (Six,Hearts), (Seven,Hearts), (Seven,Clubs), (Eight,Clubs))))
          .withHand(player2Id, cardsOf(Seq((Ace,Diamonds), (Ace,Spades), (Five,Hearts),(Five,Clubs),(Five,Diamonds),(Jack,Spades))))
          .withCribDiscard(player1Id, cardsOf(Seq((Ace,Hearts), (Ace,Clubs))))
          .withCribDiscard(player2Id, cardsOf(Seq((Ace,Diamonds), (Ace,Spades))))
          .withCut(cardOf(Five,Spades))

        deal(gameUnderTest.withSwappedDealer()) should be(Seq(DealHands))
      }
    }

    "not deal" when {

      "the deal has been made" in {
        val gameUnderTest = game
          .withPlayer(player1Id)
          .withPlayer(player2Id)
          .withDealer(player1Id)
          .withHand(player1Id, Seq.empty)
          .withHand(player2Id, Seq.empty)

        deal(gameUnderTest) should be(empty)
      }

    }

  }
}
