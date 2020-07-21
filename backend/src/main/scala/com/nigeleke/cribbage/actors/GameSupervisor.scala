package com.nigeleke.cribbage.actors

import akka.actor.typed.{ActorRef, Behavior}
import akka.persistence.typed.PersistenceId
import akka.persistence.typed.scaladsl.{Effect, EventSourcedBehavior}
import com.nigeleke.cribbage.model.Game.{Id => GameId}

// SRR: Supervise the Game creation...
object GameSupervisor {

  sealed trait Command
  final case class CreateGame(id: GameId) extends Command

  sealed trait Query extends Command
  final case class GetGames(replyTo: ActorRef[Games]) extends Query

  sealed trait CommandReply
  final case class Games(games: Set[GameId]) extends CommandReply

  sealed trait Event
  final case class GameCreated(id: GameId) extends Event

  final case class State(games: Set[GameId])

  def apply() : Behavior[Command] =
    EventSourcedBehavior[Command, Event, State](
      persistenceId = PersistenceId.ofUniqueId("game-supervisor"),
      emptyState = State(Set.empty),
      commandHandler = onCommand,
      eventHandler = onEvent
    )

  def onCommand(state: State, command: Command) : Effect[Event, State] =
    command match {
      case CreateGame(id) =>
        if (state.games.contains(id)) Effect.none
        else Effect.persist(GameCreated(id))

      case GetGames(replyTo: ActorRef[Games]) =>
        Effect.reply(replyTo)(Games(state.games))
    }

  def onEvent(state: State, event: Event) : State =
    event match {
      case GameCreated(id) =>
        state.copy(state.games + id)
    }

}
