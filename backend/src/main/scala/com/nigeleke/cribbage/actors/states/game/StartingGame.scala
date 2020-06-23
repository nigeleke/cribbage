package com.nigeleke.cribbage.actors.states.game

import akka.actor.typed.Behavior
import akka.actor.typed.eventstream.EventStream.Publish
import akka.actor.typed.scaladsl.Behaviors
import com.nigeleke.cribbage.actors.Game._
import com.nigeleke.cribbage.model.Game.{Id => GameId}
import com.nigeleke.cribbage.model.Player.{Id => PlayerId}

object StartingGame {

  def apply(id: GameId) : Behavior[Command] = starting(id, Set.empty)

  def starting(gameId: GameId, players: Set[PlayerId]) : Behavior[Command] = Behaviors.setup { context =>

    def playerCanJoin(id: PlayerId) = players.size < 2 && !players.contains(id)

    Behaviors.receiveMessage {
      case Join(playerId) if playerCanJoin(playerId) =>
        context.system.eventStream ! Publish(PlayerJoined(gameId, playerId))
        starting(gameId, players + playerId)

      case Players(replyTo) =>
        replyTo ! players
        Behaviors.same

      case _ =>
        Behaviors.unhandled
    }

  }
}
