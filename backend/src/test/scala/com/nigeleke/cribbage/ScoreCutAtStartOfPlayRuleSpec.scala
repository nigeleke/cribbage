package com.nigeleke.cribbage

import com.nigeleke.cribbage.actors.Game.PegScore
import com.nigeleke.cribbage.actors.rules.Rules._
import com.nigeleke.cribbage.model._
import com.nigeleke.cribbage.suit._
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
      val deck = Deck()
      val jack = deck.find(_.face == Face.Jack).head
      val gameUnderTest = game.withDeck(deck).withCut(jack)
      scoreCutAtStartOfPlay(gameUnderTest) should be(Seq(PegScore(player1Id, 2)))
    }

    "no score dealer if Jack is not cut" in {
      val deck = Deck()
      val notJack = deck.find(_.face == Face.Queen).head
      val gameUnderTest = game.withDeck(deck).withCut(notJack)
      scoreCutAtStartOfPlay(gameUnderTest) should be(empty)
    }

  }
}
