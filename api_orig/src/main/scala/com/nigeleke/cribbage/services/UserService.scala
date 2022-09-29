package com.nigeleke.cribbage.services

import java.util.UUID

import akka.actor.typed.ActorSystem
import akka.cluster.sharding.typed.scaladsl.{ ClusterSharding, Entity, EntityRef }
import akka.persistence.typed.PersistenceId
import akka.util.Timeout
import com.nigeleke.cribbage.entity.UserEntity
import com.nigeleke.cribbage.entity.UserEntity._

import scala.concurrent.Future
import scala.concurrent.duration._

class UserService(system: ActorSystem[_]) {

  implicit private val log = system.log

  private val sharding = ClusterSharding(system)

  sharding.init(Entity(typeKey = UserEntity.TypeKey)(createBehavior = { entityContext =>
    UserEntity(entityContext.entityId, PersistenceId.ofUniqueId(entityContext.entityId))
  }))

  private implicit val askTimeout = Timeout(5.seconds)

  def login(name: String): Future[LoggedIn] = {
    val id = UUID.randomUUID()
    entityRefFor(id).askWithStatus(Login(name, _)).mapTo[LoggedIn]
  }

  private def entityRefFor(id: UUID): EntityRef[Command] = sharding.entityRefFor[Command](UserEntity.TypeKey, id.toString)

}
