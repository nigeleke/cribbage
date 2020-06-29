package com.nigeleke.cribbage

import java.util.UUID

import akka.actor.testkit.typed.scaladsl.ScalaTestWithActorTestKit
import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit
import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit.SerializationSettings
import com.nigeleke.cribbage.actors.Game
import com.nigeleke.cribbage.actors.Game._
import org.scalatest.BeforeAndAfterEach
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpecLike

class GameSpec
  extends ScalaTestWithActorTestKit(EventSourcedBehaviorTestKit.config)
    with AnyWordSpecLike
    with BeforeAndAfterEach
    with Matchers {

  private val eventSourcedTestKit =
    EventSourcedBehaviorTestKit[Command, Event, State](
      system,
      Game(UUID.randomUUID()),
      SerializationSettings.disabled)

  override protected def beforeEach(): Unit = {
    super.beforeEach()
    eventSourcedTestKit.clear()
  }


  "A Game" should {

    "invoke the CutForDealRule" when {

      "two players have joined" in {
        val probe = testKit.createTestProbe()

        val (player1Id, player2Id) = (UUID.randomUUID(), UUID.randomUUID())

        val result = Seq(Join(player1Id), Join(player2Id)).map { command =>
          val r = eventSourcedTestKit.runCommand(command)
          probe.expectNoMessage()
          r
        }.head

        println(s"result: $result")
        result.state.players.size should be(2)
        result.state.optDealer should not be(empty)
        result.state.hands.size should be(2)

        fail()
      }

    }
  }

}
