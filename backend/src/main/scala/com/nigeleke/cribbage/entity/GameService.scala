package com.nigeleke.cribbage.entity

import akka.actor.typed.ActorSystem
import akka.persistence.typed.PersistenceId
import akka.util.Timeout
import scala.concurrent.duration._

class GameService(system: ActorSystem[_]) {

  //  REQUIRED when implementing service...
  //  import system.executionContext

  //  private val sharding = ClusterSharding(system)
  //
  //  sharding.init(Entity(typeKey = GameEntity.TypeKey) { entityContext =>
  //    GameEntity(entityContext.entityId, PersistenceId(entityContext.entityTypeKey.name, entityContext.entityId))
  //  })

  private implicit val askTimeout: Timeout = Timeout(5.seconds)

  //  def greet(worldId: String, whom: String): Future[Int] = {
  //    val entityRef = sharding.entityRefFor(HelloWorld.TypeKey, worldId)
  //    val greeting = entityRef ? HelloWorld.Greet(whom)
  //    greeting.map(_.numberOfPeople)
  //  }

}
