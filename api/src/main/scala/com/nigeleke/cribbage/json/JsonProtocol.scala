//package com.nigeleke.cribbage.json
//
//import com.nigeleke.cribbage.actors.*
//import com.nigeleke.cribbage.domain.*
//
//import akka.http.scaladsl.marshallers.sprayjson.SprayJsonSupport
//import spray.json.*
//import spray.json.DefaultJsonProtocol
//
//import java.util.UUID
//
//implicit object UUIDFormat extends JsonFormat[UUID]:
//  def write(uuid: UUID) = JsString(uuid.toString)
//  def read(value: JsValue) =
//    value match
//      case JsString(uuid) => UUID.fromString(uuid)
//      case _              => throw new DeserializationException("Expected hexadecimal UUID string")
//
//object JsonProtocol:
//  import DefaultJsonProtocol.*
//  given RootJsonFormat[GameService.CreateGame.type] = jsonFormat0(() => GameService.CreateGame)
//  given RootJsonFormat[GameService.GameCreated.type] = jsonFormat0(() => GameService.GameCreated)
////  given RootJsonFormat[GameListItem] = jsonFormat1(GameListItem.apply)
////  given RootJsonFormat[GameList] = jsonFormat1(GameList.apply)
