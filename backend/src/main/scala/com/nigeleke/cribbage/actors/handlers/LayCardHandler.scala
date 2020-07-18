package com.nigeleke.cribbage.actors.handlers

import akka.persistence.typed.scaladsl.{Effect, EffectBuilder}
import com.nigeleke.cribbage.actors.Game.{CardLaid, LayCard}
import com.nigeleke.cribbage.actors.handlers.Handler.{Event, State}

object PlayCardHandler extends Handler {

  def apply(state: State, play: LayCard) : EffectBuilder[Event, State] =
    Effect.persist(CardLaid(play.playerId, play.card))

}
