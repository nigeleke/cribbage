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

import Card.{ Id => CardId }
import Player.{ Id => PlayerId }

final case class Game(
  deck: Deck,
  players: Players,
  optDealer: Option[PlayerId],
  hands: Hands,
  crib: Crib,
  optCut: Option[CardId],
  play: Play,
  scores: Scores)

object Game {
  def apply(): Game =
    Game(
      deck = Seq.empty,
      players = Set.empty,
      optDealer = None,
      hands = Map.empty,
      crib = Seq.empty,
      optCut = None,
      play = Play(),
      scores = Map.empty)

  implicit class GameOps(game: Game) {

    lazy val optPone: Option[PlayerId] = (for {
      dealer <- game.optDealer
      otherPlayers = game.players - dealer
    } yield otherPlayers.headOption).flatten

    def withPlayer(id: PlayerId): Game = {
      require(game.players.size < 2, s"Player $id cannot join $game")
      game.copy(players = game.players + id)
    }

    def withDealer(id: PlayerId): Game = {
      require(game.players.contains(id), s"Dealer $id is not player in $game")
      game.copy(optDealer = Some(id))
    }

    def withZeroScores(): Game = {
      val updatedScores = game.players.map(_ -> Score(0, 0)).toMap
      game.copy(scores = updatedScores)
    }

    lazy val availableDeck: Seq[CardId] = {
      val usedCardIds = game.optCut.toList ++ game.hands.values.flatten ++ game.crib
      game.deck.map(_.id).filterNot(usedCardIds.contains(_))
    }

    def withDeal(hands: Hands, deck: Deck) = {
      require((game.players -- hands.keySet).size == 0, s"Players ${hands.keySet} should be part of $game")
      require((deck.map(_.id).toSet intersect hands.values.flatten.toSet) == hands.values.flatten.toSet, s"Dealt hands ${hands.values} should be from deck $deck")
      game.copy(deck = deck, hands = hands)
    }

    def withCribDiscard(id: PlayerId, cardIds: CardIds): Game = {
      require(game.players.contains(id), s"Player $id is not in $game")
      require(cardIds.size == 2, s"Player $id must discard two cards")
      require((game.hands(id).toSet -- cardIds).size == 4, s"Player $id does not own cards being discarded")
      val updatedHand = game.hands(id).filterNot(cardIds.contains(_))
      val updatedCrib = game.crib ++ cardIds
      game.copy(hands = game.hands.updated(id, updatedHand), crib = updatedCrib)
    }

    def withCut(cardId: CardId): Game = {
      require(game.deck.map(_.id).contains(cardId), s"Cut $cardId not in deck")
      game.copy(optCut = Some(cardId))
    }

    def withNextToLay(playerId: PlayerId): Game = {
      require(game.players.contains(playerId), s"Player $playerId is not in $game")
      val updatedPlay = game.play.withNextToLay(playerId)
      game.copy(play = updatedPlay)
    }

    def withLay(playerId: PlayerId, cardId: CardId): Game = {
      require(game.players.contains(playerId), s"Player $playerId is not in $game")
      require(game.hands(playerId).contains(cardId), s"Player $playerId does not hold $cardId in ${game.hands(playerId)}")
      require(runningTotal + card(cardId).value <= 31, s"Player $playerId cannot lay $cardId in current play ${game.play.current}")
      val updatedHand = game.hands(playerId).filterNot(_ == cardId)
      val updatedPlay = game.play.withLay(Lay(playerId, cardId)).withNextToLay(opponent(playerId))
      game.copy(hands = game.hands.updated(playerId, updatedHand), play = updatedPlay)
    }

    lazy val runningTotal: Int = game.play.current.map(_.cardId).map(card).map(_.value).sum

    def card(cardId: CardId): Card = game.deck.find(_.id == cardId).head

    def withPass(playerId: PlayerId): Game = {
      require(game.players.contains(playerId), s"Player $playerId is not in $game")
      require(game.hands(playerId).forall(cardId => runningTotal + card(cardId).value > 31), s"Player $playerId cannot pass")
      val updatedPlay = game.play.withPass().withNextToLay(opponent(playerId))
      game.copy(play = updatedPlay)
    }

    def withNextPlay(): Game = {
      require(
        game.players.forall(game.hands(_).forall(cardId => runningTotal + card(cardId).value > 31)),
        s"""Cannot progress to next play with cards still available to lay:
           | ${game.play.current}
           | ${game.hands}""".stripMargin)
      val updatedPlay = game.play.withNextPlay()
      game.copy(play = updatedPlay)
    }

    def withPlaysReturned(): Game = {
      require(game.hands.forall(_._2.isEmpty), s"All cards should have been played")
      val laysByPlayerId = (for {
        lays <- game.play.previous :+ game.play.current
        lay <- lays
      } yield lay).groupBy(_.playerId)
      val updatedHands = laysByPlayerId.view.mapValues(_.map(_.cardId)).toMap
      game.copy(hands = updatedHands, play = Play())
    }

    def opponent(playerId: PlayerId): PlayerId = {
      game.players.filterNot(_ == playerId).head
    }

    def withScore(playerId: PlayerId, points: Int): Game = {
      val currentScore = game.scores.getOrElse(playerId, Score(0, 0))
      val updatedScore = Score(currentScore.front, currentScore.front + points)
      game.copy(scores = game.scores.updated(playerId, updatedScore))
    }

    def withSwappedDealer(): Game = game.copy(optDealer = game.optPone, deck = Deck(), hands = Map.empty)
  }

}
