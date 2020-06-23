package com.nigeleke.cribbage

import java.util.UUID

import akka.actor.testkit.typed.scaladsl.{ActorTestKit, ScalaTestWithActorTestKit}
import akka.actor.typed.eventstream.EventStream.{Publish, Subscribe}
import com.nigeleke.cribbage.actors.Game.{DealerCutRevealed, DealerSelected, PlayerJoined, Event => GameEvent}
import com.nigeleke.cribbage.actors.rules.CutForDealRule
import com.nigeleke.cribbage.model.Game.{Id => GameId}
import com.nigeleke.cribbage.model.Player.{Id => PlayerId}
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpecLike

import scala.concurrent.duration._

class CutForDealRuleSpec extends ScalaTestWithActorTestKit with AnyWordSpecLike with Matchers {

  val eventsProbe = testKit.createTestProbe[GameEvent]()
  testKit.system.eventStream ! Subscribe(eventsProbe.ref)

  def joinPlayer(gameId: GameId) : PlayerId = {
    val playerId = UUID.randomUUID()
    val player1JoinedEvent = PlayerJoined(gameId, playerId)
    system.eventStream ! Publish(player1JoinedEvent)
    eventsProbe.expectMessage(player1JoinedEvent)
    playerId
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
        joinPlayer(gameId)
        eventsProbe.expectNoMessage()
      }
    }

    "select dealer" when {
      "second player joins game" in {
        val gameId = UUID.randomUUID()
        spawn(CutForDealRule(gameId))
        eventsProbe.expectNoMessage()

        val player1Id = joinPlayer(gameId)
        val player2Id = joinPlayer(gameId)

        def expectedCuts() = {
          val cut1 = eventsProbe.expectMessageType[DealerCutRevealed]
          val cut2 = eventsProbe.expectMessageType[DealerCutRevealed]
          (cut1, cut2)
        }

        def cardRanksTheSame(cuts: (DealerCutRevealed, DealerCutRevealed)) = {
          val cutRank1 = cuts._1.card.rank
          val cutRank2 = cuts._2.card.rank
          cutRank1 == cutRank2
        }

        val cuts = Iterator(expectedCuts()).dropWhile(cardRanksTheSame).next()

        def expectedDealer(cuts: (DealerCutRevealed, DealerCutRevealed)) = {
          val (player1, cutRank1) = (cuts._1.playerId, cuts._1.card.rank)
          val (player2, cutRank2) = (cuts._2.playerId, cuts._2.card.rank)
          if (cutRank1 < cutRank2) player1 else player2
        }

        val dealerSelected = eventsProbe.expectMessageType[DealerSelected]
        dealerSelected should be(DealerSelected(gameId, expectedDealer(cuts)))
      }
    }
  }

}
