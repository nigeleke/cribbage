package com.nigeleke.cribbage.actors.handlers

import akka.persistence.typed.scaladsl.{Effect, EffectBuilder}
import com.nigeleke.cribbage.actors.Game.{HandsDealt, HandDealt}
import com.nigeleke.cribbage.actors.handlers.Handler.{Event, State}
import com.nigeleke.cribbage.model.Deck._

object DealHandsHandler extends Handler {
  def apply(state: State): EffectBuilder[Event, State] = {
    val game = state.game
    val deck = game.deck.shuffled
    val players = game.players
    val hands = (0 to players.size).map(n => deck.ids.drop(n*6).take(6))
    val deals = players.zip(hands)
    val events = deals.map(deal => HandDealt(deal._1, deal._2)).toSeq

    Effect.persist(events :+ HandsDealt)
  }
}
