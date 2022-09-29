package com.nigeleke.cribbage.services

import java.util.UUID

import akka.actor.typed.ActorSystem
import akka.cluster.sharding.typed.scaladsl.{ ClusterSharding, Entity, EntityRef }
import akka.persistence.typed.PersistenceId
import akka.util.Timeout
import com.nigeleke.cribbage.entity.GameEntity
import com.nigeleke.cribbage.entity.GameEntity._

import scala.concurrent.Future
import scala.concurrent.duration._

class GameService(system: ActorSystem[_]) {

  implicit private val log = system.log

  private val sharding = ClusterSharding(system)

  sharding.init(Entity(typeKey = GameEntity.TypeKey)(createBehavior = { entityContext =>
    GameEntity(entityContext.entityId, PersistenceId.ofUniqueId(entityContext.entityId))
  }))

  private implicit val askTimeout = Timeout(5.seconds)

  def createGame(): Future[GameCreated] = {
    val id = UUID.randomUUID()
    entityRefFor(id).askWithStatus(CreateGame(_)).mapTo[GameCreated]
  }

  private def entityRefFor(id: UUID): EntityRef[Command] = sharding.entityRefFor[Command](GameEntity.TypeKey, id.toString)

  def join(gameId: UUID, playerId: UUID): Future[_] = {
    entityRefFor(gameId).askWithStatus(Join(playerId, _))
  }

  //  def discard(gameId: GameEntityId, playerId: Player.Id, cards: CardIds) = {
  //    val entityRef = sharding.entityRefFor(TypeKey, gameId)
  //    entityRef ? (DiscardCribCards(playerId, cards, _))
  //  }
  //
  //  //  final case class LayCard(playerId: Player.Id, cardId: Card, replyTo: ActorRef[Reply]) extends Command
  //  //  final case class Pass(playerId: Player.Id, replyTo: ActorRef[Reply]) extends Command
  //
  //  implicit private def uuidToString(id: UUID): String = id.toString
  //  implicit private def stringToUUID(name: String): UUID = UUID.fromString(name)

}
