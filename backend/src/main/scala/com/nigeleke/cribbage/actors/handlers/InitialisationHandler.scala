package com.nigeleke.cribbage.actors.handlers

import akka.persistence.typed.scaladsl.{Effect, EffectBuilder}
import com.nigeleke.cribbage.actors.Game.DeckAllocated
import com.nigeleke.cribbage.actors.handlers.Handler.{Event, State}
import com.nigeleke.cribbage.model.Deck

object InitialisationHandler extends Handler {

  def apply(state: State) : EffectBuilder[Event, State] = {
    val deck = Deck()
    Effect.persist(DeckAllocated(deck))
  }

}
