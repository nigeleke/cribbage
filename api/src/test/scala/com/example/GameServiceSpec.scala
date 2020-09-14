package com.example

import akka.actor.testkit.typed.scaladsl.ScalaTestWithActorTestKit
import com.nigeleke.cribbage.services.GameService
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpecLike

class GameServiceSpec extends ScalaTestWithActorTestKit() with AnyWordSpecLike with Matchers {

  //  lazy val testKit = ActorTestKit()
  //  implicit def typedSystem = testKit.system
  //  override def createActorSystem(): akka.actor.ActorSystem = testKit.system.toClassic
  //
  //  val userRegistry = testKit.spawn(UserRegistry())
  //
  //  // use the json formats to marshal and unmarshall objects in the test
  //  import akka.http.scaladsl.marshallers.sprayjson.SprayJsonSupport._
  //  import JsonFormats._
  //  //#set-up

  val service = new GameService(system)

  "A GameService" should {

    "allow a game to be created" in {
     // service.createGame()

    }

    //    "return no users if no present (GET /users)" in {
    //      // note that there's no need for the host part in the uri:
    //      val request = HttpRequest(uri = "/users")
    //
    //      request ~> routes ~> check {
    //        status should ===(StatusCodes.OK)
    //
    //        // we expect the response to be json:
    //        contentType should ===(ContentTypes.`application/json`)
    //
    //        // and no entries should be in the list:
    //        entityAs[String] should ===("""{"users":[]}""")
    //      }
    //    }
    //    //#actual-test
    //
    //    //#testing-post
    //    "be able to add users (POST /users)" in {
    //      val user = User("Kapi", 42, "jp")
    //      val userEntity = Marshal(user).to[MessageEntity].futureValue // futureValue is from ScalaFutures
    //
    //      // using the RequestBuilding DSL:
    //      val request = Post("/users").withEntity(userEntity)
    //
    //      request ~> routes ~> check {
    //        status should ===(StatusCodes.Created)
    //
    //        // we expect the response to be json:
    //        contentType should ===(ContentTypes.`application/json`)
    //
    //        // and we know what message we're expecting back:
    //        entityAs[String] should ===("""{"description":"User Kapi created."}""")
    //      }
    //    }
    //    //#testing-post
    //
    //    "be able to remove users (DELETE /users)" in {
    //      // user the RequestBuilding DSL provided by ScalatestRouteSpec:
    //      val request = Delete(uri = "/users/Kapi")
    //
    //      request ~> routes ~> check {
    //        status should ===(StatusCodes.OK)
    //
    //        // we expect the response to be json:
    //        contentType should ===(ContentTypes.`application/json`)
    //
    //        // and no entries should be in the list:
    //        entityAs[String] should ===("""{"description":"User Kapi deleted."}""")
    //      }
    //    }
    //    //#actual-test
    //  }

  }

}
