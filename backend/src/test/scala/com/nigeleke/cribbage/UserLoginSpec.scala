package com.nigeleke.cribbage

import java.util.UUID

import akka.actor.testkit.typed.scaladsl.ScalaTestWithActorTestKit
import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit
import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit.SerializationSettings
import akka.persistence.typed.PersistenceId
import com.nigeleke.cribbage.entity.UserEntity
import com.nigeleke.cribbage.entity.UserEntity._
import org.scalatest.BeforeAndAfterEach
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpecLike

class UserLoginSpec
  extends ScalaTestWithActorTestKit(EventSourcedBehaviorTestKit.config)
  with AnyWordSpecLike
  with BeforeAndAfterEach
  with Matchers {

  implicit val log = system.log

  private val eventSourcedTestKit =
    EventSourcedBehaviorTestKit[Command, Event, State](
      system,
      UserEntity("test-user", PersistenceId.ofUniqueId(UUID.randomUUID().toString)),
      SerializationSettings.disabled)

  override protected def beforeEach(): Unit = {
    super.beforeEach()
    eventSourcedTestKit.clear()
  }

  "A UserEntity" should {

    "be able to 'login'" in {
      val loginCommand = Login("test-name", _)
      val result = eventSourcedTestKit.runCommand(loginCommand)
      result.reply.isSuccess should be(true)
      result.event should be(a[LoggedIn])
    }

  }

}
