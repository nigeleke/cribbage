package com.nigeleke.cribbage

import akka.actor.testkit.typed.scaladsl.{ ActorTestKit, LogCapturing, ScalaTestWithActorTestKit }
import akka.persistence.testkit.scaladsl.{ EventSourcedBehaviorTestKit, PersistenceTestKit }
import akka.stream.scaladsl.Sink
import com.nigeleke.cribbage.TestModel.randomId
import com.nigeleke.cribbage.entity.{ GameEntity, GameJournal }
import com.nigeleke.cribbage.entity.GameEntity._
import com.typesafe.config.ConfigFactory
import org.scalatest.{ BeforeAndAfterAll, BeforeAndAfterEach }
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.{ AsyncWordSpec, AsyncWordSpecLike }

class QuerySpec
  extends AsyncWordSpec
  with BeforeAndAfterAll
  with Matchers {

  val testKit = ActorTestKit(ConfigFactory.parseResources("reference-test.conf").resolve())
  implicit val system = testKit.system
  implicit val ec = system.executionContext
  implicit val config = testKit.config

  val probe = testKit.createTestProbe[Reply]()

  override def afterAll(): Unit = testKit.shutdownTestKit()

  "Persisted GameEntity(s)" should {

    "not be created initially" in {
      testKit.spawn(GameEntity(Idle("game-test")))
      val journal = GameJournal()
      val fGames = journal.currentGames.runWith(Sink.seq)
      fGames.map(games => games.size should be(0))
    }

    "be retrievable after being created" in {
      val id = randomId
      val gut = testKit.spawn(GameEntity(Idle(id.toString)))
      gut ! GameEntity.CreateGame(probe.ref)
      probe.expectMessage(Accepted)
      val journal = new GameJournal()
      val fGames = journal.currentGames.runWith(Sink.seq)
      fGames.map(games => games.size should be(1))
    }

  }

}