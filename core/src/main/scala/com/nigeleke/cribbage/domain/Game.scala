package com.nigeleke.cribbage.domain

import cats.*
import cats.data.ValidatedNel
import cats.syntax.validated.*

import java.util.UUID
import scala.util.Random

sealed trait Game:
  def id: Game.Id

trait HasHands:
  def hands: Map[Player.Id, Hand]

case class StartingGame(id: Game.Id = Game.newGameId, players: Seq[Player.Id] = Seq.empty) extends Game

case class DiscardingGame(
    id: Game.Id,
    deck: Deck,
    scores: Map[Player.Id, Score],
    hands: Map[Player.Id, Hand],
    dealer: Player.Id,
    pone: Player.Id,
    crib: Crib
) extends Game
    with HasHands

case class PlayingGame(
    id: Game.Id,
    scores: Map[Player.Id, Score],
    hands: Map[Player.Id, Hand],
    dealer: Player.Id,
    pone: Player.Id,
    crib: Crib,
    cut: Card,
    plays: Plays
) extends Game
    with HasHands

case class ScoringGame(
    id: Game.Id,
    scores: Map[Player.Id, Score],
    hands: Map[Player.Id, Hand],
    dealer: Player.Id,
    pone: Player.Id,
    crib: Crib,
    cut: Card
) extends Game

case class WonGame(
    id: Game.Id,
    scores: Map[Player.Id, Score]
) extends Game

object Game:
  opaque type Id = UUID

  val maxPlayers = 2

  def apply(): Game = StartingGame()
  def newGameId: Game.Id = UUID.randomUUID()

extension (game: Game)
  def players: Seq[Player.Id] = game match
    case StartingGame(_, players) => players
    case game: DiscardingGame     => Seq(game.dealer, game.pone)
    case game: PlayingGame        => Seq(game.dealer, game.pone)
    case game: ScoringGame        => game.scores.keys.toSeq
    case game: WonGame            => game.scores.keys.toSeq

extension (game: PlayingGame) def opponent(player: Player.Id): Player.Id = game.players.filterNot(_ == player).head
