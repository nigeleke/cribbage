package com.nigeleke.cribbage

import com.nigeleke.cribbage.actors.Game._
import com.nigeleke.cribbage.actors.rules.Rules._
import com.nigeleke.cribbage.model.{Deck, Game}
import com.nigeleke.cribbage.model.Player.{Id => PlayerId}
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

class CutAtStartOfPlayRuleSpec extends AnyWordSpec with Matchers {

  "The CutAtStartOfPlayRule" should {

    val game = Game(randomId)
    val deck = Deck()

    def gameWithHandsDealt(player1Id: PlayerId, player2Id: PlayerId) = {
      game
        .withDeck(deck)
        .withPlayer(player1Id)
        .withPlayer(player2Id)
        .withDealer(player1Id)
        .withHand(player1Id,deck.take(6))
        .withHand(player2Id,deck.drop(6).take(6))
    }

    "issue the CutAtStartOfPlay command" when {

      "players have discarded all cards to the crib" in {
        val (player1Id, player2Id) = (randomId, randomId)

        val gameUnderTest = gameWithHandsDealt(player1Id, player2Id)
          .withCribDiscard(player1Id, deck.take(2))
          .withCribDiscard(player2Id, deck.drop(6).take(2))

        cutAtStartOfPlay(gameUnderTest) should be(Seq(CutAtStartOfPlay))
      }

    }

    "not issue the CutAtStartOfPlay command" when {

      "player discards still required" in {
        val (player1Id, player2Id) = (randomId, randomId)

        val gameUnderTest = gameWithHandsDealt(player1Id, player2Id)

        cutAtStartOfPlay(gameUnderTest) should be(empty)
      }

    }

  }
}
