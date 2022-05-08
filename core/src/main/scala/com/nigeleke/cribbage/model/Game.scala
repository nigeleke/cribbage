package com.nigeleke.cribbage.model

import com.nigeleke.cribbage.util.*

import cats.*
import cats.data.ValidatedNel
import cats.syntax.validated.*

import java.util.UUID
import scala.util.Random

type GameId = Game.Id

final case class Game(
    id: GameId,
    deck: Deck,
    players: Seq[PlayerId],
    scores: Map[PlayerId, Score],
    hands: Map[PlayerId, Hand],
    optDealer: Option[PlayerId],
    crib: Crib,
    optCut: Option[Card],
    plays: Plays
)

object Game:
  opaque type Id = UUID

  val maxPlayers = 2

  def apply(): Game =
    Game(
      id = UUID.randomUUID(),
      deck = Deck.shuffledDeck,
      players = Seq.empty,
      scores = Map.empty,
      hands = Map.empty,
      optDealer = None,
      crib = Crib(),
      optCut = None,
      plays = Plays()
    )

extension (game: Game)
  def optOpponent(playerId: PlayerId): Option[PlayerId] = game.players.filterNot(_ == playerId).headOption
  def optPone: Option[PlayerId] = game.optDealer.flatMap(game.optOpponent)

private def confirmNeedsAnotherPlayer(game: Game): ValidatedNel[String, Game] =
  if game.players.size < Game.maxPlayers
  then game.validNel
  else s"Game ${game.id} does not need any further players".invalidNel

private def confirmNotExistingPlayer(id: PlayerId)(game: Game): ValidatedNel[String, Game] =
  if !game.players.contains(id)
  then game.validNel
  else s"Player $id has already joined Game ${game.id}".invalidNel

private def confirmExistingPlayer(id: PlayerId)(game: Game): ValidatedNel[String, Game] =
  if game.players.contains(id)
  then game.validNel
  else s"Player $id has not joined Game ${game.id}".invalidNel

private def confirmCorrectNumberOfPlayers(game: Game): ValidatedNel[String, Game] =
  if game.players.size == Game.maxPlayers
  then game.validNel
  else s"Game ${game.id} requires more Players".invalidNel

private def confirmNotStarted(game: Game): ValidatedNel[String, Game] =
  if game.optDealer.isEmpty
  then game.validNel
  else s"Game ${game.id} has already been started".invalidNel

private def confirmDiscardingTwoCards(discards: Seq[Card])(game: Game): ValidatedNel[String, Game] =
  val expectedDiscardCount = 2
  val actualDiscardCount = discards.size
  if actualDiscardCount == expectedDiscardCount
  then game.validNel
  else s"Game ${game.id} expecting $expectedDiscardCount cards discarded; have $actualDiscardCount".invalidNel

private def confirmPlayerHoldsCards(playerId: PlayerId, cards: Seq[Card])(game: Game): ValidatedNel[String, Game] =
  val (removed, _) = game.hands(playerId).remove(cards)
  if removed.size == cards.size
  then game.validNel
  else s"Player $playerId does not own card(s) ($cards)".invalidNel

private def confirmPlayerHoldsCard(playerId: PlayerId, card: Card)(game: Game): ValidatedNel[String, Game] =
  confirmPlayerHoldsCards(playerId, Seq(card))(game)

private def confirmPlayersTurnToPlay(playerId: PlayerId)(game: Game): ValidatedNel[String, Game] =
  val plays = game.plays
  val nextToPlayHasAlreadyPassed = plays.optNextToPlay.map(playerId => plays.passedPlayers.contains(playerId)).getOrElse(false)
  val reallyNextToPlay = game.plays.optNextToPlay == Some(playerId)
  if nextToPlayHasAlreadyPassed || reallyNextToPlay
  then game.validNel
  else s"It is not Player $playerId's turn to play".invalidNel

private def confirmRunningTotalDoesNotExceed31(card: Card)(game: Game): ValidatedNel[String, Game] =
  val runningTotal = game.plays.runningTotal
  if runningTotal + card.face.value <= 31
  then game.validNel
  else s"Playing $card exceeds 31; current play total is $runningTotal".invalidNel

