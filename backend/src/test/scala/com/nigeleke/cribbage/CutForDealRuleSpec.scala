package com.nigeleke.cribbage

import java.util.UUID

import akka.actor.testkit.typed.scaladsl.ScalaTestWithActorTestKit
import akka.actor.typed.eventstream.EventStream.{Publish, Subscribe}
import com.nigeleke.cribbage.actors.Game.{DealerCutRevealed, DealerSelected, PlayerJoined, Event => GameEvent}
import com.nigeleke.cribbage.actors.rules.CutForDealRule
import com.nigeleke.cribbage.model.Card
import com.nigeleke.cribbage.model.Game.{Id => GameId}
import com.nigeleke.cribbage.suit.{Face, Suit}
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpecLike

class CutForDealRuleSpec extends ScalaTestWithActorTestKit with AnyWordSpecLike with Matchers {

  val eventsProbe = testKit.createTestProbe[GameEvent]()
  testKit.system.eventStream ! Subscribe(eventsProbe.ref)

  private def addPlayer(gameId: GameId) = {
    val player1JoinedEvent = PlayerJoined(gameId, UUID.randomUUID())
    system.eventStream ! Publish(player1JoinedEvent)
    eventsProbe.expectMessage(player1JoinedEvent)
  }

  "The CutForDealRule" should {
    "do nothing" when {
      "created on behalf of a game" in {
        val gameId = UUID.randomUUID()
        spawn(CutForDealRule(gameId))
        eventsProbe.expectNoMessage()
      }

      "first player joins" in {
        val gameId = UUID.randomUUID()
        spawn(CutForDealRule(gameId))
        addPlayer(gameId)
        eventsProbe.expectNoMessage()
      }
    }

    "select dealer" when {
      "second player joins game" in {
        val gameId = UUID.randomUUID()
        spawn(CutForDealRule(gameId))
        eventsProbe.expectNoMessage()

        addPlayer(gameId)
        addPlayer(gameId)

        def cardRanksTheSame(cuts: (DealerCutRevealed, DealerCutRevealed)) = {
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
          eventsProbe.expectMessageType[DealerCutRevealed],
          eventsProbe.expectMessageType[DealerCutRevealed])

        Iterator
          .continually(expectReveals)
          .dropWhile(cardRanksTheSame)
          .take(1)
          .foreach { reveals =>
            val dealerSelected = eventsProbe.expectMessageType[DealerSelected]
            dealerSelected should be(DealerSelected(gameId, expectedDealer(reveals)))
          }
      }
    }
  }

}
