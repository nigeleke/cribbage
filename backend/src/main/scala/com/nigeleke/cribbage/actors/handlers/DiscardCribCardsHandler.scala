package com.nigeleke.cribbage.actors.handlers

import akka.persistence.typed.scaladsl.{Effect, EffectBuilder}
import com.nigeleke.cribbage.actors.Game.{CribCardsDiscarded, DiscardCribCards}
import com.nigeleke.cribbage.actors.handlers.Handler.{Event, State}

object DiscardCribCardsHandler extends Handler {

  def apply(state: State, discard: DiscardCribCards): EffectBuilder[Event, State] = {
    val game = state.game

    val playerInGame = game.players.contains(discard.playerId)
    val twoCardsDiscarded = discard.cards.size == 2
    val playerOwnsCards = (game.hands(discard.playerId) intersect  discard.cards) == discard.cards

    val discardPermitted = playerInGame && twoCardsDiscarded && playerOwnsCards

    if (discardPermitted) Effect.persist(CribCardsDiscarded(discard.playerId, discard.cards))
    else Effect.unhandled
  }

}
