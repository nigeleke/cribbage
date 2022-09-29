package com.nigeleke.cribbage.util

import com.nigeleke.cribbage.domain.*

import cats.data.ValidatedNel
import cats.syntax.validated.*

type ActiveGame = DiscardingGame | PlayingGame

def confirmNeedsAnotherPlayer(game: StartingGame): ValidatedNel[String, StartingGame] =
  if game.players.size < Game.maxPlayers
  then game.validNel
  else s"Game ${game.id} does not need any further players".invalidNel

def confirmNotExistingPlayer(id: Player.Id)(game: StartingGame): ValidatedNel[String, StartingGame] =
  if !game.players.contains(id)
  then game.validNel
  else s"Player $id has already joined Game ${game.id}".invalidNel

def confirmCorrectNumberOfPlayers(game: StartingGame): ValidatedNel[String, StartingGame] =
  if game.players.size == Game.maxPlayers
  then game.validNel
  else s"Game ${game.id} requires more Players".invalidNel

def confirmExistingPlayer[A <: ActiveGame](player: Player.Id)(game: A): ValidatedNel[String, A] =
  if game.players.contains(player)
  then game.validNel
  else s"Player $player has not joined Game ${game.id}".invalidNel

def confirmDiscardingTwoCards(discards: Seq[Card])(game: DiscardingGame): ValidatedNel[String, DiscardingGame] =
  val expectedDiscardCount = 2
  val actualDiscardCount = discards.size
  if actualDiscardCount == expectedDiscardCount
  then game.validNel
  else s"Game ${game.id} expecting $expectedDiscardCount cards discarded; have $actualDiscardCount".invalidNel

def confirmPlayerHoldsCards[A <: ActiveGame](playerId: Player.Id, cards: Seq[Card])(game: A): ValidatedNel[String, A] =
  val (removed, _) = game.hands(playerId).remove(cards)
  if removed.size == cards.size
  then game.validNel
  else s"Player $playerId does not own card(s) ($cards)".invalidNel

def confirmPlayerHoldsCard[A <: ActiveGame](playerId: Player.Id, card: Card)(game: A): ValidatedNel[String, A] =
  confirmPlayerHoldsCards(playerId, Seq(card))(game)

def confirmCribHasAllDiscards(game: DiscardingGame): ValidatedNel[String, DiscardingGame] =
  if game.crib.hasAllDiscards
  then game.validNel
  else s"Crib required further discards in ${game.id}".invalidNel

def confirmPlayersTurnToPlay(player: Player.Id)(game: PlayingGame): ValidatedNel[String, PlayingGame] =
  val plays = game.plays
  val nextToPlayHasAlreadyPassed = plays.passedPlayers.contains(plays.nextToPlay)
  val reallyNextToPlay = game.plays.nextToPlay == player
  if nextToPlayHasAlreadyPassed || reallyNextToPlay
  then game.validNel
  else s"It is not Player $player's turn to play".invalidNel

def confirmRunningTotalWillNotExceed31(card: Card)(game: PlayingGame): ValidatedNel[String, PlayingGame] =
  val runningTotal = game.plays.runningTotal
  if runningTotal + card.face.value <= 31
  then game.validNel
  else s"Playing $card exceeds 31; current play total is $runningTotal".invalidNel

def confirmNoValidCardsToPlay(player: Player.Id)(game: PlayingGame): ValidatedNel[String, PlayingGame] =
  val runningTotal = game.plays.runningTotal
  val cardValues = game.hands(player).values
  val validPlays = cardValues.filter(_ + runningTotal <= 31)
  if (validPlays.isEmpty)
  then game.validNel
  else s"Player $player cannot pass; they have one or more valid cards to play".invalidNel
