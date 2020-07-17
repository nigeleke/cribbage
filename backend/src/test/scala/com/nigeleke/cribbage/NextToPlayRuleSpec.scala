package com.nigeleke.cribbage

import com.nigeleke.cribbage.model.Game
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

class NextToPlayRuleSpec extends AnyWordSpec with Matchers {

  val (player1Id, player2Id) = (randomId, randomId)

  val game = Game(randomId)
    .withPlayer(player1Id)
    .withPlayer(player2Id)
    .withDealer(player1Id)

  "The NextToPlayRule" should {

    "score final play if neither player can go" when {
      "running total is 31" ignore {}
      "running total is less than 31" ignore {}
    }

    "set up for next play" when {
      "neither player can go" ignore {}
    }

    "set up for scoring the hands" when {
      "neither player can go" ignore {}
    }

  }

}
