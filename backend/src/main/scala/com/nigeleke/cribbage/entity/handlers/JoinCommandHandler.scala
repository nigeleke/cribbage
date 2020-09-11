/*
 * Copyright (C) 2020  Nigel Eke
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

package com.nigeleke.cribbage.entity.handlers

import akka.persistence.typed.scaladsl.{ Effect, EffectBuilder }
import com.nigeleke.cribbage.entity.GameEntity._
import com.nigeleke.cribbage.entity.handlers.Validations._
import com.nigeleke.cribbage.entity.validate.Validation._
import com.nigeleke.cribbage.model.{ Card, Deck }
import com.nigeleke.cribbage.model.Deck._
import com.nigeleke.cribbage.model.Player.{ Id => PlayerId }

case class JoinCommandHandler(join: Join, state: Starting) extends CommandHandler {

  val playerId = join.playerId
  val game = state.game

  val optRejectionReasons =
    validate(gameRequiresPlayers(game) and
      playerNotAlreadyJoinedGame(playerId, game))

  override def canDo: Boolean = optRejectionReasons.isEmpty

  override def rejectionReasons: String = optRejectionReasons.getOrElse("")

  override def acceptedEffect: EffectBuilder[Event, State] = {
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
      val hands = (0 to players.size).map(n => deck.drop(n * 6).take(6))
      val deals = players.zip(hands)
      HandsDealt(deals.toMap, deck)
    }

    val cutAndDealEvents =
      if (players.size == 2) (cutForDealEvents :+ dealHandsEvent)
      else Seq.empty

    Effect.persist((PlayerJoined(join.playerId) +: cutAndDealEvents))
  }
}
