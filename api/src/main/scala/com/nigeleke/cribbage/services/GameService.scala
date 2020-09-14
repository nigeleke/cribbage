package com.nigeleke.cribbage.services

import akka.actor.typed.ActorSystem
//import akka.cluster.typed._
import akka.util.Timeout

class GameService(system: ActorSystem[_]) {

  //  private implicit val log = system.log
  //
//  val TypeKey = Entity

//  sharding.init(Entity(typeKey = TypeKey) { entityContext =>
//    GameEntity(PersistenceId(entityContext.entityTypeKey.name, entityContext.entityId))
//  })

  private implicit val askTimeout: Timeout = ??? //Timeout(5.seconds)

//  def createGame(): Future[Reply] = {
//    val id = UUID.randomUUID()
//    val entityRef = sharding.entityRefFor(TypeKey, id)
//    entityRef ? CreateGame
//  }
  //
  //  def join(gameId: GameEntityId, playerId: PlayerId): Future[Reply] = {
  //    val entityRef = sharding.entityRefFor(TypeKey, gameId)
  //    entityRef ? (Join(playerId, _))
  //  }
  //
  //  def discard(gameId: GameEntityId, playerId: PlayerId, cards: CardIds) = {
  //    val entityRef = sharding.entityRefFor(TypeKey, gameId)
  //    entityRef ? (DiscardCribCards(playerId, cards, _))
  //  }
  //
  //  //  final case class LayCard(playerId: PlayerId, cardId: Card, replyTo: ActorRef[Reply]) extends Command
  //  //  final case class Pass(playerId: PlayerId, replyTo: ActorRef[Reply]) extends Command
  //
  //  implicit private def uuidToString(id: UUID): String = id.toString
  //  implicit private def stringToUUID(name: String): UUID = UUID.fromString(name)

}
