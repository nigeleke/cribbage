package com.nigeleke.cribbage

import java.util.UUID

import akka.actor.testkit.typed.scaladsl.{ScalaTestWithActorTestKit, TestProbe}
import akka.actor.typed.ActorRef
import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit
import com.nigeleke.cribbage.actors.Game
import com.nigeleke.cribbage.actors.Game._
import com.nigeleke.cribbage.actors.handlers.StartingGame
import com.nigeleke.cribbage.model.Player.{Id => PlayerId}
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpecLike

class StartingGameSpec
  extends ScalaTestWithActorTestKit(EventSourcedBehaviorTestKit.config)
    with AnyWordSpecLike
    with Matchers {

  "A StartingGame" should {

    "be initialised with no players" in withStartingGameAndProbes { (game, eventProbe, responseProbe) =>
      game ! Players(responseProbe.ref)
      responseProbe.expectMessage(Set.empty[PlayerId])
      eventProbe.expectNoMessage
    }

    "allow a new player to join" in withStartingGameAndProbes { (game, eventProbe, responseProbe) =>
      val playerId = UUID.randomUUID

      game ! Join(playerId)
      eventProbe.expectMessage(PlayerJoined(playerId))

      game ! Players(responseProbe.ref)
      responseProbe.expectMessage(Set(playerId))
    }

    "allow two new players to join" in withStartingGameAndProbes { (game, eventProbe, responseProbe) =>
      val (player1Id, player2Id) = (UUID.randomUUID, UUID.randomUUID)

      game ! Join(player1Id)
      eventProbe.expectMessage(PlayerJoined(player1Id))

      game ! Join(player2Id)
      eventProbe.expectMessage(PlayerJoined(player2Id))

      game ! Players(responseProbe.ref)
      responseProbe.expectMessage(Set(player1Id, player2Id))
    }

    "not allow three players to join" in withStartingGameAndProbes { (game, eventProbe, responseProbe) =>
      val (player1Id, player2Id, player3Id) = (UUID.randomUUID, UUID.randomUUID, UUID.randomUUID)

      game ! Join(player1Id)
      eventProbe.expectMessage(PlayerJoined(player1Id))

      game ! Join(player2Id)
      eventProbe.expectMessage(PlayerJoined(player2Id))

      game ! Join(player3Id)
      eventProbe.expectNoMessage()

      game ! Players(responseProbe.ref)
      responseProbe.expectMessage(Set(player1Id, player2Id))
    }

    "not allow the same player to join twice" in withStartingGameAndProbes { (game, eventProbe, responseProbe) =>
      val playerId = UUID.randomUUID

      game ! Join(playerId)
      eventProbe.expectMessage(PlayerJoined(playerId))

      game ! Join(playerId)
      eventProbe.expectNoMessage()

      game ! Players(responseProbe.ref)
      responseProbe.expectMessage(Set(playerId))
    }
  }

  def withStartingGameAndProbes(f: (ActorRef[Game.Command], TestProbe[Game.Event], TestProbe[Set[PlayerId]]) => Unit) = {
    val responseProbe = testKit.createTestProbe[Set[PlayerId]]()
    val eventProbe = testKit.createTestProbe[Game.Event]()
    val game = testKit.spawn(StartingGame(eventProbe.ref))
    f(game, eventProbe, responseProbe)
  }

}
