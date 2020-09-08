package com.nigeleke.cribbage

import akka.Done
import akka.actor.testkit.typed.scaladsl.{ LogCapturing, ScalaTestWithActorTestKit }
import akka.persistence.query.PersistenceQuery
import akka.persistence.testkit.scaladsl.{ EventSourcedBehaviorTestKit, PersistenceTestKit }
import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit.SerializationSettings
import akka.stream.scaladsl.Sink
import com.nigeleke.cribbage.TestModel.randomId
import com.nigeleke.cribbage.actors.{ Game, GameJournal }
import com.nigeleke.cribbage.actors.Game._
import com.typesafe.config.{ Config, ConfigFactory }
import org.scalatest.BeforeAndAfterEach
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AsyncWordSpecLike

import scala.concurrent.Future

class QuerySpec
  extends ScalaTestWithActorTestKit()
  with AsyncWordSpecLike
  with BeforeAndAfterEach
  with LogCapturing
  with Matchers {

  implicit val ec = system.executionContext

//  override protected def beforeEach(): Unit = {
//    super.beforeEach()
//    eventSourcedTestKit.clear()
//  }

  "Persisted Game(s)" should {

    "not be created initially" in {
      spawn(Game())
      val journal = new GameJournal()
      val fGames = journal.currentGames.runWith(Sink.seq)
      fGames.map(games => games.size should be(0))
    }

    "be retrievable after being created" in {
      {
        val gut = spawn(Game())
        val id = randomId
        gut ! Game.CreateGame(id)
      }
      {
        val journal = new GameJournal()
        val fGames = journal.currentGames.runWith(Sink.seq)
        fGames.map(games => games.size should be(1))
      }
    }

    //    "not be createds" when {
    //      "no Games have been created" in {
    //        val result = eventSourcedTestKit.runCommand(GetGames)
    //        result.reply should be(Games(Set.empty))
    //        result.events should be(empty)
    //        result.state should be(State(Set.empty))
    //      }
    //
    //      "a new Attributes is created" in {
    //        val gameId = UUID.randomUUID()
    //        val result = eventSourcedTestKit.runCommand(CreateGame(gameId))
    //        result.events should contain allElementsOf (Seq(GameCreated(gameId)))
    //        result.state should be(State(Set(gameId)))
    //
    //        val result2 = eventSourcedTestKit.runCommand(GetGames)
    //        result2.reply should be(Games(Set(gameId)))
    //        result2.events should be(empty)
    //        result2.state should be(State(Set(gameId)))
    //      }
    //
    //      "a Attributes is added more than once" in {
    //        val gameId = UUID.randomUUID()
    //        val results = Seq(CreateGame(gameId), CreateGame(gameId)).map(eventSourcedTestKit.runCommand)
    //        results.flatMap(_.events) should contain theSameElementsInOrderAs (Seq(GameCreated(gameId)))
    //        results.last.state should be(State(Set(gameId)))
    //
    //        val result2 = eventSourcedTestKit.runCommand(GetGames)
    //        result2.reply should be(Games(Set(gameId)))
    //        result2.events should be(empty)
    //        result2.state should be(State(Set(gameId)))
    //      }
    //

  }

}