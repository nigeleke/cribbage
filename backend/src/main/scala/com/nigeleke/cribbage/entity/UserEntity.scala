package com.nigeleke.cribbage.entity

import java.util.UUID

import akka.actor.typed.{ ActorRef, Behavior }
import akka.cluster.sharding.typed.scaladsl.EntityTypeKey
import akka.pattern.StatusReply
import akka.persistence.typed.PersistenceId
import akka.persistence.typed.scaladsl.{ Effect, EventSourcedBehavior }
import com.nigeleke.cribbage.json.CborSerializable

object UserEntity {

  type Id = UUID

  trait Command extends CborSerializable
  final case class Login(name: String, replyTo: ActorRef[StatusReply[LoggedIn]]) extends Command

  trait Event extends CborSerializable
  final case class LoggedIn(name: String, userId: Id) extends Event

  final case class State(users: Map[String, Id]) extends CborSerializable

  val TypeKey: EntityTypeKey[Command] = EntityTypeKey[Command]("user")

  // Dummy entity - is just used to map a session id ("name") to a UserId (used elsewhere)
  def apply(entityId: String, persistenceId: PersistenceId): Behavior[Command] =
    EventSourcedBehavior.withEnforcedReplies[Command, Event, State](
      persistenceId = persistenceId,
      emptyState = State(Map.empty),
      commandHandler = (state: State, command: Command) => command match {
        case Login(name, replyTo) =>
          val id = state.users.getOrElse(name, UUID.fromString(persistenceId.id))
          val reply = StatusReply.Success(LoggedIn(name, id))
          if (state.users.contains(name)) Effect.none.thenReply(replyTo)(_ => reply)
          else Effect.persist(LoggedIn(name, id)).thenReply(replyTo)(_ => reply)
      },
      eventHandler = (state: State, event: Event) => event match {
        case LoggedIn(name, userId) =>
          val updatedUsers = state.users.updated(name, userId)
          State(updatedUsers)
      })
}
