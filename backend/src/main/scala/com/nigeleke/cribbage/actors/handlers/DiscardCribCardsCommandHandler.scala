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

package com.nigeleke.cribbage.actors.handlers

import akka.persistence.typed.scaladsl.Effect
import com.nigeleke.cribbage.actors.Game._
import com.nigeleke.cribbage.actors.handlers.Validations._
import com.nigeleke.cribbage.actors.validate.Validation._

case class DiscardCribCardsCommandHandler(discard: DiscardCribCards, state: Discarding) extends CommandHandler {

  import CommandHandler._

  val playerId = discard.playerId
  val cards = discard.cards
  val game = state.game

  override def canDo: Option[String] =
    validate(playerInGame(playerId, game) and
      validDeal(game) and
      playerHoldsCards(playerId, cards, game) and
      discardingTwoCardsOnly(playerId, cards))

  lazy val events = CribCardsDiscarded(playerId, cards) +:
    (if (state.game.crib.size == 2) scoreCutAtStartOfPlay(game) else Seq.empty)

  override def effects: Effect[Event, State] = Effect.persist(events)

}
