package com.nigeleke.cribbage.actors

import akka.actor.typed.scaladsl.Behaviors
import akka.actor.typed.{ActorRef, Behavior}
import com.nigeleke.cribbage.model.Game
import com.nigeleke.cribbage.model.Game.{Id => GameId}

object GameSupervisor {

  sealed trait Command
  final case class CreateGame(id: GameId, ackTo: ActorRef[GameCreated]) extends Command

  sealed trait Query extends Command
  final case class GetGames(replyTo: ActorRef[Games]) extends Query

  sealed trait Response
  final case class GameCreated(id: GameId) extends Response
  final case class Games(games: Seq[GameId]) extends Response

  def apply() : Behavior[Command] = repository(Seq.empty)

  def repository(games: Seq[GameId]) : Behavior[Command] = Behaviors.receiveMessage {
    case CreateGame(id, _) if games.contains(id) =>
      Behaviors.same

    case CreateGame(id, ackTo) =>
      ackTo ! GameCreated(id)
      repository(games :+ id)

    case GetGames(replyTo: ActorRef[Games]) =>
      replyTo ! Games(games)
      Behaviors.same
  }

}
