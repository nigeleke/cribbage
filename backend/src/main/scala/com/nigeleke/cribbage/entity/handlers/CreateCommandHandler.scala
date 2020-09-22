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

import akka.pattern.StatusReply
import akka.pattern.StatusReply.Success
import akka.persistence.typed.scaladsl.{ Effect, EffectBuilder }
import com.nigeleke.cribbage.entity.GameEntity
import com.nigeleke.cribbage.entity.GameEntity._

case class CreateCommandHandler(id: GameEntity.Id) extends CommandHandler {

  override def canDo: Boolean = true

  override def rejectionReasons: String = ???

  override def acceptedEffect: EffectBuilder[Event, State] = Effect.persist(GameCreated(id))

  override def acceptedReply: StatusReply[_] = Success(GameCreated(id))

}
