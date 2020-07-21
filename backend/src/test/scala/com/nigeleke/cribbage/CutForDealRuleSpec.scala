package com.nigeleke.cribbage

import com.nigeleke.cribbage.actors.Game.CutForDeal
import com.nigeleke.cribbage.actors.rules.Rules._
import com.nigeleke.cribbage.model.Game
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

class CutForDealRuleSpec extends AnyWordSpec with Matchers {

  "The CutForDealRule" should {

    val game = Game(randomId)

    "do nothing" when {
      "created on behalf of a game" in {
        cutForDeal(game) should be(empty)
      }

      "first player joins" in {
        val gameUnderTest = game.withPlayer(randomId)
        cutForDeal(gameUnderTest) should be(empty)
      }
    }

    "select dealer" when {
      "second player joins game and no dealer" in {
        val gameUnderTest = game.withPlayer(randomId).withPlayer(randomId)
        cutForDeal(gameUnderTest) should contain inOrderElementsOf(Seq(CutForDeal))
      }
    }

    "not select dealer" when {
      "second player joins game and dealer already present" in {
        val (player1Id, player2Id) = (randomId, randomId)
        val gameUnderTest = game.withPlayer(player1Id).withPlayer(player2Id).withDealer(player1Id)
        cutForDeal(gameUnderTest) should be(empty)
      }
    }

  }
}
