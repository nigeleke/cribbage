package com.nigeleke.cribbage.actors.handlers

import akka.persistence.typed.scaladsl.{Effect, EffectBuilder}
import com.nigeleke.cribbage.actors.Game.PlayCutRevealed
import com.nigeleke.cribbage.actors.handlers.Handler.{Event, State}
import com.nigeleke.cribbage.model.Deck._

object CutAtStartOfPlayHandler extends Handler {

  def apply(state: State) : EffectBuilder[Event, State] = {
    require(state.game.deck.size == 40)
    val game = state.game
    val deck = game.deck.shuffled
    val cut = deck.head // Will always be Some[Card]
    Effect.persist(PlayCutRevealed(cut))
  }
}
