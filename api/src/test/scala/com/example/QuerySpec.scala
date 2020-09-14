package com.example

import akka.actor.testkit.typed.scaladsl.ActorTestKit
import akka.pattern.StatusReply
import akka.stream.scaladsl.Sink
import com.nigeleke.cribbage.TestModel._
import com.nigeleke.cribbage.entity.GameEntity
import com.nigeleke.cribbage.services.GameJournal
import com.typesafe.config.ConfigFactory
import org.scalatest.BeforeAndAfterAll
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AsyncWordSpec

class QuerySpec
  extends AsyncWordSpec
  with BeforeAndAfterAll
  with Matchers {

  val testKit = ActorTestKit(ConfigFactory.parseResources("reference.conf").resolve())
  implicit val system = testKit.system
  implicit val ec = system.executionContext
  implicit val config = testKit.config
  implicit val log = system.log

  val probe = testKit.createTestProbe[StatusReply[_]]()

  override def afterAll(): Unit = testKit.shutdownTestKit()

  "Persisted GameEntity(s)" should {

    "not be created initially" in {
      testKit.spawn(GameEntity(randomId))
      val journal = GameJournal()
      val fGames = journal.currentGames.runWith(Sink.seq)
      fGames.map(games => games.size should be(0))
    }

    "be retrievable after being created" in {
      val entity = testKit.spawn(GameEntity(randomId))
      entity ! GameEntity.CreateGame(probe.ref)
      probe.expectMessage(StatusReply.Success(GameEntity.CreateGame(probe.ref)))
      val journal = new GameJournal()
      val fGames = journal.currentGames.runWith(Sink.seq)
      fGames.map(games => games.size should be(1))
    }

  }

}