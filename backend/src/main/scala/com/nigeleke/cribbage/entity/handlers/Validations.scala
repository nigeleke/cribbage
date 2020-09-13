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

import com.nigeleke.cribbage.entity.validate.Validation
import com.nigeleke.cribbage.model._
import com.nigeleke.cribbage.model.Card.{ Id => CardId }
import com.nigeleke.cribbage.model.Player.{ Id => PlayerId }

object Validations {

  def cardCanBeLaid(cardId: CardId, game: Game): Validation = new Validation {
    def validate: Option[String] = {
      val card = game.card(cardId)
      if (game.runningTotal + card.value <= 31) None
      else Some(s"Card $card cannot be laid in play (will make total > 31)")
    }
  }

  def discardingTwoCardsOnly(playerId: PlayerId, cardIds: CardIds): Validation = new Validation {
    def validate: Option[String] = {
      if (cardIds.size == 2) None
      else Some(s"Player $playerId must discard two cards into the crib")
    }
  }

  def gameRequiresPlayers(game: Game): Validation = new Validation {
    def validate: Option[String] =
      if (game.players.size < 2) None
      else Option(s"GameEntity has enough players")
  }

  def playerHoldsCards(playerId: PlayerId, cardIds: CardIds, game: Game): Validation = new Validation {
    override def validate: Option[String] = {
      val playerCardIds = game.hands(playerId)
      if (cardIds.forall(playerCardIds.contains(_))) None
      else Option(s"Player $playerId does not hold all cards")
    }
  }

  def playerInGame(id: PlayerId, game: Game): Validation = new Validation {
    def validate: Option[String] =
      if (game.players.contains(id)) None
      else Option(s"Player $id is not a member of game")
  }

  def playerIsNextToLay(id: PlayerId, game: Game): Validation = new Validation {
    def validate: Option[String] =
      if (id == game.play.optNextToLay.get) None
      else Option(s"Player $id's opponent is the next player to lay a cardId")
  }

  def playerNotAlreadyJoinedGame(id: PlayerId, game: Game): Validation = new Validation {
    def validate: Option[String] =
      if (!game.players.contains(id)) None
      else Some(s"Player ${id} already joined game")
  }

  def playHasNoCardsToLay(id: PlayerId, game: Game): Validation = new Validation {
    def validate: Option[String] = {
      val runningTotal = game.runningTotal
      val playableCards = game.hands(id).filter(cardId => runningTotal + game.card(cardId).value <= 31)
      if (playableCards.isEmpty) None
      else Some(s"Player $id cannot pass; they hold cards that can be laid")
    }
  }

  def validDeal(game: Game): Validation = new Validation {
    def validate: Option[String] = {
      val allCardsDealt = (game.availableDeck ++ game.hands.flatMap(_._2) ++ game.crib).size == 52
      if (allCardsDealt) None
      else Option(
        s"""Invalid deal:
           | Deck: (${game.availableDeck.size}) ${game.availableDeck}
           | Deal: (${game.hands.flatMap(_._2).size}) ${game.hands}
           | Crib: (${game.crib.size})  ${game.crib}
           |""".stripMargin)
    }
  }

}
