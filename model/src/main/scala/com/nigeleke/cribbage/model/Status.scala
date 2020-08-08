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

import Status.{ Id => GameId }
import Player.{ Id => PlayerId }

final case class Status(
  id: GameId,
  deck: Deck,
  players: Players,
  optDealer: Option[PlayerId],
  hands: Hands,
  crib: Crib,
  optCut: Option[Card],
  play: Play,
  scores: Scores)

object Status {
  type Id = UUID

  def apply(id: Id): Status =
    Status(
      id,
      deck = Seq.empty,
      players = Set.empty,
      optDealer = None,
      hands = Map.empty,
      crib = Seq.empty,
      optCut = None,
      play = Play(),
      scores = Map.empty)

  implicit class GameOps(game: Status) {

    lazy val optPone: Option[PlayerId] = (for {
      dealer <- game.optDealer
      otherPlayers = game.players - dealer
    } yield otherPlayers.headOption).flatten

    def withPlayer(id: PlayerId): Status = {
      require(game.players.size < 2, s"Player $id cannot join $game")
      game.copy(players = game.players + id)
    }

    def withDealer(id: PlayerId): Status = {
      require(game.players.contains(id), s"Dealer $id is not player in $game")
      game.copy(optDealer = Some(id))
    }

    def withZeroScores(): Status = {
      val updatedScores = game.players.map(_ -> Score(0, 0)).toMap
      game.copy(scores = updatedScores)
    }

    def withDeal(hands: Hands, deck: Deck) = {
      require((game.players -- hands.keySet).size == 0, s"Players ${hands.keySet} should be part of $game")
      require((deck.toSet intersect hands.values.flatten.toSet) == hands.values.flatten.toSet, s"Dealt hands ${hands.values} should be from deck $deck")
      game.copy(deck = (deck.toSet -- hands.values.flatten).toSeq, hands = hands)
    }

    def withCribDiscard(id: PlayerId, cards: Cards): Status = {
      require(game.players.contains(id), s"Player $id is not in $game")
      require(cards.size == 2, s"Player $id must discard two cards")
      require((game.hands(id).toSet -- cards).size == 4, s"Player $id does not own cards being discarded")
      val updatedHand = game.hands(id).filterNot(cards.contains(_))
      val updatedCrib = game.crib ++ cards
      game.copy(hands = game.hands.updated(id, updatedHand), crib = updatedCrib)
    }

    def withCut(cut: Card): Status = {
      require(game.deck.contains(cut), s"Cut card $cut not in deck")
      game.copy(optCut = Some(cut))
    }

    def withNextToLay(id: PlayerId) = {
      require(game.players.contains(id), s"Player $id is not in $game")
      val updatedPlay = game.play.withNextToLay(id)
      game.copy(play = updatedPlay)
    }

    def withLay(id: PlayerId, card: Card) = {
      require(game.players.contains(id), s"Player $id is not in $game")
      require(game.hands(id).contains(card), s"Player $id does not hold $card in ${game.hands(id)}")
      require(game.play.runningTotal + card.value <= 31, s"Player $id cannot lay $card in current play ${game.play.current}")
      val updatedHand = game.hands(id).filterNot(_ == card)
      val updatedPlay = game.play.withLay(Lay(id, card)).withNextToLay(opponent(id))
      game.copy(hands = game.hands.updated(id, updatedHand), play = updatedPlay)
    }

    def withPass(id: PlayerId) = {
      require(game.players.contains(id), s"Player $id is not in $game")
      require(game.hands(id).forall(card => game.play.runningTotal + card.value > 31), s"Player $id cannot pass")
      val updatedPlay = game.play.withPass().withNextToLay(opponent(id))
      game.copy(play = updatedPlay)
    }

    def withNextPlay() = {
      require(
        game.players.forall(game.hands(_).forall(card => game.play.runningTotal + card.value > 31)),
        s"""Cannot progress to next play with cards still available to lay:
           | ${game.play.current}
           | ${game.hands}""".stripMargin)
      val updatedPlay = game.play.withNextPlay()
      game.copy(play = updatedPlay)
    }

    def withPlaysReturned() = {
      require(game.hands.forall(_._2.isEmpty), s"All cards should have been played")
      val laysByPlayerId = (for {
        lays <- game.play.previous :+ game.play.current
        lay <- lays
      } yield lay).groupBy(_.playerId)
      val updatedHands = laysByPlayerId.view.mapValues(_.map(_.card)).toMap
      game.copy(hands = updatedHands, play = Play())
    }

    def opponent(playerId: PlayerId): PlayerId = {
      game.players.filterNot(_ == playerId).head
    }

    def withScore(id: PlayerId, points: Int): Status = {
      val currentScore = game.scores.getOrElse(id, Score(0, 0))
      val updatedScore = Score(currentScore.front, currentScore.front + points)
      game.copy(scores = game.scores.updated(id, updatedScore))
    }

    def withSwappedDealer(): Status = game.copy(optDealer = game.optPone, deck = Deck(), hands = Map.empty)
  }

}