private def confirmNoValidCardsToPlay(playerId: PlayerId)(game: Game): ValidatedNel[String, Game] =
  val runningTotal = game.plays.runningTotal
  val cardValues = game.hands(playerId).cardValues
  val validPlays = cardValues.filter(_ + runningTotal <= 31)
  if (validPlays.isEmpty)
  then game.validNel
  else s"Player $playerId cannot pass; they have one or more valid cards to play".invalidNel

def addPlayer(id: PlayerId)(game: Game): ValidatedNel[String, Game] =
  def addPlayer(game: Game) = game.copy(players = game.players :+ id).validNel
  game.validNel andThen
    confirmNeedsAnotherPlayer andThen
    confirmNotExistingPlayer(id) andThen
    addPlayer

def start(game: Game): ValidatedNel[String, Game] =
  lazy val selectDealer = (_: Game) =>
    val ids = game.players
    val dealerId = ids(Random.nextInt(Game.maxPlayers))
    val updatedScores = game.players.map(id => (id, Score.zero)).toMap
    game.copy(optDealer = Some(dealerId), scores = updatedScores).validNel
  game.validNel andThen
    confirmCorrectNumberOfPlayers andThen
    confirmNotStarted andThen
    selectDealer andThen
    deal

def deal(game: Game): ValidatedNel[String, Game] =
  val (deck, hands) = Deck.deal
  val updatedHands = game.players.zip(hands).toMap
  val updatedCrib = Crib()
  val updatedPlays = Plays(game.optPone)
  game.copy(deck = deck, hands = updatedHands, crib = updatedCrib, plays = updatedPlays).validNel

def discardCribCards(playerId: PlayerId, cards: Seq[Card])(game: Game): ValidatedNel[String, Game] =
  def discard(game: Game) =
    val (discarded, remaining) = game.hands(playerId).remove(cards)
    val updatedHands = game.hands.updated(playerId, Hand(remaining))
    val updatedCrib = game.crib ++ discarded
    game.copy(hands = updatedHands, crib = updatedCrib).validNel
  def initialPlays(game: Game) = game.copy(plays = Plays(game.optPone)).validNel
  def cutStarter(game: Game) = game.copy(optCut = game.deck.headOption).validNel
  def scoreHisHeels(game: Game) =
    game.optCut match
      case Some(Card(_, Card.Face.Jack, _)) => scorePoints(playerId, 2)(game)
      case _                                => game.validNel
  def startPlays(game: Game) =
    if game.crib.hasAllDiscards
    then initialPlays(game)
    else game.validNel
  game.validNel andThen
    confirmExistingPlayer(playerId) andThen
    confirmDiscardingTwoCards(cards) andThen
    confirmPlayerHoldsCards(playerId, cards) andThen
    discard andThen
    cutStarter andThen
    scoreHisHeels andThen
    startPlays

def scorePoints(playerId: PlayerId, points: Int)(game: Game): ValidatedNel[String, Game] =
  val score = game.scores(playerId)
  val updatedScore = Score(score.front, score.front + points)
  val updatedScores = game.scores.updated(playerId, updatedScore)
  game.copy(scores = updatedScores).validNel

def playCard(playerId: PlayerId, card: Card)(game: Game): ValidatedNel[String, Game] =
  def playCard(game: Game): ValidatedNel[String, Game] =
    val updatedOptNextToPlay = game.optOpponent(playerId)
    val (_, remaining) = game.hands(playerId).remove(Seq(card))
    val updatedHands = game.hands.updated(playerId, Hand(remaining))
    val plays = game.plays
    val updatedCurrent = plays.current :+ Plays.Play.Laid(playerId, card)
    val updatedPlays = plays.copy(optNextToPlay = updatedOptNextToPlay, current = updatedCurrent)
    game.copy(hands = updatedHands, plays = updatedPlays).validNel
  def scorePlay(game: Game): ValidatedNel[String, Game] =
    val plays = game.plays
    val totalPoints = Scorer.forPlay(plays.current).total + Scorer.forEndPlay(plays).total
    val scorerId = plays.current.last match
      case Plays.Play.Laid(id, _) => id
      case Plays.Play.Pass(id)    => id
    val updatedScore = game.scores(scorerId).add(totalPoints)
    val updatedScores = game.scores.updated(scorerId, updatedScore)
    game.copy(scores = updatedScores).validNel
  game.validNel andThen
    confirmExistingPlayer(playerId) andThen
    confirmPlayersTurnToPlay(playerId) andThen
    confirmPlayerHoldsCard(playerId, card) andThen
    confirmRunningTotalDoesNotExceed31(card) andThen
    playCard andThen
    scorePlay

