//package com.nigeleke.cribbage
//
//import com.nigeleke.cribbage.server.*
//import akka.Done
//import akka.actor.testkit.typed.scaladsl.ScalaTestWithActorTestKit
//import akka.actor.typed.ActorSystem
//import akka.http.scaladsl.model.StatusCodes
//import akka.http.scaladsl.model.headers.BasicHttpCredentials
//import akka.http.scaladsl.testkit.ScalatestRouteTest
//import akka.http.scaladsl.server.*
//
//import com.typesafe.config.ConfigFactory
//import org.scalatest.matchers.should.Matchers
//import org.scalatest.wordspec.AnyWordSpec
//
//import java.util.UUID
//import scala.concurrent.Future
//
//class GameServerSpec
////    extends ScalaTestWithActorTestKit(ConfigFactory.defaultApplication())
//    extends AnyWordSpec
//    with Matchers
//    with ScalatestRouteTest:
//
//  "The GameServer" should {
//
//    val server = GameServer
//    val routes = Route.seal(server.routes)
//
//    val validCredentials = BasicHttpCredentials("user", "secret")
//    val invalidCredentials = BasicHttpCredentials("notuser", "notpassword")
//
//    "return the current User's active Games for GET requests to the /game path" ignore {
//      Get("/game") ~> addCredentials(validCredentials) ~> routes ~> check {
//        status should be(StatusCodes.OK)
//        fail()
////        responseAs[GameList] shouldEqual (GameList(Seq.empty[GameListItem]))
//      }
//    }
//
//    "reject an unauthorised user's GET requests to the /game path" in {
//      Get("/game") ~> addCredentials(invalidCredentials) ~> routes ~> check {
//        status should be(StatusCodes.Unauthorized)
//      }
//    }
//
//    "allow the current User to create a Game for POST requests to the /game path" ignore {
//      Post("/game") ~> addCredentials(validCredentials) ~> routes ~> check {
//        status should be(StatusCodes.Created)
//        fail()
//      }
//    }
//
//  }
