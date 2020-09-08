package com.nigeleke.cribbage

import java.util.UUID

import akka.actor.testkit.typed.scaladsl.{ LogCapturing, ScalaTestWithActorTestKit }
import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit
import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit.SerializationSettings
import com.nigeleke.cribbage.actors.Game
import com.nigeleke.cribbage.actors.Game._
import org.scalatest.BeforeAndAfterEach
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpecLike

class CreateGameSpec
  extends ScalaTestWithActorTestKit(EventSourcedBehaviorTestKit.config)
  with AnyWordSpecLike
  with BeforeAndAfterEach
  with LogCapturing
  with Matchers {

  private val eventSourcedTestKit =
    EventSourcedBehaviorTestKit[Command, Event, State](
      system,
      Game(),
      SerializationSettings.disabled)

  override protected def beforeEach(): Unit = {
    super.beforeEach()
    eventSourcedTestKit.clear()
  }

  "A Game" should {

    "be creatable" in {
      val gameId = UUID.randomUUID()
      val createGameCommand = CreateGame(gameId)
      val result = eventSourcedTestKit.runCommand(createGameCommand)
      result.command should be(createGameCommand)
      result.event should be(GameCreated(gameId))
      result.state should be(a[Starting])
    }

    "not be able to be created more than once" in {
      val gameId = UUID.randomUUID()
      val createGameCommand = CreateGame(gameId)
      val results = Seq(createGameCommand, createGameCommand).map(eventSourcedTestKit.runCommand)
      results.flatMap(_.events) should contain theSameElementsInOrderAs (Seq(GameCreated(gameId)))
      results.last.state should be(a[Starting])
    }

  }

}