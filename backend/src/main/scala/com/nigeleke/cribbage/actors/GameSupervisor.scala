package com.nigeleke.cribbage.actors

import akka.actor.typed.scaladsl.Behaviors
import akka.actor.typed.{ActorRef, Behavior}
import com.nigeleke.cribbage.model.Game.{Id => GameId}

// SRR: Supervise the Game creation...
object GameSupervisor {

  sealed trait Command
  final case class CreateGame(id: GameId) extends Command

  sealed trait Query extends Command
  final case class GetGames(replyTo: ActorRef[Games]) extends Query

  sealed trait Response
  final case class Games(games: Seq[GameId]) extends Response

  def apply() : Behavior[Command] = repository(Seq.empty)

  def repository(games: Seq[GameId]) : Behavior[Command] = Behaviors.setup { context =>
    Behaviors.receiveMessage {
      case CreateGame(id) if !games.contains(id) =>
        context.spawn(Game(id), s"game-$id")
        repository(games :+ id)

      case GetGames(replyTo: ActorRef[Games]) =>
        replyTo ! Games(games)
        Behaviors.same

      case _ =>
        Behaviors.unhandled
    }
  }

}
