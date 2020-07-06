package com.nigeleke.cribbage.actors.rules

import akka.actor.typed.ActorRef
import com.nigeleke.cribbage.actors.Game

trait Rule {

  import Rule._
  def apply(state: State)(implicit notify: ActorRef[Command]) =
    commands(state).foreach(command => notify ! command)

  def commands(state: State) : Seq[Command]
}

object Rule {

  type Command = Game.Command
  type Event = Game.Event
  type State = Game.State

}
