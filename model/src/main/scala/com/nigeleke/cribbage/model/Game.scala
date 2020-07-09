package com.nigeleke.cribbage.model

import java.util.UUID

import Card.{Id => CardId}
import Game.{Id => GameId}
import Player.{Id => PlayerId}

final case class Game(id: GameId,
                      players: Players,
                      optDealer: Option[PlayerId],
                      hands: Hands,
                      crib: Crib,
                      plays: Plays,
                      previousPlays: Seq[Plays],
                      scores: Scores)

object Game {
  type Id = UUID

  def apply(id: Id) : Game = Game(id, Set.empty, None, Map.empty, Seq.empty, Seq.empty, Seq.empty, Map.empty)

  implicit class GameOps(game: Game) {

    def withPlayer(id: PlayerId): Game = {
      require(game.players.size < 2)
      game.copy(players = game.players + id)
    }

    def withDealer(id: PlayerId): Game = {
      require(game.players.contains(id))
      game.copy(optDealer = Some(id))
    }

    def withHand(id: PlayerId, hand: Hand) = {
      require(game.players.contains(id))
      game.copy(hands = game.hands.updated(id, hand))
    }

    def withCribDiscard(id: PlayerId, cards: Cards): Game = {
      require(game.players.contains(id))
      require(cards.size == 2)
      require((game.hands(id).toSet -- cards).size == 4)
      val updatedHand = game.hands(id).filterNot(cards.contains(_))
      val updatedCrib = game.crib ++ cards
      game.copy(hands = game.hands.updated(id, updatedHand), crib = updatedCrib)
    }
  }

}
