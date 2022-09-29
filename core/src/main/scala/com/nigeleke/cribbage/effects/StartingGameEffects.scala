package com.nigeleke.cribbage.effects

import com.nigeleke.cribbage.util.*
import cats.data.ValidatedNel
import cats.syntax.validated.*
import com.nigeleke.cribbage.domain.Crib
import com.nigeleke.cribbage.domain.Deck
import com.nigeleke.cribbage.domain.DiscardingGame
import com.nigeleke.cribbage.domain.Game
import com.nigeleke.cribbage.domain.Hand
import com.nigeleke.cribbage.domain.Player
import com.nigeleke.cribbage.domain.Score
import com.nigeleke.cribbage.domain.StartingGame

import scala.util.Random

def addPlayer(id: Player.Id)(game: StartingGame): ValidatedNel[String, StartingGame] =
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
    val hands = game.players.map(id => (id, Hand.undealt)).toMap
    com.nigeleke.cribbage.domain
      .DiscardingGame(
        game.id,
        Deck.shuffledDeck,
        scores = scores,
        hands = hands,
        dealer = dealer,
        pone = pone,
        crib = Crib.empty
      )
      .validNel
  game.validNel andThen
    confirmCorrectNumberOfPlayers andThen
    initialiseDiscardingGame andThen
    deal
