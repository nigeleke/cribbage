package com.nigeleke.cribbage.model

import java.util.UUID

import Card.{Id => CardId}
import Game.{Id => GameId}
import Player.{Id => PlayerId}

final case class Game(id: GameId,
                      deck: Deck,
                      players: Players,
                      optDealer: Option[PlayerId],
                      hands: Hands,
                      crib: Crib,
                      optCut: Option[Card],
                      plays: Plays,
                      previousPlays: Seq[Plays],
                      scores: Scores)

object Game {
  type Id = UUID

  def apply(id: Id) : Game =
    Game(id,
      deck = Seq.empty,
      players = Set.empty,
      optDealer = None,
      hands = Map.empty,
      crib = Seq.empty,
      optCut = None,
      plays = Seq.empty,
      previousPlays = Seq.empty,
      scores = Map.empty)

  implicit class GameOps(game: Game) {

    def withDeck(deck: Deck): Game = game.copy(deck = deck)

    def withPlayer(id: PlayerId): Game = {
      require(game.players.size < 2)
      game.copy(players = game.players + id)
    }

    def withDealer(id: PlayerId): Game = {
      require(game.players.contains(id))
      game.copy(optDealer = Some(id))
    }

    def withHand(id: PlayerId, hand: Hand) = {
      import Deck._
      require(game.players.contains(id))
      require(hand.forall(id => game.deck.ids.contains(id)))
      val updatedHands = game.hands.updated(id, hand)
      val updatedDeck = game.deck.filterNot(card => hand.contains(card.id))
       game.copy(deck = updatedDeck, hands = updatedHands)
    }

    def withCribDiscard(id: PlayerId, cards: Cards): Game = {
      require(game.players.contains(id))
      require(cards.size == 2)
      require((game.hands(id).toSet -- cards).size == 4)
      val updatedHand = game.hands(id).filterNot(cards.contains(_))
      val updatedCrib = game.crib ++ cards
      game.copy(hands = game.hands.updated(id, updatedHand), crib = updatedCrib)
    }

    def withCut(cut: Card): Game = {
      game.copy(optCut = Some(cut))
    }

    def withScore(id: PlayerId, points: Int): Game = {
      val currentScore = game.scores.getOrElse(id, Score(0,0))
      val updatedScore = Score(currentScore.front, currentScore.front + points)
      game.copy(scores = game.scores.updated(id, updatedScore))
    }
  }

}
