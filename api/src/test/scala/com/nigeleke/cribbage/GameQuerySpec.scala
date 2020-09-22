package com.nigeleke.cribbage

import akka.actor.testkit.typed.scaladsl.ScalaTestWithActorTestKit
import akka.stream.scaladsl.Sink
import com.nigeleke.cribbage.services.{ GameJournal, GameService }
import com.typesafe.config.ConfigFactory
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AsyncWordSpecLike

class GameQuerySpec
  extends ScalaTestWithActorTestKit(ConfigFactory.parseString(
    """akka {
      |  cluster {
      |    seed-nodes = ["akka://GameQuerySpec@127.0.0.1:2551"]
      |  }
      |}
      |""".stripMargin).withFallback(ConfigFactory.load()))
  with AsyncWordSpecLike
  with Matchers {

  implicit val config = testKit.config
  implicit val log = system.log

  "A GameJournal" should {

    "return currentGames after Game creation" in {
      val service = new GameService(system)
      val journal = new GameJournal(system)
      for {
        created <- service.createGame()
        games <- journal.currentGames.runWith(Sink.seq)
      } yield (games should contain(created.id))
    }

  }

}