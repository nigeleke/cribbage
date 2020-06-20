package com.nigeleke.cribbage.actors

import akka.actor.typed.eventstream.EventStream.Publish
import akka.actor.typed.scaladsl.Behaviors
import akka.actor.typed.{ActorRef, Behavior}
import com.nigeleke.cribbage.model.Game.{Id => GameId}

object GameSupervisor {

  sealed trait Command
  final case class CreateGame(id: GameId, ackTo: ActorRef[GameCreated]) extends Command

  sealed trait Query extends Command
  final case class GetGames(replyTo: ActorRef[Games]) extends Query

  sealed trait Response
  final case class Games(games: Seq[GameId]) extends Response

  sealed trait Event extends Response
  final case class GameCreated(id: GameId) extends Event

  def apply() : Behavior[Command] = repository(Seq.empty)

  def repository(games: Seq[GameId]) : Behavior[Command] = Behaviors.setup { context =>
    Behaviors.receiveMessage {
      case CreateGame(id, _) if games.contains(id) =>
        Behaviors.same

      case CreateGame(id, ackTo) =>
        val event = GameCreated(id)
        ackTo ! event
        context.system.eventStream ! Publish(event)
        repository(games :+ id)

      case GetGames(replyTo: ActorRef[Games]) =>
        replyTo ! Games(games)
        Behaviors.same
    }
  }

}
