package com.nigeleke.cribbage.effects

import com.nigeleke.cribbage.domain.*
import com.nigeleke.cribbage.util.*

import cats.data.Validated.*
import cats.data.ValidatedNel
import cats.syntax.validated.*

def playCard(player: Player.Id, card: Card)(game: PlayingGame): ValidatedNel[String, PlayingGame | WonGame] =
  def playCard(game: PlayingGame): ValidatedNel[String, PlayingGame] =
    val updatedNextToPlay = game.opponent(player)
    val (_, remaining) = game.hands(player).remove(Seq(card))
    val updatedHands = game.hands.updated(player, remaining)
    val plays = game.plays
    val updatedCurrent = plays.current :+ Plays.Laid(player, card)
    val updatedPlays = plays.copy(nextToPlay = updatedNextToPlay, current = updatedCurrent)
    game.copy(hands = updatedHands, plays = updatedPlays).validNel
  def scorePlay(game: PlayingGame): ValidatedNel[String, PlayingGame | WonGame] =
    val plays = game.plays
    val totalPoints = Scorer.forPlay(plays.current).total + Scorer.forEndPlay(plays).total
    val scorer = plays.current.last match
      case Plays.Laid(id, _) => id
      case Plays.Pass(id)    => id
    game.validNel andThen scorePoints(scorer, totalPoints)
  game.validNel andThen
    confirmExistingPlayer(player) andThen
    confirmPlayersTurnToPlay(player) andThen
    confirmPlayerHoldsCard(player, card) andThen
    confirmRunningTotalWillNotExceed31(card) andThen
    playCard andThen
    scorePlay

def pass(player: Player.Id)(game: PlayingGame): ValidatedNel[String, PlayingGame | WonGame] =
  def pass(game: PlayingGame): ValidatedNel[String, PlayingGame] =
    val updatedNextToPlay = game.opponent(player)
    val plays = game.plays
    val updatedCurrent = plays.current :+ Plays.Pass(player)
    val updatedPassedPlayers = plays.passedPlayers + player
    val updatedPlays =
      plays.copy(nextToPlay = updatedNextToPlay, current = updatedCurrent, passedPlayers = updatedPassedPlayers)
    game.copy(plays = updatedPlays).valid
  def scorePass(game: PlayingGame): ValidatedNel[String, PlayingGame | WonGame] =
    val plays = game.plays
    val points = Scorer.forEndPlay(plays)
    game.validNel andThen resetPlays andThen scorePoints(player, points.total)
  def resetPlays(game: PlayingGame): ValidatedNel[String, PlayingGame] =
    val plays = game.plays
    val updatedPlays =
      if plays.passCount == 2
      then plays.nextPlay
      else plays
    game.copy(plays = updatedPlays).validNel
  game.validNel andThen
    confirmExistingPlayer(player) andThen
    confirmPlayersTurnToPlay(player) andThen
    confirmNoValidCardsToPlay(player) andThen
    pass andThen
    scorePass

def scorePoints(player: Player.Id, points: Int)(game: PlayingGame): ValidatedNel[String, PlayingGame | WonGame] =
  val updatedScore = game.scores(player).add(points)
  val updatedScores = game.scores.updated(player, updatedScore)
  if updatedScore.points < 121
  then game.copy(scores = updatedScores).validNel
  else WonGame(game.id, updatedScores).validNel

def regatherPlays(game: PlayingGame): ValidatedNel[String, ScoringGame] =
  def regatherPlays: ScoringGame =
    val plays = game.plays
    val (dealerPlays, ponePlays) = plays.laidSoFar.partition(_.playerId == game.dealer)
    val dealerHand = dealerPlays.map(_.card)
    val poneHand = ponePlays.map(_.card)
    val updatedHands = Map(game.dealer -> dealerHand, game.pone -> poneHand)
    ScoringGame(game.id, game.scores, updatedHands, game.dealer, game.pone, game.crib, game.cut)
  val playersHandsPlayed = game.hands.values.forall(_.isEmpty)
  if playersHandsPlayed
  then regatherPlays.validNel
  else s"There are still cards left to play in ${game.id}".invalidNel
