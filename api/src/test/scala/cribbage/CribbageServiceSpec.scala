package cribbage.api

import model.*
import repository.*
import service.*

import cats.effect.*
import cats.effect.testing.scalatest.*
import org.http4s.*
import org.http4s.circe.*
import org.http4s.circe.CirceEntityCodec.*
import org.http4s.implicits.*
import org.scalatest.*
import org.scalatest.matchers.should.*
import org.scalatest.wordspec.*
import sttp.tapir.client.http4s.*
import sttp.tapir.testing.*

class CribbageServiceSpec extends AsyncWordSpec with AsyncIOSpec with Matchers:

  import JsonProtocol.{given}

  given Repository with {
    var invitations_                           = scala.collection.mutable.Seq.empty[Invitation]
    override def invitations: List[Invitation] = invitations_.toList
    override def add(invitation: Invitation)   =
      invitations_ = invitations_ :+ invitation
      Right(invitation)
  }

  "The CribbageService" should {

    "provide unique services" in {
      val test = EndpointVerifier(Endpoints.all)
      test should be(empty)
    }

    "provide a list of game invitations" in {
      val request = Request[IO](Method.GET, uri"/api/v1/game/")
      for
        response <- CribbageService.service(request)
        body     <- response.decodeJson[List[Invitation]]
      yield
        response.status should be(Status.Ok)
        body should be(empty)
    }

    "allow a new invitation to be made" in {
      val createRequest = Request[IO](Method.POST, uri"/api/v1/game/")
      val getRequest    = Request[IO](Method.GET, uri"/api/v1/game/")
      for
        createResponse <- CribbageService.service(createRequest)
        createBody     <- createResponse.decodeJson[Invitation.Id]
        getResponse    <- CribbageService.service(getRequest)
        getBody        <- getResponse.decodeJson[List[Invitation]]
      yield
        createResponse.status should be(Status.Created)
        createBody should be(a[Invitation.Id])
        getResponse.status should be(Status.Ok)
        getBody.map(_.id) should contain(createBody)
    }

    "accept Player commands" when {
      "join inactive game" ignore { fail() }
      "valid discard" ignore { fail() }
      "valid play" ignore { fail() }
      "valid pass" ignore { fail() }
    }

    "reject Player commands" when {
      "join active game" ignore { fail() }
      "invalid discard" ignore { fail() }
      "invalid play" ignore { fail() }
      "invalid pass" ignore { fail() }
    }
  }
