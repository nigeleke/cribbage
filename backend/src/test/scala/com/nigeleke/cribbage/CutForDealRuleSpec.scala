package com.nigeleke.cribbage

import java.util.UUID

import akka.actor.testkit.typed.scaladsl.{ScalaTestWithActorTestKit, TestProbe}
import akka.actor.typed.ActorRef
import com.nigeleke.cribbage.actors.Game.{DealerCutRevealed, DealerSelected, PlayerJoined}
import com.nigeleke.cribbage.actors.rules.CutForDealRule
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpecLike

class CutForDealRuleSpec extends ScalaTestWithActorTestKit with AnyWordSpecLike with Matchers {

  "The CutForDealRule" should {
    "do nothing" when {

      "created on behalf of a game" in withRuleAndProbe { (_, probe) =>
        probe.expectNoMessage()
      }

      "first player joins" in withRuleAndProbe { (rule, probe) =>
        rule ! PlayerJoined(UUID.randomUUID())
        probe.expectNoMessage()
      }
    }

    "select dealer" when {
      "second player joins game" in withRuleAndProbe { (rule, probe) =>
        def sameCardRanks(cuts: (DealerCutRevealed, DealerCutRevealed)) = {
          val cutRank1 = cuts._1.card.rank
          val cutRank2 = cuts._2.card.rank
          cutRank1 == cutRank2
        }

        def expectedDealer(cuts: (DealerCutRevealed, DealerCutRevealed)) = {
          val (player1, cutRank1) = (cuts._1.playerId, cuts._1.card.rank)
          val (player2, cutRank2) = (cuts._2.playerId, cuts._2.card.rank)
          if (cutRank1 < cutRank2) player1 else player2
        }

        def expectReveals() = (
          probe.expectMessageType[DealerCutRevealed],
          probe.expectMessageType[DealerCutRevealed])

        rule ! PlayerJoined(UUID.randomUUID())
        rule ! PlayerJoined(UUID.randomUUID())

        Iterator
          .continually(expectReveals)
          .dropWhile(sameCardRanks)
          .take(1)
          .foreach { reveals =>
            val dealerSelected = probe.expectMessageType[DealerSelected]
            dealerSelected should be(DealerSelected(expectedDealer(reveals)))
          }
      }
    }
  }

  type Rule = ActorRef[CutForDealRule.Command]
  type Probe = TestProbe[CutForDealRule.Event]

  private def withRuleAndProbe(f: (Rule, Probe) => Unit) = {
    val probe = createTestProbe[CutForDealRule.Event]()
    val rule = spawn(CutForDealRule(probe.ref))
    f(rule, probe)
  }

}
