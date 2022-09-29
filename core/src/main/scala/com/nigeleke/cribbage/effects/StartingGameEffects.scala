package com.nigeleke.cribbage.effects

import com.nigeleke.cribbage.model.*
import com.nigeleke.cribbage.util.*

import cats.data.ValidatedNel
import cats.syntax.validated.*

import scala.util.Random

def addPlayer(id: PlayerId)(game: StartingGame): ValidatedNel[String, StartingGame] =
  def addPlayer(game: StartingGame) = game.copy(players = game.players :+ id).validNel
  game.validNel andThen
    confirmNeedsAnotherPlayer andThen
    confirmNotExistingPlayer(id) andThen
    addPlayer

def start(game: StartingGame): ValidatedNel[String, DiscardingGame] =
  lazy val initialiseDiscardingGame = (_: StartingGame) =>
    val ids = game.players
    val randomIndex = Random.nextInt(Game.maxPlayers)
    val dealer = ids(randomIndex)
    val pone = ids(1 - randomIndex)
    val scores = game.players.map(id => (id, Score.zero)).toMap
    val hands = game.players.map(id => (id, Hand())).toMap
    DiscardingGame(game.id, Deck.shuffledDeck, scores = scores, hands = hands, dealer = dealer, pone = pone, crib = Crib()).validNel
  game.validNel andThen
    confirmCorrectNumberOfPlayers andThen
    initialiseDiscardingGame andThen
    deal