def pass(playerId: PlayerId)(game: Game): ValidatedNel[String, Game] =
  def pass(game: Game): ValidatedNel[String, Game] =
    val updatedOptNextToPlay = game.optOpponent(playerId)
    val plays = game.plays
    val updatedCurrent = plays.current :+ Plays.Play.Pass(playerId)
    val updatedPassedPlayers = plays.passedPlayers + playerId
    val updatedPlays =
      plays.copy(optNextToPlay = updatedOptNextToPlay, current = updatedCurrent, passedPlayers = updatedPassedPlayers)
    game.copy(plays = updatedPlays).valid
  def scorePass(game: Game): ValidatedNel[String, Game] =
    val plays = game.plays
    val points = Scorer.forEndPlay(plays)
    val updatedScore = game.scores(playerId).add(points.total)
    val updatedScores = game.scores.updated(playerId, updatedScore)
    game.copy(scores = updatedScores).validNel
  def resetPlays(game: Game): ValidatedNel[String, Game] =
    val plays = game.plays
    val updatedPlays =
      if plays.passCount == 2
      then plays.nextPlay
      else plays
    game.copy(plays = updatedPlays).validNel
  game.validNel andThen
    confirmExistingPlayer(playerId) andThen
    confirmPlayersTurnToPlay(playerId) andThen
    confirmNoValidCardsToPlay(playerId) andThen
    pass andThen
    scorePass andThen
    resetPlays

def regatherPlays(game: Game): ValidatedNel[String, Game] =
  def regatherPlays: ValidatedNel[String, Game] =
    val plays = game.plays
    val (dealerPlays, ponePlays) = plays.laidSoFar.partition(play => Some(play.playerId) == game.optDealer)
    val dealerHand = dealerPlays.map(_.card)
    val poneHand = ponePlays.map(_.card)
    val updatedHands = Map(game.optDealer.get -> Hand(dealerHand), game.optPone.get -> Hand(poneHand))
    game.copy(hands = updatedHands, plays = Plays()).validNel
  val playersHandsPlayed = game.hands.values.forall(_.isEmpty)
  if playersHandsPlayed
  then regatherPlays
  else s"There are still cards left to play in ${game.id}".invalidNel

def scorePoneHand(game: Game): ValidatedNel[String, Game] =
  val poneId = game.optPone.get
  scoreCardsForPlayer(game, poneId, game.hands(poneId).toCardSeq, game.optCut.get)

def scoreDealerHand(game: Game): ValidatedNel[String, Game] =
  val dealerId = game.optDealer.get
  scoreCardsForPlayer(game, dealerId, game.hands(dealerId).toCardSeq, game.optCut.get)

def scoreCrib(game: Game): ValidatedNel[String, Game] =
  val dealerId = game.optDealer.get
  scoreCardsForPlayer(game, dealerId, game.crib.toCardSeq, game.optCut.get)

private def scoreCardsForPlayer(game: Game, playerId: PlayerId, cards: Seq[Card], cut: Card): ValidatedNel[String, Game] =
  val points = Scorer.forCards(cards, cut)
  val updatedScore = game.scores(playerId).add(points.total)
  val updatedScores = game.scores.updated(playerId, updatedScore)
  game.copy(scores = updatedScores).validNel

def swapDealer(game: Game): ValidatedNel[String, Game] =
  game.copy(optDealer = game.optPone, deck = Deck.shuffledDeck).validNel andThen deal
