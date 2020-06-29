package com.nigeleke.cribbage.actors.handlers

import akka.actor.typed.scaladsl.Behaviors
import akka.actor.typed.{ActorRef, Behavior}
import com.nigeleke.cribbage.actors.Game._

// SRR: MakeCut when Players have discarded Crib cards...
object Discarding {

  def apply(notify: ActorRef[Event]) : Behavior[Command] = Behaviors.setup { context =>
    Behaviors.receiveMessage {
      case any =>
        Behaviors.same
    }

  }

}
