package com.nigeleke.cribbage

import java.util.UUID

import akka.actor.testkit.typed.scaladsl.ScalaTestWithActorTestKit
import akka.actor.typed.eventstream.EventStream.Subscribe
import com.nigeleke.cribbage.actors.GameSupervisor
import com.nigeleke.cribbage.actors.GameSupervisor._
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpecLike

class GameSupervisorSpec extends ScalaTestWithActorTestKit with AnyWordSpecLike with Matchers {

  val eventsProbe = testKit.createTestProbe[Event]()
  testKit.system.eventStream ! Subscribe(eventsProbe.ref)

  "A GameSupervisor" should {

    "maintain a list of Games" when {
      "no Games have been created" in {
        val probe = testKit.createTestProbe[Response]()
        val supervisor = testKit.spawn(GameSupervisor())

        supervisor ! GetGames(probe.ref)
        probe.expectMessage(Games(Seq.empty))
      }

      "a new GameFacade is created" in {
        val probe = testKit.createTestProbe[Response]()
        val supervisor = testKit.spawn(GameSupervisor())

        val gameId = UUID.randomUUID()
        supervisor ! CreateGame(gameId, probe.ref)
        probe.expectMessage(GameCreated(gameId))
        eventsProbe.expectMessage(GameCreated(gameId))

        supervisor ! GetGames(probe.ref)
        probe.expectMessage(Games(Seq(gameId)))
      }

      "a GameFacade is added more than once" in {
        val probe = testKit.createTestProbe[Response]()
        val supervisor = testKit.spawn(GameSupervisor())

        val gameId = UUID.randomUUID()
        supervisor ! CreateGame(gameId, probe.ref)
        probe.expectMessage(GameCreated(gameId))
        eventsProbe.expectMessage(GameCreated(gameId))

        supervisor ! CreateGame(gameId, probe.ref)
        probe.expectNoMessage()
        eventsProbe.expectNoMessage()

        supervisor ! GetGames(probe.ref)
        probe.expectMessage(Games(Seq(gameId)))

      }

    }

  }

}