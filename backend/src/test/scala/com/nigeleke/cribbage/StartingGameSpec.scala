package com.nigeleke.cribbage

import java.util.UUID

import akka.actor.testkit.typed.scaladsl.ScalaTestWithActorTestKit
import akka.actor.typed.eventstream.EventStream.Subscribe
import com.nigeleke.cribbage.actors.Game._
import com.nigeleke.cribbage.actors.states.game.StartingGame
import com.nigeleke.cribbage.model.Player.{ Id => PlayerId }
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpecLike

class StartingGameSpec extends ScalaTestWithActorTestKit with AnyWordSpecLike with Matchers {

  val gameId = UUID.randomUUID()

  val eventsProbe = testKit.createTestProbe[Event]()
  testKit.system.eventStream ! Subscribe(eventsProbe.ref)

  "A StartingGame" should {
    "be initialised with no players" in {
      val probe = testKit.createTestProbe[Set[PlayerId]]()
      val game = testKit.spawn(StartingGame(gameId))
      game ! Players(probe.ref)
      probe.expectMessage(Set.empty[PlayerId])
    }

    "allow a new player to join" in {
      val game = testKit.spawn(StartingGame(gameId))
      val playerId = UUID.randomUUID
      game ! Join(playerId)

      eventsProbe.expectMessage(PlayerJoined(gameId, playerId))

      val probe = testKit.createTestProbe[Set[PlayerId]]()
      game ! Players(probe.ref)
      probe.expectMessage(Set(playerId))
    }

    "allow two new players to join" in {
      val game = testKit.spawn(StartingGame(gameId))
      val player1Id = UUID.randomUUID
      val player2Id = UUID.randomUUID
      game ! Join(player1Id)
      game ! Join(player2Id)

      eventsProbe.expectMessage(PlayerJoined(gameId, player1Id))
      eventsProbe.expectMessage(PlayerJoined(gameId, player2Id))

      val probe = testKit.createTestProbe[Set[PlayerId]]()
      game ! Players(probe.ref)
      probe.expectMessage(Set(player1Id, player2Id))
    }

    "not allow three players to join" in {
      val game = testKit.spawn(StartingGame(gameId))
      val player1Id = UUID.randomUUID
      val player2Id = UUID.randomUUID
      val player3Id = UUID.randomUUID
      game ! Join(player1Id)
      game ! Join(player2Id)

      eventsProbe.receiveMessages(2)

      game ! Join(player3Id)
      eventsProbe.expectNoMessage()

      val probe = testKit.createTestProbe[Set[PlayerId]]()
      game ! Players(probe.ref)
      probe.expectMessage(Set(player1Id, player2Id))
    }

    "not allow the same player to join twice" in {
      val game = testKit.spawn(StartingGame(gameId))
      val playerId = UUID.randomUUID
      game ! Join(playerId)
      eventsProbe.receiveMessages(1)

      game ! Join(playerId)
      eventsProbe.expectNoMessage()

      val probe = testKit.createTestProbe[Set[PlayerId]]()
      game ! Players(probe.ref)
      probe.expectMessage(Set(playerId))
    }
  }

}
