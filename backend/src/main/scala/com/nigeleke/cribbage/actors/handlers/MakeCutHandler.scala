package com.nigeleke.cribbage.actors.handlers

import akka.persistence.typed.scaladsl.{Effect, EffectBuilder}
import com.nigeleke.cribbage.actors.Game.CutMade
import com.nigeleke.cribbage.actors.handlers.Handler.{Event, State}
import com.nigeleke.cribbage.model.Deck._

object MakeCutHandler extends Handler {

  def apply(state: State) : EffectBuilder[Event, State] = {
    val game = state.game
    val deck = game.deck.shuffled
    val cut = deck.head // Will always be Some
    Effect.persist(CutMade(cut))
  }
}
