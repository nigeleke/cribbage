package com.nigeleke.cribbage

import com.nigeleke.cribbage.actors.Game._
import com.nigeleke.cribbage.actors.rules.MakeCutRule
import com.nigeleke.cribbage.model.{Deck, Game}
import com.nigeleke.cribbage.model.Player.{Id => PlayerId}
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

class MakeCutRuleSpec extends AnyWordSpec with Matchers {

  "The MakeCutRule" should {

    val game = Game(randomId)
    val cardIds = Deck().map(_.id)

    def gameWithHandsDealt(player1Id: PlayerId, player2Id: PlayerId) = {
      game.withPlayer(player1Id)
        .withPlayer(player2Id)
        .withDealer(player1Id)
        .withHand(player1Id, cardIds.take(6))
        .withHand(player2Id, cardIds.drop(6).take(6))
    }

    "make a cut" when {

      "players have discarded all cards to the crib" in {
        val (player1Id, player2Id) = (randomId, randomId)

        val gameUnderTest = gameWithHandsDealt(player1Id, player2Id)
          .withCribDiscard(player1Id, cardIds.take(2))
          .withCribDiscard(player2Id, cardIds.drop(6).take(2))

        MakeCutRule.commands(Discarding(gameUnderTest)) should be(Seq(MakeCut))
      }

    }

    "not make a cut" when {

      "player discards still required" in {
        val (player1Id, player2Id) = (randomId, randomId)

        val gameUnderTest = gameWithHandsDealt(player1Id, player2Id)

        MakeCutRule.commands(Discarding(gameUnderTest)) should be(empty)
      }

    }

  }
}
