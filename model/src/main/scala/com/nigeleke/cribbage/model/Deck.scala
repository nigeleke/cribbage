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

package com.nigeleke.cribbage.model

import java.util.UUID

import com.nigeleke.cribbage.model.Card.{ Id => CardId }
import com.nigeleke.cribbage.model.{ Face, Suit }

import scala.util.Random
import scala.language.implicitConversions

object Deck {

  def apply(): Deck = fullSetOfCards

  private def fullSetOfCards = (for {
    face <- Face.values()
    suit <- Suit.values()
  } yield Card(UUID.randomUUID(), face, suit)).toSeq

  implicit class DeckOps(deck: Deck) {
    val shuffled = Random.shuffle(deck)
  }

}
