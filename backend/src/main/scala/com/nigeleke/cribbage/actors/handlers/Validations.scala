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
import com.nigeleke.cribbage.model.Player.{Id => PlayerId}

object Validations {

  def cardCanBeLaid(card: Card, play: Play) : Validation = new Validation {
    def validate: Option[String] = {
      if (play.runningTotal + card.value <= 31) None
      else Some(s"Card $card cannot be laid in play $play (makes total > 31)")
    }
  }

  def discardingTwoCardsOnly(id: PlayerId, cards: Cards) : Validation = new Validation {
    def validate = {
      if (cards.size == 2) None
      else Some(s"Player $id must discard two cards into the crib")
    }
  }

  def gameRequiresPlayers(game: Status) : Validation = new Validation {
    def validate =
      if (game.players.size < 2) None
      else Option(s"Game ${game.id} has enough players")
  }

  def playerHoldsCards(id: PlayerId, cards: Cards, game: Status) : Validation = new Validation {
    override def validate: Option[String] =
      if (cards.forall(game.hands(id).contains(_))) None
      else Option(s"Player $id does not hold all $cards")
  }

  def playerInGame(id: PlayerId, game: Status) : Validation = new Validation {
    def validate =
      if (game.players.contains(id)) None
      else Option(s"Player $id is not a member of game ${game.id}")
  }

  def playerIsNextToLay(id: PlayerId, game: Status) : Validation = new Validation {
    def validate =
      if (id == game.play.optNextToLay.get) None
      else Option(s"Player $id's opponent is the next player to lay a card")
  }

  def playerNotAlreadyJoinedGame(id: PlayerId, game: Status) : Validation = new Validation {
    def validate =
      if (!game.players.contains(id)) None
      else Some(s"Player ${id} already joined game ${game.id}")
  }

  def playHasNoCardsToLay(id: PlayerId, game: Status) : Validation = new Validation {
    def validate = {
      val runningTotal = game.play.runningTotal
      val playableCards = game.hands(id).filter(runningTotal + _.value <= 31)
      if (playableCards.isEmpty) None
      else Some(s"Player $id cannot pass; they hold cards that can be laid")
    }
  }

  def validDeal(game: Status) : Validation = new Validation {
    def validate = {
      val allCardsDealt = (game.deck ++ game.hands.flatMap(_._2) ++ game.crib).size == 52
      if (allCardsDealt) None
      else Option(
        s"""Invalid deal:
           | Deck: (${game.deck.size}) ${game.deck}
           | Deal: (${game.hands.flatMap(_._2).size}) ${game.hands}
           | Crib: (${game.crib})  ${game.crib}
           |""".stripMargin)
    }
  }

}
