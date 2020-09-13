package com.nigeleke.cribbage.api.v1

import java.util.UUID

import akka.actor.typed.ActorSystem
import akka.persistence.typed.PersistenceId
import akka.util.Timeout
import com.nigeleke.cribbage.entity.GameEntity
import com.nigeleke.cribbage.entity.GameEntity.{ Id => GameEntityId, _ }
import com.nigeleke.cribbage.model.Player.{ Id => PlayerId }
import com.nigeleke.cribbage.model._

import scala.concurrent.Future
import scala.concurrent.duration._

class GameService(system: ActorSystem[_]) {

  //  private implicit val log = system.log
  //
  //  sharding.init(Entity(typeKey = TypeKey) { entityContext =>
  //    GameEntity(entityContext.entityId, PersistenceId(entityContext.entityTypeKey.name, entityContext.entityId))
  //  })
  //
  //  private implicit val askTimeout: Timeout = Timeout(5.seconds)
  //
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
