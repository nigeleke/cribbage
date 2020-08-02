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

case class LayCardCommandHandler(lay: LayCard, state: Playing) extends CommandHandler {

  import CommandHandler._

  val game = state.game
  val play = game.play

  val playerId = lay.playerId
  val card = lay.card

  override def canDo: Option[String] =
    validate(playerInGame(playerId, game) and
      playerIsNextToLay(playerId, game) and
      cardCanBeLaid(lay.card, play))

  lazy val events = CardLaid(playerId, card) +: {
    val gameWithLay = game.withLay(playerId, card)
    scoreLay(gameWithLay) ++ endPlay(gameWithLay) ++ endPlays(gameWithLay)
  }

  override def effects: Effect[Event, State] = Effect.persist(events)

}
