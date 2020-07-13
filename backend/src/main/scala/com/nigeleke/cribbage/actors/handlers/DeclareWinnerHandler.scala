package com.nigeleke.cribbage.actors.handlers

import akka.persistence.typed.scaladsl.{Effect, EffectBuilder}
import com.nigeleke.cribbage.actors.Game.{DeclareWinner, WinnerDeclared}
import com.nigeleke.cribbage.actors.handlers.Handler.{Event, State}

object DeclareWinnerHandler extends Handler {

  def apply(state: State, winner: DeclareWinner) : EffectBuilder[Event, State] =
    Effect.persist(WinnerDeclared(winner.playerId))

}
