package com.nigeleke.cribbage.actors.handlers

import akka.persistence.typed.scaladsl.{Effect, EffectBuilder}
import com.nigeleke.cribbage.actors.Game.{DealerCutRevealed, DealerSelected}
import com.nigeleke.cribbage.actors.handlers.Handler.{Event, State}
import com.nigeleke.cribbage.model.{Card, Deck}
import com.nigeleke.cribbage.model.Player.{Id => PlayerId}

object CutForDealHandler extends Handler {

  def apply(state: State): EffectBuilder[Event, State] = {
    println(s"CutForDealHandler1")
    val game = state.game
    println(s"CutForDealHandler1a")
    val players = game.players
    println(s"CutForDealHandler1b")

    def cutDeck() = {
      println(s"CutForDealHandler1CD $players")
      val deck = Deck().shuffled.cards
      println(s"CutForDealHandler2CD $deck")
      val cuts = players.zip(deck).toMap
      println(s"CutForDealHandler3CD $cuts")
      cuts
    }
    println(s"CutForDealHandler1c")
    def sameRank(cuts: Map[PlayerId, Card]) = {
      println(s"CutForDealHandler1SR")
      cuts.values.groupBy(_.rank).size == 1
    }
    println(s"CutForDealHandler1d")

    val drawnCuts = Iterator.continually(cutDeck()).takeWhile(sameRank).toSeq
    println(s"CutForDealHandler1e")
    val finalCuts = Iterator.continually(cutDeck()).dropWhile(sameRank).take(1).toSeq
    println(s"CutForDealHandler1f")

    val reveals : Seq[Event] = for {
      cuts <- (drawnCuts ++ finalCuts)
      cut <- cuts
    } yield DealerCutRevealed(cut._1, cut._2)

    println(s"CutForDealHandler2")

    def selectDealer(cuts: Map[PlayerId, Card]) = cuts.minBy(_._2.rank)._1
    val dealerSelected = finalCuts.map(selectDealer).map(DealerSelected)

    println(s"CutForDealHandler3")

    Effect.persist(reveals ++ dealerSelected)
  }

}
