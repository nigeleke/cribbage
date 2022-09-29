//package com.nigeleke.cribbage.actors
//
//import akka.actor.typed.*
//import akka.actor.typed.scaladsl.Behaviors
//import akka.persistence.typed.scaladsl.EventSourcedBehavior
//import akka.persistence.typed.PersistenceId
//import akka.persistence.typed.scaladsl.Effect
//import com.nigeleke.cribbage.domain.Game
//
//import java.util.UUID
//
//object GameActor:
//
//  sealed trait Command
//  sealed trait Query extends Command
//
//  sealed trait CommandReply
//
//  sealed trait Event
//
//  final case class State(game: Game)
//
//  def apply(): Behavior[Command] =
//    EventSourcedBehavior[Command, Event, State](
//      persistenceId = PersistenceId.ofUniqueId("game"),
//      emptyState = State(Game()),
//      commandHandler = onCommand,
//      eventHandler = onEvent
//    )
//
//  def onCommand(state: State, command: Command): Effect[Event, State] = ???
//
//  def onEvent(state: State, event: Event): State = ???
