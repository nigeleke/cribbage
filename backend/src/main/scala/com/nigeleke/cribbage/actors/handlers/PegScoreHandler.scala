package com.nigeleke.cribbage.actors.handlers

import akka.persistence.typed.scaladsl.{Effect, EffectBuilder}
import com.nigeleke.cribbage.actors.Game.{PegScore, PointsScored}
import com.nigeleke.cribbage.actors.handlers.Handler.{Event, State}

object PegScoreHandler extends Handler {

  def apply(state: State, score: PegScore) : EffectBuilder[Event, State] =
    Effect.persist(PointsScored(score.playerId, score.points))

}
