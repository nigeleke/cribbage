package com.nigeleke.cribbage.model

//import com.nigeleke.cribbage.util.*

import cats.*
import cats.data.ValidatedNel
import cats.syntax.validated.*

import java.util.UUID
import scala.util.Random

type GameId = Game.Id

sealed trait Game:
  def id: GameId

trait HasHands:
  def hands: Map[PlayerId, Hand]

case class StartingGame(id: GameId, players: Seq[PlayerId]) extends Game

case class DiscardingGame(
    id: GameId,
    deck: Deck,
    scores: Map[PlayerId, Score],
    hands: Map[PlayerId, Hand],
    dealer: PlayerId,
    pone: PlayerId,
    crib: Crib
) extends Game
    with HasHands

case class PlayingGame(
    id: GameId,
    scores: Map[PlayerId, Score],
    hands: Map[PlayerId, Hand],
    dealer: PlayerId,
    pone: PlayerId,
    crib: Crib,
    cut: Card,
    plays: Plays
) extends Game
    with HasHands

case class ScoringGame(
    id: GameId,
    scores: Map[PlayerId, Score],
    hands: Map[PlayerId, Hand],
    dealer: PlayerId,
    pone: PlayerId,
    crib: Crib,
    cut: Card
) extends Game

case class WonGame(
    id: GameId,
    scores: Map[PlayerId, Score]
) extends Game

object Game:
  opaque type Id = UUID

  val maxPlayers = 2

  def apply(): StartingGame = StartingGame(id = UUID.randomUUID(), players = Seq.empty)

extension (game: Game)
  def players: Seq[PlayerId] = game match
    case StartingGame(_, players) => players
    case game: DiscardingGame     => Seq(game.dealer, game.pone)
    case game: PlayingGame        => Seq(game.dealer, game.pone)
    case game: ScoringGame        => ???
    case game: WonGame            => game.scores.keys.toSeq

extension (game: PlayingGame) def opponent(player: PlayerId): PlayerId = game.players.filterNot(_ == player).head
