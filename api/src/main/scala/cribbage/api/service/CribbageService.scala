package cribbage.api
package service

import model.*
import repository.*
import cats.effect.*
import cats.syntax.all.*
import cribbage.api.model.Invitation.Id
import org.http4s.*
import org.http4s.server.*
import sttp.capabilities.fs2.*
import sttp.tapir.*
import sttp.tapir.server.*
import sttp.tapir.server.http4s.*
import sttp.tapir.server.ServerEndpoint.Full

object CribbageService:

  private def getInvitations(unused: Unit)(using
      repository: Repository
  ): IO[Either[String, List[Invitation]]] =
    Right(repository.invitations).pure

  private def createInvitation(unused: Unit)(using
      repository: Repository
  ): IO[Either[String, Invitation.Id]] =
    (repository.add(Invitation.create).map(_.id)).pure

  def service(using Repository) =
    val logic = List[ServerEndpoint[Fs2Streams[IO], IO]](
      Endpoints.invitations.serverLogic(getInvitations),
      Endpoints.createInvitation.serverLogic(createInvitation)
    )

    Http4sServerInterpreter[IO]().toRoutes(logic).orNotFound
