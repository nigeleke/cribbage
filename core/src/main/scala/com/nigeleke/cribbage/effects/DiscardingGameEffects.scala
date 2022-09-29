package com.nigeleke.cribbage.effects

import com.nigeleke.cribbage.model.*
import Card.Face.*
import com.nigeleke.cribbage.util.*

import cats.data.ValidatedNel
import cats.syntax.validated.*

def deal(game: DiscardingGame): ValidatedNel[String, DiscardingGame] =
  val (deck, hands) = Deck.deal
  val updatedHands = Seq(game.pone, game.dealer).zip(hands).toMap
  game.copy(deck = deck, hands = updatedHands, crib = Crib()).validNel

def discardCribCards(player: PlayerId, cards: Seq[Card])(game: DiscardingGame): ValidatedNel[String, DiscardingGame] =
  def discard(game: DiscardingGame) =
    val (discarded, remaining) = game.hands(player).remove(cards)
    val updatedHands = game.hands.updated(player, Hand(remaining))
    val updatedCrib = game.crib ++ discarded
    game.copy(hands = updatedHands, crib = updatedCrib).validNel
  game.validNel andThen
    confirmExistingPlayer(player) andThen
    confirmDiscardingTwoCards(cards) andThen
    confirmPlayerHoldsCards(player, cards) andThen
    discard

def startPlay(dealer: PlayerId)(game: DiscardingGame): ValidatedNel[String, PlayingGame | WonGame] =
  def startPlay(game: DiscardingGame) =
    val (_, cut) = game.deck.cut
    PlayingGame(game.id, game.scores, game.hands, game.dealer, game.pone, game.crib, cut, Plays(game.pone)).validNel
  def scoreHisHeels(game: PlayingGame): ValidatedNel[String, PlayingGame | WonGame] =
    game.cut match
      case Card(_, Jack, _) => game.validNel andThen scorePoints(dealer, 2)
      case _                => game.validNel
  game.validNel andThen
    confirmCribHasAllDiscards andThen
    startPlay andThen
    scoreHisHeels
