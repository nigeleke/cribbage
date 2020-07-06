package com.nigeleke.cribbage.actors.handlers

import com.nigeleke.cribbage.actors.Game

// SRR: Handlers raise Events for the Command they represent...
trait Handler {
//  import Handler._
//  def apply(state: State) : EffectBuilder[Event, State]
// TODO: Sort out what a handler trait really is...
}

object Handler {

  type Command = Game.Command
  type Event = Game.Event
  type State = Game.State
  
}
