package com.nigeleke.cribbage

import java.util.UUID

import akka.actor.testkit.typed.scaladsl.{ScalaTestWithActorTestKit, TestProbe}
import akka.actor.typed.ActorRef
import com.nigeleke.cribbage.actors.Game._
import com.nigeleke.cribbage.actors.rules.DealerRule
import com.nigeleke.cribbage.model.Deck
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpecLike

class DealerRuleSpec extends ScalaTestWithActorTestKit with AnyWordSpecLike with Matchers {

  "The DealerRule" should {

    "deal" when {

      "A dealer has been selected" in withRuleAndProbe { (rule, probe) =>
        val (player1Id, player2Id) = (UUID.randomUUID(), UUID.randomUUID())
        rule ! PlayerJoined(player1Id)
        rule ! PlayerJoined(player2Id)
        rule ! DealerSelected(player1Id)
        val hand1 = probe.expectMessageType[HandDealt]
        val hand2 = probe.expectMessageType[HandDealt]
        hand1.playerId should not be(hand2.playerId)
        hand1.hand.size should be(6)
        hand2.hand.size should be(6)

        val remaining = Deck.shuffled().map(_.id).toSet -- hand1.hand.toSet -- hand2.hand.toSet
        remaining.size should be(40)
      }

      "The scoring completes and the dealer is swapped" ignore { }

    }

  }

  type Rule = ActorRef[DealerRule.Command]
  type Probe = TestProbe[DealerRule.Event]

  private def withRuleAndProbe(f: (Rule, Probe) => Unit) = {
    val probe = createTestProbe[DealerRule.Event]()
    val rule = spawn(DealerRule(probe.ref))
    f(rule, probe)
  }

}
