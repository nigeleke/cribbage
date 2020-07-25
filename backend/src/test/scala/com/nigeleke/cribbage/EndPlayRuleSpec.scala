package com.nigeleke.cribbage

import com.nigeleke.cribbage.actors.Game.{CompletePlay, PegScore}
import com.nigeleke.cribbage.actors.rules.Rules
import com.nigeleke.cribbage.model.Game
import com.nigeleke.cribbage.suit.Face._
import com.nigeleke.cribbage.suit.Suit._
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

class EndPlayRuleSpec extends AnyWordSpec with Matchers {

  "The EndPlayRule" should {

    import TestEvents._

    val (player1Id, player2Id) = (randomId, randomId)
    val game = Game(randomId)
      .withPlayer(player1Id)
      .withPlayer(player2Id)
      .withDealer(player1Id)
      .withDeal(Map(
        (player1Id, cardsOf(Seq((King,Hearts), (King,Clubs), (King,Diamonds), (King,Spades)))),
        (player2Id, cardsOf(Seq((Jack,Hearts), (Jack,Clubs), (Jack,Diamonds), (Jack,Spades))))),
        deck.reverse.take(48)) // Swizzle the crib discards; we don't care for this test

    "not declare the Play finished" when {
      "either Player has Cards available for the CurrentPlay" in {
        val gameUnderTest = game
          .withLay(player2Id, cardOf(Jack,Hearts))
          .withLay(player1Id, cardOf(King,Hearts))
        Rules.endPlay(gameUnderTest) should be(empty)
      }
    }

    "declare the Play finished" when {
      "both Players have no Cards available for the CurrentPlay" in {
        val gameUnderTest = game
          .withLay(player2Id, cardOf(Jack,Hearts))
          .withLay(player1Id, cardOf(King,Hearts))
          .withLay(player2Id, cardOf(Jack,Clubs))
          .withPass(player1Id)
          .withPass(player2Id)
        Rules.endPlay(gameUnderTest) should contain theSameElementsInOrderAs(
          Seq(PegScore(player2Id, 1), CompletePlay))
      }
    }

    "declare all Plays finished" when {
      "both Player have no Cards available" in {
        val gameUnderTest = game
          .withLay(player2Id, cardOf(Jack,Hearts))
          .withLay(player1Id, cardOf(King,Hearts))
          .withLay(player2Id, cardOf(Jack,Clubs))
          .withPass(player1Id)
          .withPass(player2Id)
          .withNextPlay()
          .withLay(player1Id, cardOf(King,Clubs))
          .withLay(player2Id, cardOf(Jack,Diamonds))
          .withLay(player1Id, cardOf(King,Diamonds))
          .withPass(player2Id)
          .withPass(player1Id)
          .withNextPlay()
          .withLay(player2Id, cardOf(Jack,Spades))
          .withLay(player1Id, cardOf(King,Spades))
        Rules.endPlay(gameUnderTest) should contain theSameElementsInOrderAs(
          Seq(PegScore(player1Id, 1), CompletePlay))
      }
    }


  }

}
