package cribbage.api
package service

import model.*

import io.circe.generic.auto.*
import sttp.model.*
import sttp.tapir.*
import sttp.tapir.generic.auto.*
import sttp.tapir.json.circe.*
import sttp.tapir.model.*

object Endpoints:
  import JsonProtocol.{given}

  private val base = endpoint.in("api" / "v1")

  private val gameBase = base.in("game").tag("Cribbage")

  private[service] val invitations = gameBase.get
    .name("Invitations")
    .description("Get a list of game invitations")
    .out(statusCode(StatusCode.Ok))
    .out(jsonBody[List[Invitation]])
    .errorOut(statusCode(StatusCode.InternalServerError))
    .errorOut(stringBody)

  private[service] val createInvitation = gameBase.post
    .name("Create Invitation")
    .description("Create an invitation to a new game")
    .out(statusCode(StatusCode.Created))
    .out(jsonBody[Invitation.Id])
    .errorOut(statusCode(StatusCode.InternalServerError))
    .errorOut(stringBody)

  val all = List(
    invitations,
    createInvitation
  )
