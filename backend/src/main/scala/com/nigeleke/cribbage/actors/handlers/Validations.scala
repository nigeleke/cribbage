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

import com.nigeleke.cribbage.actors.validate.Validation
import com.nigeleke.cribbage.model._
import com.nigeleke.cribbage.model.Player.{ Id => PlayerId }

object Validations {

  def cardCanBeLaid(card: Card, play: Play): Validation = new Validation {
    def validate: Option[String] = {
      if (play.runningTotal + card.value <= 31) None
      else Some(s"Card $card cannot be laid in play $play (makes total > 31)")
    }
  }

  def discardingTwoCardsOnly(id: PlayerId, cards: Cards): Validation = new Validation {
    def validate = {
      if (cards.size == 2) None
      else Some(s"Player $id must discard two cards into the crib")
    }
  }

  def gameRequiresPlayers(attributes: Attributes): Validation = new Validation {
    def validate =
      if (attributes.players.size < 2) None
      else Option(s"Game has enough players")
  }

  def playerHoldsCards(id: PlayerId, cards: Cards, attributes: Attributes): Validation = new Validation {
    override def validate: Option[String] =
      if (cards.forall(attributes.hands(id).contains(_))) None
      else Option(s"Player $id does not hold all $cards")
  }

  def playerInGame(id: PlayerId, attributes: Attributes): Validation = new Validation {
    def validate =
      if (attributes.players.contains(id)) None
      else Option(s"Player $id is not a member of attributes")
  }

  def playerIsNextToLay(id: PlayerId, attributes: Attributes): Validation = new Validation {
    def validate =
      if (id == attributes.play.optNextToLay.get) None
      else Option(s"Player $id's opponent is the next player to lay a card")
  }

  def playerNotAlreadyJoinedGame(id: PlayerId, attributes: Attributes): Validation = new Validation {
    def validate =
      if (!attributes.players.contains(id)) None
      else Some(s"Player ${id} already joined attributes")
  }

  def playHasNoCardsToLay(id: PlayerId, attributes: Attributes): Validation = new Validation {
    def validate = {
      val runningTotal = attributes.play.runningTotal
      val playableCards = attributes.hands(id).filter(runningTotal + _.value <= 31)
      if (playableCards.isEmpty) None
      else Some(s"Player $id cannot pass; they hold cards that can be laid")
    }
  }

  def validDeal(attributes: Attributes): Validation = new Validation {
    def validate = {
      val allCardsDealt = (attributes.deck ++ attributes.hands.flatMap(_._2) ++ attributes.crib).size == 52
      if (allCardsDealt) None
      else Option(
        s"""Invalid deal:
           | Deck: (${attributes.deck.size}) ${attributes.deck}
           | Deal: (${attributes.hands.flatMap(_._2).size}) ${attributes.hands}
           | Crib: (${attributes.crib})  ${attributes.crib}
           |""".stripMargin)
    }
  }

}
