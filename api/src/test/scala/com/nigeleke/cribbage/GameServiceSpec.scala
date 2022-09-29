//package com.nigeleke.cribbage
//
//import com.nigeleke.cribbage.actors.*
//import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit
//import akka.persistence.typed.PersistenceId
//import akka.actor.testkit.typed.scaladsl.ActorTestKit
//import akka.actor.testkit.typed.scaladsl.ScalaTestWithActorTestKit
//import akka.pattern.StatusReply
//import akka.Done
//import com.typesafe.config.ConfigFactory
//import org.scalatest.*
//import org.scalatest.matchers.should.Matchers
//import org.scalatest.wordspec.AnyWordSpecLike
//
//class GameServiceSpec
//    extends ScalaTestWithActorTestKit(EventSourcedBehaviorTestKit.config)
//    with AnyWordSpecLike
//    with Matchers
//    with BeforeAndAfterAll
//    with BeforeAndAfterEach:
//
//  import com.nigeleke.cribbage.json.*
//
//  private val eventSourcedTestKit =
//    EventSourcedBehaviorTestKit[GameService.Command, GameService.Event, GameService.State](system, GameService())
//
//  override protected def beforeEach(): Unit =
//    super.beforeEach()
//    eventSourcedTestKit.clear()
//
//  override def afterAll(): Unit =
//    testKit.shutdownTestKit()
//    super.afterAll()
//
//  "A GameService" should {
//    "create a new game" in {
//      val result = eventSourcedTestKit.runCommand(GameService.CreateGame)
//
//      val gameCreated =
//        result.event match
//          case e: GameService.GameCreated => e
//
//      result.state match
//        case GameService.State(games) =>
//          games.size should be(1)
//          games should contain(gameCreated.ref)
//    }
//  }
