package com.nigeleke.cribbage

import java.util.UUID

import akka.Done
import akka.actor.testkit.typed.scaladsl.ScalaTestWithActorTestKit
import akka.stream.scaladsl.Sink
import com.nigeleke.cribbage.entity.GameEntity.GameCreated
import com.nigeleke.cribbage.services.{ GameJournal, GameService }
import com.typesafe.config.ConfigFactory
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AsyncWordSpecLike

class GameServiceSpec
  extends ScalaTestWithActorTestKit(ConfigFactory.parseString(
    """akka {
      |  cluster {
      |    seed-nodes = ["akka://GameServiceSpec@127.0.0.1:2551"]
      |  }
      |}
      |""".stripMargin).withFallback(ConfigFactory.load()))
  with AsyncWordSpecLike
  with Matchers {

  implicit val config = testKit.config

  "A GameService" should {

    "allow a game to be created" in {
      val service = new GameService(system)
      for {
        created <- service.createGame()
      } yield (created should be(GameCreated(created.id)))
    }

    "allow a player to join a game" in {
      val service = new GameService(system)
      val playerId = UUID.randomUUID()
      val fCreated = service.createGame()
      for {
        created <- fCreated
        joined <- service.join(created.id, playerId)
      } yield (joined should be(Done))
    }

    "provide current games" in {
      val service = new GameService(system)
      val journal = new GameJournal(system)
      val fCreated = service.createGame()
      for {
        created <- fCreated
        fGames = journal.currentGames
        gameIds <- fGames.runWith(Sink.seq)
      } yield (gameIds should contain(created.id))
    }

    "allow a player to view the cards in their hand" ignore {
      val service = new GameService(system)
      val player1Id = UUID.randomUUID()
      val player2Id = UUID.randomUUID()
      for {
        created <- service.createGame().mapTo[GameCreated]
        gameId = created.id
        _ <- service.join(gameId, player1Id)
        _ <- service.join(gameId, player2Id)
        // hand <- journal.hand(gameId, player1Id).mapTo[Hand]
      } yield ??? //(hand.size should be(6))
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
