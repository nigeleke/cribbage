package com.nigeleke.cribbage.actors.handlers

import akka.persistence.typed.scaladsl.{Effect, EffectBuilder}
import com.nigeleke.cribbage.actors.Game.{Join, PlayerJoined}
import com.nigeleke.cribbage.actors.handlers.Handler.{Event, State}

object JoinHandler extends Handler {

  def apply(state: State, join: Join) : EffectBuilder[Event, State] = {
    val game = state.game
    val players = game.players

    val stillNeedPlayers = players.size < 2
    val notAlreadyJoined = !players.contains(join.playerId)
    val permitted = stillNeedPlayers && notAlreadyJoined

    if (permitted) Effect.persist(PlayerJoined(join.playerId))
    else Effect.unhandled
  }
}
