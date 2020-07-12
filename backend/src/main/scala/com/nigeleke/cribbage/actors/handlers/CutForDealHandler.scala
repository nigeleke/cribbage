package com.nigeleke.cribbage.actors.handlers

import akka.persistence.typed.scaladsl.{Effect, EffectBuilder}
import com.nigeleke.cribbage.actors.Game.{DealerCutRevealed, DealerSelected}
import com.nigeleke.cribbage.actors.handlers.Handler.{Event, State}
import com.nigeleke.cribbage.model.{Card, Deck}
import com.nigeleke.cribbage.model.Player.{Id => PlayerId}

object CutForDealHandler extends Handler {

  def apply(state: State): EffectBuilder[Event, State] = {
    val game = state.game
    val players = game.players

    def cutDeck() = {
      import Deck._
      players.zip(game.deck.shuffled).toMap
    }

    def sameRank(cuts: Map[PlayerId, Card]) = cuts.values.groupBy(_.rank).size == 1

    val drawnCuts = Iterator.continually(cutDeck()).takeWhile(sameRank).toSeq
    val finalCuts = Iterator.continually(cutDeck()).dropWhile(sameRank).take(1).toSeq

    val reveals: Seq[Event] = for {
      cuts <- (drawnCuts ++ finalCuts)
      cut <- cuts
    } yield DealerCutRevealed(cut._1, cut._2)

    def selectDealer(cuts: Map[PlayerId, Card]) = cuts.minBy(_._2.rank)._1

    val dealerSelected = finalCuts.map(selectDealer).map(DealerSelected)

    Effect.persist(reveals ++ dealerSelected)
  }

}
