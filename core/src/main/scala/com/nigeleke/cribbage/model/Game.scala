package com.nigeleke.cribbage.model

import com.nigeleke.cribbage.util.*
import GameState.*
import scala.util.Random

case class Game(state: GameState)

object Game:
  def create = Game(Starting(Set.empty))

extension (game: Game)
  def players: Set[Player] =
    game.state match
      case Starting(players)                 => players
      case Discarding(_, scores, _, _, _, _) => scores.keySet
      case Playing(scores, _, _, _, _, _, _) => scores.keySet
      case Scoring(scores, _, _, _, _, _)    => scores.keySet
      case Finished(scores)                  => scores.keySet

  def opponent(player: Player): Player =
    require(game.players.contains(player))
    require(game.players.size == maxPlayers)
    val others = players.filterNot(_ == player)
    others.head

  def scores(player: Player): Score =
    game.state match
      case Starting(_)                       => Score.zero
      case Discarding(_, scores, _, _, _, _) => scores.get(player).getOrElse(Score.zero)
      case Playing(scores, _, _, _, _, _, _) => scores.get(player).getOrElse(Score.zero)
      case Scoring(scores, _, _, _, _, _)    => scores.get(player).getOrElse(Score.zero)
      case Finished(scores)                  => scores.get(player).getOrElse(Score.zero)

  def withState(state: GameState) = Right(game.copy(state = state))
  def withError(message: String)  = Left(message)

  def addPlayer(player: Player): Either[String, Game] = game.state match
    case Starting(players) if players.contains(player)   => withError(s"Player $player already added")
    case Starting(players) if players.size == maxPlayers => withError(s"Cannot add player $player")
    case s @ Starting(players)                           => withState(s.copy(players = players + player))
    case _                                               => withError(s"Cannot add player $player")

  def start: Either[String, Game] = game.state match
    case Starting(players) if players.size == maxPlayers =>
      val scores = players.map((_, Score.zero)).toMap
      val cut    = Random.shuffle(players)
      withState(newDiscarding(scores, cut.head, cut.last))
    case _                                               => withError("Cannot start game")

  private def newDiscarding(scores: Map[Player, Score], dealer: Player, pone: Player): Discarding =
    val (deck, hands) = Deck.shuffledDeck.deal(players.size, cardsPerHand)
    val dealtHands    = players.zip(hands).toMap
    val crib          = Crib.empty
    Discarding(deck, scores, dealtHands, dealer, pone, crib)

  def discardCribCards(player: Player, discards: Seq[Card]): Either[String, Game] =
    game.state match
      case d @ Discarding(_, _, hands, _, _, crib) =>
        lazy val playerHand = hands(player)
        val canDiscard      =
          players.contains(player) &&
            playerHand.size - discards.size >= 4 &&
            discards.forall(playerHand.contains)
        if (canDiscard)
          val updatedHand  = playerHand.filterNot(discards.contains)
          val updatedHands = hands.updated(player, updatedHand)
          val updatedCrib  = crib ++ discards
          withState(d.copy(hands = updatedHands, crib = updatedCrib))
        else withError("Cannot discard cards")
      case _                                       => withError("Cannot discard cards")

  def startPlay(nextPlayer: Player): Either[String, Game] =
    game.state match
      case Discarding(deck, scores, hands, dealer, pone, crib) =>
        val canStartPlay = players.contains(nextPlayer) && crib.size == cribDiscards
        if (canStartPlay)
          val (_, cut)      = deck.cut
          val updatedScore  = scores(dealer).add(Scorer.forCut(cut).total)
          val updatedScores = scores.updated(dealer, updatedScore)
          val plays         = Plays(nextPlayer)
          if updatedScore.points >= winningPoints
          then withState(Finished(updatedScores))
          else withState(Playing(updatedScores, hands, dealer, pone, crib, cut, plays))
        else withError("Cannot start play")
      case _                                                   => withError("Cannot start play")

  def playCard(player: Player, card: Card): Either[String, Game] =
    game.state match
      case p @ Playing(scores, hands, _, _, _, _, plays) =>
        val canPlay =
          plays.nextPlayer == player &&
            hands(player).contains(card) &&
            plays.runningTotal + card.face.value <= playLimit
        if (canPlay)
          val updatedHand   = hands(player).filterNot(_ == card)
          val updatedHands  = hands.updated(player, updatedHand)
          val currentPlay   = plays.inPlay :+ Plays.Laid(player, card)
          val nextPlayer    = game.opponent(player)
          val updatedPlays  = plays.copy(nextPlayer = nextPlayer, inPlay = currentPlay)
          val updatedScore  = scores(player).add(Scorer.forPlay(currentPlay).total)
          val updatedScores = scores.updated(player, updatedScore)
          if updatedScore.points >= winningPoints
          then withState(Finished(updatedScores))
          else withState(p.copy(scores = updatedScores, hands = updatedHands, plays = updatedPlays))
        else withError("Cannot play card")
      case _                                             => withError("Cannot play card")

  def pass(player: Player): Either[String, Game] =
    game.state match
      case p @ Playing(scores, hands, _, _, _, _, plays) =>
        val currentTotal = plays.runningTotal
        val canPass      =
          plays.nextPlayer == player
            && hands(player).forall(_.face.value + currentTotal > playLimit)
        if (canPass)
          val currentPlay   = plays.inPlay :+ Plays.Pass(player)
          val nextPlayer    = game.opponent(player)
          val passedPlayers = plays.passedPlayers + player
          val updatedPlays0 = plays.copy(nextPlayer = nextPlayer, inPlay = currentPlay)
          val resetPlays    = Plays(nextPlayer, Seq.empty, plays.played ++ currentPlay)
          val updatedScore  = scores(player).add(Scorer.forEndPlay(updatedPlays0).total)
          val updatedScores = scores.updated(player, updatedScore)
          val updatedPlays1 = if updatedPlays0.passCount == 2 then resetPlays else updatedPlays0
          if updatedScore.points >= winningPoints
          then withState(Finished(updatedScores))
          else withState(p.copy(scores = updatedScores, plays = updatedPlays1))
        else withError("Cannot pass")
      case _                                             => withError("Cannot pass")

  def regatherPlays: Either[String, Game] =
    game.state match
      case p @ Playing(scores, hands, dealer, pone, crib, cut, plays) =>
        val canRegather  = hands.forall(_._2.isEmpty)
        val updatedHands = (plays.played ++ plays.inPlay)
          .collect { case p: Plays.Laid => p }
          .groupBy(_.player)
          .view
          .mapValues(_.map(_.card))
          .toMap
        if canRegather
        then withState(Scoring(scores, updatedHands, dealer, pone, crib, cut))
        else withError("Cannot regather")
      case _                                                          => withError("Cannot regather")

  def scorePoneHand: Either[String, Game] =
    game.state match
      case s @ Scoring(scores, hands, _, pone, _, cut) =>
        val updatedScore  = scores(pone).add(Scorer.forCards(hands(pone), cut).total)
        val updatedScores = scores.updated(pone, updatedScore)
        if updatedScore.points >= winningPoints
        then withState(Finished(updatedScores))
        else withState(s.copy(scores = updatedScores))
      case _                                           => withError("Cannot score pone hand")

  def scoreDealerHand: Either[String, Game] =
    game.state match
      case s @ Scoring(scores, hands, dealer, _, _, cut) =>
        val updatedScore  = scores(dealer).add(Scorer.forCards(hands(dealer), cut).total)
        val updatedScores = scores.updated(dealer, updatedScore)
        if updatedScore.points >= winningPoints
        then withState(Finished(updatedScores))
        else withState(s.copy(scores = updatedScores))
      case _                                             => withError("Cannot score dealer hand")

  def scoreCrib: Either[String, Game] =
    game.state match
      case s @ Scoring(scores, _, dealer, _, crib, cut) =>
        val updatedScore  = scores(dealer).add(Scorer.forCards(crib, cut).total)
        val updatedScores = scores.updated(dealer, updatedScore)
        if updatedScore.points >= winningPoints
        then withState(Finished(updatedScores))
        else withState(s.copy(scores = updatedScores))
      case _                                            => withError("Cannot score crib")

  def swapDealer: Either[String, Game] =
    game.state match
      case Scoring(scores, _, dealer, pone, _, _) =>
        withState(newDiscarding(scores, pone, dealer))
      case _                                      => withError("Cannot swap dealer")

  def winner: Option[Player] =
    game.state match
      case Finished(scores) => scores.toList.sortBy(_._2.points).lastOption.map(_._1)
      case _                => None

  def loser: Option[Player] = winner.map(opponent)
