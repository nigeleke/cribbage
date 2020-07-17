package com.nigeleke.cribbage.actors.handlers

import akka.persistence.typed.scaladsl.{Effect, EffectBuilder}
import com.nigeleke.cribbage.actors.Game.{CardPlayed, PlayCard}
import com.nigeleke.cribbage.actors.handlers.Handler.{Event, State}

object PlayCardHandler extends Handler {

  def apply(state: State, play: PlayCard) : EffectBuilder[Event, State] =
    Effect.persist(CardPlayed(play.playerId, play.card))

}
