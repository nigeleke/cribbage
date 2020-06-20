package com.nigeleke.cribbage.actors.states.game

import akka.actor.typed.Behavior
import akka.actor.typed.eventstream.EventStream.Publish
import akka.actor.typed.scaladsl.Behaviors
import com.nigeleke.cribbage.actors.GameFacade._
import com.nigeleke.cribbage.model.Player.{Id => PlayerId}

object StartingGame {

  def apply() : Behavior[Command] = starting(Set.empty)

  def starting(players: Set[PlayerId]) : Behavior[Command] = Behaviors.setup { context =>

    def playerCanJoin(id: PlayerId) = players.size < 2 && !players.contains(id)

    Behaviors.receiveMessage {
      case Join(playerId) if playerCanJoin(playerId) =>
        context.system.eventStream ! Publish(PlayerJoined(playerId))
        starting(players + playerId)

      case GetPlayers(replyTo) =>
        replyTo ! players
        Behaviors.same

      case _ =>
        Behaviors.unhandled
    }

  }
}
