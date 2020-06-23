package com.nigeleke.cribbage

import java.util.UUID

import akka.actor.testkit.typed.scaladsl.ScalaTestWithActorTestKit
import akka.actor.typed.eventstream.EventStream.Subscribe
import com.nigeleke.cribbage.actors.Game._
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
        val supervisor = testKit.spawn(GameSupervisor())

        val probe = testKit.createTestProbe[Response]()
        supervisor ! GetGames(probe.ref)
        probe.expectMessage(Games(Seq.empty))
      }

      "a new Game is created" in {
        val supervisor = testKit.spawn(GameSupervisor())

        val gameId = UUID.randomUUID()
        supervisor ! CreateGame(gameId)
        eventsProbe.expectMessage(GameCreated(gameId))

        val probe = testKit.createTestProbe[Response]()
        supervisor ! GetGames(probe.ref)
        probe.expectMessage(Games(Seq(gameId)))
      }

      "a Game is added more than once" in {
        val supervisor = testKit.spawn(GameSupervisor())

        val gameId = UUID.randomUUID()
        supervisor ! CreateGame(gameId)
        eventsProbe.expectMessage(GameCreated(gameId))

        supervisor ! CreateGame(gameId)
        eventsProbe.expectNoMessage()

        val probe = testKit.createTestProbe[Response]()
        supervisor ! GetGames(probe.ref)
        probe.expectMessage(Games(Seq(gameId)))

      }

    }

  }

}