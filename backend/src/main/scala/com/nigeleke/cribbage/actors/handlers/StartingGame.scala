package com.nigeleke.cribbage.actors.handlers

import akka.actor.typed.{ActorRef, Behavior}
import akka.actor.typed.scaladsl.Behaviors
import com.nigeleke.cribbage.actors.Game
import com.nigeleke.cribbage.actors.Game._
import com.nigeleke.cribbage.model.Player.{Id => PlayerId}

object StartingGame {

  def apply(notify: ActorRef[Game.Event]) : Behavior[Command] = starting(notify, Set.empty)

  private def starting(notify: ActorRef[Game.Event],
                       players: Set[PlayerId]) : Behavior[Command] = {

    def playerCanJoin(id: PlayerId) = players.size < 2 && !players.contains(id)

    Behaviors.receiveMessage {
      case Join(playerId) if playerCanJoin(playerId) =>
        notify ! PlayerJoined(playerId)
        starting(notify, players + playerId)

      case Players(replyTo) =>
        replyTo ! players
        Behaviors.same

      case other =>
        println(s"Starting $other")
        Behaviors.unhandled
    }

  }

}
