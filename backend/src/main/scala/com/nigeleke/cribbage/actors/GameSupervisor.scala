/*
 * Copyright (C) 2020  Nigel Eke
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

package com.nigeleke.cribbage.actors

import java.util.UUID

import akka.actor.SupervisorStrategy
import akka.actor.typed.{ActorRef, Behavior}
import akka.persistence.typed.PersistenceId
import akka.persistence.typed.scaladsl.{Effect, EventSourcedBehavior}

// SRR: Supervise the Game creation...
object GameSupervisor {

  type GameId = UUID

  sealed trait Command
  final case class CreateGame(id: GameId) extends Command

  sealed trait Query extends Command
  final case class GetGames(replyTo: ActorRef[Games]) extends Query

  sealed trait CommandReply
  final case class Games(games: Set[GameId]) extends CommandReply

  sealed trait Event
  final case class GameCreated(id: GameId) extends Event

  final case class State(games: Set[GameId])

  def apply(): Behavior[Command] =
    EventSourcedBehavior[Command, Event, State](
      persistenceId = PersistenceId.ofUniqueId("game-supervisor"),
      emptyState = State(Set.empty),
      commandHandler = onCommand,
      eventHandler = onEvent)

  def onCommand(state: State, command: Command): Effect[Event, State] =
    command match {
      case CreateGame(id) =>
        if (state.games.contains(id)) Effect.none
        else Effect.persist(GameCreated(id))

      case GetGames(replyTo: ActorRef[Games]) =>
        Effect.reply(replyTo)(Games(state.games))
    }

  def onEvent(state: State, event: Event): State =
    event match {
      case GameCreated(id) =>
        state.copy(state.games + id)
    }

}
