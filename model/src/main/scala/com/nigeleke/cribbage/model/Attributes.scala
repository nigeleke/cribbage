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

import Player.{ Id => PlayerId }

final case class Attributes(
  deck: Deck,
  players: Players,
  optDealer: Option[PlayerId],
  hands: Hands,
  crib: Crib,
  optCut: Option[Card],
  play: Play,
  scores: Scores)

object Attributes {
  def apply(): Attributes =
    Attributes(
      deck = Seq.empty,
      players = Set.empty,
      optDealer = None,
      hands = Map.empty,
      crib = Seq.empty,
      optCut = None,
      play = Play(),
      scores = Map.empty)

  implicit class AttributeOps(attributes: Attributes) {

    lazy val optPone: Option[PlayerId] = (for {
      dealer <- attributes.optDealer
      otherPlayers = attributes.players - dealer
    } yield otherPlayers.headOption).flatten

    def withPlayer(id: PlayerId): Attributes = {
      require(attributes.players.size < 2, s"Player $id cannot join $attributes")
      attributes.copy(players = attributes.players + id)
    }

    def withDealer(id: PlayerId): Attributes = {
      require(attributes.players.contains(id), s"Dealer $id is not player in $attributes")
      attributes.copy(optDealer = Some(id))
    }

    def withZeroScores(): Attributes = {
      val updatedScores = attributes.players.map(_ -> Score(0, 0)).toMap
      attributes.copy(scores = updatedScores)
    }

    def withDeal(hands: Hands, deck: Deck) = {
      require((attributes.players -- hands.keySet).size == 0, s"Players ${hands.keySet} should be part of $attributes")
      require((deck.toSet intersect hands.values.flatten.toSet) == hands.values.flatten.toSet, s"Dealt hands ${hands.values} should be from deck $deck")
      attributes.copy(deck = (deck.toSet -- hands.values.flatten).toSeq, hands = hands)
    }

    def withCribDiscard(id: PlayerId, cards: Cards): Attributes = {
      require(attributes.players.contains(id), s"Player $id is not in $attributes")
      require(cards.size == 2, s"Player $id must discard two cards")
      require((attributes.hands(id).toSet -- cards).size == 4, s"Player $id does not own cards being discarded")
      val updatedHand = attributes.hands(id).filterNot(cards.contains(_))
      val updatedCrib = attributes.crib ++ cards
      attributes.copy(hands = attributes.hands.updated(id, updatedHand), crib = updatedCrib)
    }

    def withCut(cut: Card): Attributes = {
      require(attributes.deck.contains(cut), s"Cut card $cut not in deck")
      attributes.copy(optCut = Some(cut))
    }

    def withNextToLay(id: PlayerId) = {
      require(attributes.players.contains(id), s"Player $id is not in $attributes")
      val updatedPlay = attributes.play.withNextToLay(id)
      attributes.copy(play = updatedPlay)
    }

    def withLay(id: PlayerId, card: Card) = {
      require(attributes.players.contains(id), s"Player $id is not in $attributes")
      require(attributes.hands(id).contains(card), s"Player $id does not hold $card in ${attributes.hands(id)}")
      require(attributes.play.runningTotal + card.value <= 31, s"Player $id cannot lay $card in current play ${attributes.play.current}")
      val updatedHand = attributes.hands(id).filterNot(_ == card)
      val updatedPlay = attributes.play.withLay(Lay(id, card)).withNextToLay(opponent(id))
      attributes.copy(hands = attributes.hands.updated(id, updatedHand), play = updatedPlay)
    }

    def withPass(id: PlayerId) = {
      require(attributes.players.contains(id), s"Player $id is not in $attributes")
      require(attributes.hands(id).forall(card => attributes.play.runningTotal + card.value > 31), s"Player $id cannot pass")
      val updatedPlay = attributes.play.withPass().withNextToLay(opponent(id))
      attributes.copy(play = updatedPlay)
    }

    def withNextPlay() = {
      require(
        attributes.players.forall(attributes.hands(_).forall(card => attributes.play.runningTotal + card.value > 31)),
        s"""Cannot progress to next play with cards still available to lay:
           | ${attributes.play.current}
           | ${attributes.hands}""".stripMargin)
      val updatedPlay = attributes.play.withNextPlay()
      attributes.copy(play = updatedPlay)
    }

    def withPlaysReturned() = {
      require(attributes.hands.forall(_._2.isEmpty), s"All cards should have been played")
      val laysByPlayerId = (for {
        lays <- attributes.play.previous :+ attributes.play.current
        lay <- lays
      } yield lay).groupBy(_.playerId)
      val updatedHands = laysByPlayerId.view.mapValues(_.map(_.card)).toMap
      attributes.copy(hands = updatedHands, play = Play())
    }

    def opponent(playerId: PlayerId): PlayerId = {
      attributes.players.filterNot(_ == playerId).head
    }

    def withScore(id: PlayerId, points: Int): Attributes = {
      val currentScore = attributes.scores.getOrElse(id, Score(0, 0))
      val updatedScore = Score(currentScore.front, currentScore.front + points)
      attributes.copy(scores = attributes.scores.updated(id, updatedScore))
    }

    def withSwappedDealer(): Attributes = attributes.copy(optDealer = attributes.optPone, deck = Deck(), hands = Map.empty)
  }

}
