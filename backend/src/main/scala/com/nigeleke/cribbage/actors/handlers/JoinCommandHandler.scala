package com.nigeleke.cribbage.actors.handlers

import akka.persistence.typed.scaladsl.Effect
import com.nigeleke.cribbage.actors.Game._
import com.nigeleke.cribbage.actors.handlers.Validations._
import com.nigeleke.cribbage.actors.validate.Validation._
import com.nigeleke.cribbage.model.{Card, Deck}
import com.nigeleke.cribbage.model.Deck._
import com.nigeleke.cribbage.model.Player.{Id => PlayerId}

case class JoinCommandHandler(join: Join, state: Starting) extends CommandHandler {

  lazy val playerId = join.playerId
  lazy val game = state.game

  override def canDo: Option[String] =
    validate(
      gameRequiresPlayers(game) and
      playerNotAlreadyJoinedGame(playerId, game))

  override def effects: Effect[Event, State] = {
    val players = state.game.players + join.playerId

    lazy val cutForDealEvents = {
      def cutDeck() = players.zip(Deck().shuffled).toMap
      def sameRank(cuts: Map[PlayerId, Card]) = cuts.values.groupBy(_.rank).size == 1

      val drawnCuts = Iterator.continually(cutDeck()).takeWhile(sameRank).toSeq
      val finalCuts = Iterator.continually(cutDeck()).dropWhile(sameRank).take(1).toSeq

      val reveals: Seq[Event] = for {
        cuts <- (drawnCuts ++ finalCuts)
        cut <- cuts
      } yield DealerCutRevealed(cut._1, cut._2)

      def selectDealer(cuts: Map[PlayerId, Card]) = cuts.minBy(_._2.rank)._1
      val dealerSelected = finalCuts.map(selectDealer).map(DealerSelected)

      reveals ++ dealerSelected
    }

    lazy val dealHandsEvent = {
      val deck = Deck().shuffled
      val hands = (0 to players.size).map(n => deck.drop(n*6).take(6))
      val deals = players.zip(hands)
      HandsDealt(deals.toMap, deck)
    }

    val cutAndDealEvents =
      if (players.size == 2) (cutForDealEvents :+ dealHandsEvent)
      else Seq.empty

    Effect.persist((PlayerJoined(join.playerId) +: cutAndDealEvents))
  }
}
