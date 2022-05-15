package com.nigeleke.cribbage.util

import com.nigeleke.cribbage.model.*

import cats.data.ValidatedNel
import cats.syntax.validated.*

type ActiveGame = DiscardingGame | PlayingGame

def confirmNeedsAnotherPlayer(game: StartingGame): ValidatedNel[String, StartingGame] =
  if game.players.size < Game.maxPlayers
  then game.validNel
  else s"Game ${game.id} does not need any further players".invalidNel

def confirmNotExistingPlayer(id: PlayerId)(game: StartingGame): ValidatedNel[String, StartingGame] =
  if !game.players.contains(id)
  then game.validNel
  else s"Player $id has already joined Game ${game.id}".invalidNel

def confirmCorrectNumberOfPlayers(game: StartingGame): ValidatedNel[String, StartingGame] =
  if game.players.size == Game.maxPlayers
  then game.validNel
  else s"Game ${game.id} requires more Players".invalidNel

def confirmExistingPlayer[A <: ActiveGame](player: PlayerId)(game: A): ValidatedNel[String, A] =
  if game.players.contains(player)
  then game.validNel
  else s"Player $player has not joined Game ${game.id}".invalidNel

def confirmDiscardingTwoCards(discards: Seq[Card])(game: DiscardingGame): ValidatedNel[String, DiscardingGame] =
  val expectedDiscardCount = 2
  val actualDiscardCount = discards.size
  if actualDiscardCount == expectedDiscardCount
  then game.validNel
  else s"Game ${game.id} expecting $expectedDiscardCount cards discarded; have $actualDiscardCount".invalidNel

def confirmPlayerHoldsCards[A <: ActiveGame](playerId: PlayerId, cards: Seq[Card])(game: A): ValidatedNel[String, A] =
  val (removed, _) = game.hands(playerId).remove(cards)
  if removed.size == cards.size
  then game.validNel
  else s"Player $playerId does not own card(s) ($cards)".invalidNel

def confirmPlayerHoldsCard[A <: ActiveGame](playerId: PlayerId, card: Card)(game: A): ValidatedNel[String, A] =
  confirmPlayerHoldsCards(playerId, Seq(card))(game)

def confirmCribHasAllDiscards(game: DiscardingGame): ValidatedNel[String, DiscardingGame] =
  if game.crib.hasAllDiscards
  then game.validNel
  else s"Crib required further discards in ${game.id}".invalidNel

def confirmPlayersTurnToPlay(player: PlayerId)(game: PlayingGame): ValidatedNel[String, PlayingGame] =
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

def confirmNoValidCardsToPlay(player: PlayerId)(game: PlayingGame): ValidatedNel[String, PlayingGame] =
  val runningTotal = game.plays.runningTotal
  val cardValues = game.hands(player).cardValues
  val validPlays = cardValues.filter(_ + runningTotal <= 31)
  if (validPlays.isEmpty)
  then game.validNel
  else s"Player $player cannot pass; they have one or more valid cards to play".invalidNel
