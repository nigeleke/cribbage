package com.nigeleke.cribbage

import java.util.UUID

import akka.Done
import akka.actor.testkit.typed.scaladsl.ScalaTestWithActorTestKit
import akka.stream.scaladsl.Sink
import com.nigeleke.cribbage.entity.GameEntity.GameCreated
import com.nigeleke.cribbage.entity.UserEntity.LoggedIn
import com.nigeleke.cribbage.services.{ GameJournal, GameService, UserService }
import com.typesafe.config.ConfigFactory
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AsyncWordSpecLike

class UserServiceSpec
  extends ScalaTestWithActorTestKit(ConfigFactory.parseString(
    """akka {
      |  cluster {
      |    seed-nodes = ["akka://UserServiceSpec@127.0.0.1:2551"]
      |  }
      |}
      |""".stripMargin).withFallback(ConfigFactory.load()))
  with AsyncWordSpecLike
  with Matchers {

  implicit val config = testKit.config

  "A UserService" should {

    "allow a user to login" in {
      val service = new UserService(system)
      for {
        loggedIn <- service.login("name")
      } yield (loggedIn should be(LoggedIn("name", loggedIn.userId)))
    }
  }
}
