package com.nigeleke.cribbage

import com.nigeleke.cribbage.actors.Game._
import com.nigeleke.cribbage.actors.rules.DealRule
import com.nigeleke.cribbage.model.Game
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

class DealRuleSpec extends AnyWordSpec with Matchers {

  "The DealRule" should {

    val game = Game(randomId)

    "deal" when {

      "a dealer has been selected" in {
        val (player1Id, player2Id) = (randomId, randomId)

        val gameUnderTest = game.withPlayer(player1Id).withPlayer(player2Id).withDealer(player1Id)
        DealRule.commands(Starting(gameUnderTest)) should be(Seq(DealHands))
      }

      "the scoring completes and the dealer is swapped" ignore { }

    }

    "not deal" when {

      "the deal has been made" in {
        val (player1Id, player2Id) = (randomId, randomId)

        val gameUnderTest = game
          .withPlayer(player1Id)
          .withPlayer(player2Id)
          .withDealer(player1Id)
          .withHand(player1Id, Seq.empty)
          .withHand(player2Id, Seq.empty)

        DealRule.commands(Starting(gameUnderTest)) should be(empty)
      }

    }

  }
}
