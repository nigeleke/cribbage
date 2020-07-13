package com.nigeleke.cribbage

import com.nigeleke.cribbage.actors.Game.{Discarding, PegScore}
import com.nigeleke.cribbage.actors.rules.ScoreCutAtStartOfPlayRule
import com.nigeleke.cribbage.model.{Card, Game}
import com.nigeleke.cribbage.suit.{Face, Suit}
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

class ScoreCutAtStartOfPlayRuleSpec extends AnyWordSpec with Matchers {

  val (player1Id, player2Id) = (randomId, randomId)

  val game = Game(randomId)
    .withPlayer(player1Id)
    .withPlayer(player2Id)
    .withDealer(player1Id)

  "The ScoreCutAtStartOfPlayRule" should {

    "score for dealer two for his heels if Jack is cut" in {
      val gameUnderTest = game.withCut(Card(randomId, Face.Jack, Suit.Hearts))
      ScoreCutAtStartOfPlayRule.commands(Discarding(gameUnderTest)) should be(Seq(PegScore(player1Id, 2)))
    }

    "no score dealer if Jack is not cut" in {
      val gameUnderTest = game.withCut(Card(randomId, Face.Queen, Suit.Hearts))
      ScoreCutAtStartOfPlayRule.commands(Discarding(gameUnderTest)) should be(empty)
    }

  }
}
