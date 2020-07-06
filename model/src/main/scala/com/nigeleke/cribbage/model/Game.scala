package com.nigeleke.cribbage.model

import java.util.UUID

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
    def withPlayer(id: PlayerId) = game.copy(players = game.players + id)
    def withDealer(id: PlayerId) = game.copy(optDealer = Some(id))
    def withHand(id: PlayerId, hand: Hand) = game.copy(hands = game.hands.updated(id, hand))
  }

}
