package com.nigeleke.cribbage.actors

import akka.actor.typed.{ActorRef, Behavior}
import akka.actor.typed.scaladsl.Behaviors
import com.nigeleke.cribbage.actors.rules.CutForDealRule

// SRR: Arbitrate the rules during the course of the Game
object RuleBook {

  type Command = Game.Event

  type Event = Game.Event

  def apply(notify: ActorRef[Event]) : Behavior[Command] = Behaviors.setup { context =>

    def rules(rules: ActorRef[Event]*) : Behavior[Command] =
      Behaviors.receiveMessage {
        case any =>
          rules.foreach(rule => rule ! any)
          Behaviors.same
      }

    rules(
      context.spawn(CutForDealRule(notify), s"cut-for-deal-rule")
    )
  }

}
