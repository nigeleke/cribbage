package com.nigeleke.cribbage

import com.nigeleke.cribbage.model.Game
import com.nigeleke.cribbage.model.Face._
import com.nigeleke.cribbage.model.Suit._
import com.nigeleke.cribbage.TestModel._
import com.nigeleke.cribbage.entity.GameEntity._
import com.nigeleke.cribbage.entity.handlers.CommandHandler
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

class EndPlayRuleSpec extends AnyWordSpec with Matchers {

  "The EndPlayRule" should {

    val attributes = Game()
      .withPlayer(player1Id)
      .withPlayer(player2Id)
      .withDealer(player1Id)
      .withDeal(
        Map(
          (player1Id, cardsOf(Seq((King, Hearts), (King, Clubs), (King, Diamonds), (King, Spades)))),
          (player2Id, cardsOf(Seq((Jack, Hearts), (Jack, Clubs), (Jack, Diamonds), (Jack, Spades))))),
        deck)

    "not declare the Play finished" when {
      "either Player has Cards available for the CurrentPlay" in {
        val attributesUnderTest = attributes
          .withLay(player2Id, cardOf(Jack, Hearts))
          .withLay(player1Id, cardOf(King, Hearts))
        CommandHandler.endPlay(attributesUnderTest) should be(Seq.empty)
      }
    }

    "declare the Play finished" when {
      "both Players have no Cards available for the CurrentPlay" in {
        val attributesUnderTest = attributes
          .withLay(player2Id, cardOf(Jack, Hearts))
          .withLay(player1Id, cardOf(King, Hearts))
          .withLay(player2Id, cardOf(Jack, Clubs))
          .withPass(player1Id)
          .withPass(player2Id)
        CommandHandler.endPlay(attributesUnderTest) should contain theSameElementsInOrderAs (
          Seq(PointsScored(player2Id, 1), PlayCompleted))
      }
    }

    "declare all Plays finished" when {
      "both Player have no Cards available" in {
        val attributesUnderTest = attributes
          .withLay(player2Id, cardOf(Jack, Hearts))
          .withLay(player1Id, cardOf(King, Hearts))
          .withLay(player2Id, cardOf(Jack, Clubs))
          .withPass(player1Id)
          .withPass(player2Id)
          .withNextPlay()
          .withLay(player1Id, cardOf(King, Clubs))
          .withLay(player2Id, cardOf(Jack, Diamonds))
          .withLay(player1Id, cardOf(King, Diamonds))
          .withPass(player2Id)
          .withPass(player1Id)
          .withNextPlay()
          .withLay(player2Id, cardOf(Jack, Spades))
          .withLay(player1Id, cardOf(King, Spades))
        CommandHandler.endPlay(attributesUnderTest) should contain theSameElementsInOrderAs (
          Seq(PointsScored(player1Id, 1), PlayCompleted))
      }
    }

  }

}
