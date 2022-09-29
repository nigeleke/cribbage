package com.nigeleke.cribbage.effects

import com.nigeleke.cribbage.domain.*
import com.nigeleke.cribbage.util.*

import cats.data.Validated.*
import cats.data.ValidatedNel
import cats.syntax.validated.*

def scorePoneHand(game: ScoringGame): ValidatedNel[String, ScoringGame | WonGame] =
  val pone = game.pone
  scoreCardsForPlayer(game, pone, game.hands(pone), game.cut)

def scoreDealerHand(game: ScoringGame): ValidatedNel[String, ScoringGame | WonGame] =
  val dealerId = game.dealer
  scoreCardsForPlayer(game, dealerId, game.hands(dealerId), game.cut)

def scoreCrib(game: ScoringGame): ValidatedNel[String, ScoringGame | WonGame] =
  val dealerId = game.dealer
  scoreCardsForPlayer(game, dealerId, game.crib, game.cut)

private def scoreCardsForPlayer(
    game: ScoringGame,
    player: Player.Id,
    cards: Seq[Card],
    cut: Card
): ValidatedNel[String, ScoringGame | WonGame] =
  val points = Scorer.forCards(cards, cut).total
  val updatedScore = game.scores(player).add(points)
  val updatedScores = game.scores.updated(player, updatedScore)
  if updatedScore.points < 121
  then game.copy(scores = updatedScores).validNel
  else WonGame(game.id, updatedScores).validNel

def swapDealer(game: ScoringGame): ValidatedNel[String, DiscardingGame] =
  DiscardingGame(
    id = game.id,
    deck = Deck.shuffledDeck,
    scores = game.scores,
    hands = Map.empty,
    dealer = game.pone,
    pone = game.dealer,
    crib = Crib.empty
  ).validNel andThen deal
