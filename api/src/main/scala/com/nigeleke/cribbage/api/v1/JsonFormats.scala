package com.nigeleke.cribbage

import java.util.UUID

import com.nigeleke.cribbage.actors.GameSupervisor.Games
import spray.json.{ DefaultJsonProtocol, DeserializationException, JsString, JsValue, JsonFormat }

trait JsonFormats extends DefaultJsonProtocol {

  implicit object UUIDFormat extends JsonFormat[UUID] {
    def write(uuid: UUID) = JsString(uuid.toString)
    def read(value: JsValue) =
      value match {
        case JsString(uuid) => UUID.fromString(uuid)
        case _ => throw new DeserializationException("Expected hexadecimal UUID string")
      }
  }

  implicit val gamesJsonFormat = jsonFormat1(Games)

}
