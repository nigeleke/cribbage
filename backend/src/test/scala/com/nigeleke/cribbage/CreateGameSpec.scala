package com.nigeleke.cribbage

import akka.actor.testkit.typed.scaladsl.{ LogCapturing, ScalaTestWithActorTestKit }
import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit
import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit.SerializationSettings
import com.nigeleke.cribbage.entity.GameEntity
import com.nigeleke.cribbage.entity.GameEntity._
import com.nigeleke.cribbage.TestModel._
import org.scalatest.BeforeAndAfterEach
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpecLike

class CreateGameSpec
  extends ScalaTestWithActorTestKit(EventSourcedBehaviorTestKit.config)
  with AnyWordSpecLike
  with BeforeAndAfterEach
  with LogCapturing
  with Matchers {

  implicit val log = system.log

  private val eventSourcedTestKit =
    EventSourcedBehaviorTestKit[Command, Event, State](
      system,
      GameEntity(randomId),
      SerializationSettings.disabled)

  override protected def beforeEach(): Unit = {
    super.beforeEach()
    eventSourcedTestKit.clear()
  }

  "A GameEntity" should {

    "be creatable" in {
      val probe = createTestProbe[Reply]()
      val createGameCommand = CreateGame(probe.ref)
      val result = eventSourcedTestKit.runCommand(createGameCommand)
      probe.expectMessage(Accepted(createGameCommand))
      result.command should be(createGameCommand)
      result.event should be(a[GameCreated])
      result.state should be(a[Starting])
    }

    "not be able to be created more than once" in {
      val probe = createTestProbe[Reply]()
      val createGameCommand = CreateGame(probe.ref)
      val results = Seq(createGameCommand, createGameCommand).map(eventSourcedTestKit.runCommand)

      probe.expectMessage(Accepted(createGameCommand))
      probe.expectMessageType[Rejected]

      results.flatMap(_.events).filter(_.isInstanceOf[GameCreated]).size should be(1)
      results.last.state should be(a[Starting])
    }

  }

}