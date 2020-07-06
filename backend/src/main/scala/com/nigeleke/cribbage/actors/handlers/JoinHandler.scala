package com.nigeleke.cribbage.actors.handlers

import akka.persistence.typed.scaladsl.{Effect, EffectBuilder}
import com.nigeleke.cribbage.actors.Game.{Join, PlayerJoined, Starting}
import com.nigeleke.cribbage.actors.handlers.Handler.{Event, State}

object JoinHandler extends Handler {

  def apply(state: State, join: Join) : EffectBuilder[Event, State] = {
    require(state.isInstanceOf[Starting])

    val game = state.game
    val players = game.players
    val permitted = players.size < 2 && !players.contains(join.playerId)

    if (permitted) Effect.persist(PlayerJoined(join.playerId))
    else Effect.unhandled
  }
}
