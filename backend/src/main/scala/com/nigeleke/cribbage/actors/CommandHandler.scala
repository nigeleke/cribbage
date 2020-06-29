package com.nigeleke.cribbage.actors

import akka.actor.typed.scaladsl.Behaviors
import akka.actor.typed.{ActorRef, Behavior}
import com.nigeleke.cribbage.actors

// SRR: Forward Game.Commands to the Game CommandHandlers
object CommandHandler {

  type Command = Game.Command

  type Event = Game.Event

  def apply(notify: ActorRef[Event]) : Behavior[Command] = Behaviors.setup { context =>

    def handlers(handlers: ActorRef[Command]*) : Behavior[Command] =
      Behaviors.receiveMessage {
        case any =>
          handlers.foreach(handler => handler ! any)
          Behaviors.same
      }

    handlers(
      context.spawn(actors.handlers.StartingGame(notify), "starting"),
      context.spawn(actors.handlers.Discarding(notify), "discarding")
    )
  }
}
