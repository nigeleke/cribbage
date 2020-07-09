package com.nigeleke.cribbage.actors.handlers

import akka.persistence.typed.scaladsl.{Effect, EffectBuilder}
import com.nigeleke.cribbage.actors.Game.{HandsDealt, HandDealt}
import com.nigeleke.cribbage.actors.handlers.Handler.{Event, State}
import com.nigeleke.cribbage.model.Deck

object DealHandsHandler extends Handler {
  def apply(state: State): EffectBuilder[Event, State] = {
    val game = state.game
    val deck = Deck().shuffled
    val players = game.players
    val hands = (0 to players.size).map(n => deck.drop(n*6).take(6).map(_.id))
    val deals = players.zip(hands)
    val events = deals.map(deal => HandDealt(deal._1, deal._2)).toSeq

    Effect.persist(events :+ HandsDealt)
  }
}
