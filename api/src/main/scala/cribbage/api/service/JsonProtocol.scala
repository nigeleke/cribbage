package cribbage.api
package service

import model.*

import io.circe.*
import io.circe.generic.semiauto.*

import java.util.UUID

object JsonProtocol:
  given Encoder[Invitation.Id] = Encoder.encodeUUID.contramap(_.asInstanceOf[UUID])
  given Decoder[Invitation.Id] = Decoder.decodeUUID.map(_.asInstanceOf[Invitation.Id])

  given Encoder[Invitation] = deriveEncoder
  given Decoder[Invitation] = deriveDecoder
