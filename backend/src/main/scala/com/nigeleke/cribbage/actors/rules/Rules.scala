package com.nigeleke.cribbage.actors.rules

import com.nigeleke.cribbage.actors.Game._
import com.nigeleke.cribbage.model.{Card, Cards, Game, Play}
import com.nigeleke.cribbage.suit.Face

object Rules {

  def cutAtStartOfPlay(game: Game): Seq[Command] = {
    val needToCut = game.crib.size == 4
    if (needToCut) Seq(CutAtStartOfPlay)
    else Seq.empty
  }

  def cutForDeal(game: Game): Seq[Command] = {
    val needToCutForDeal = game.players.size == 2 && game.optDealer.isEmpty
    if (needToCutForDeal) Seq(CutForDeal)
    else Seq.empty
  }

  def deal(game: Game): Seq[Command] = {
    val dealRequired = game.optDealer.isDefined && game.hands.isEmpty
    if (dealRequired) Seq(DealHands)
    else Seq.empty[Command]
  }

  def declareWinner(game: Game): Seq[Command] =
    (for {
      (playerId, score) <- game.scores if score.front >= 121
    } yield DeclareWinner(playerId)).toSeq

  def nextToLay(game: Game): Seq[Command] = {
    Seq.empty
  }

  val scoreCutAtStartOfPlay : Game => Seq[Command] = { game: Game =>
    (for {
      dealer <- game.optDealer
      cut <- game.optCut if cut.face == Face.Jack
    } yield PegScore(dealer, 2)).toSeq
  }

  def scoreLay(game: Game): Seq[Command] = {

    def scorePlay(play: Play) = {
      val lays = play.current
      val cards = lays.map(_.card)
      fifteensInPlay(cards) + pairsInPlay(cards) + runsInPlay(cards)
    }

    def fifteensInPlay(cards: Cards) = {
      val total = cards.map(_.value).sum
      if (total == 15) 2 else 0
    }

    def pairsInPlay(cards: Cards) = {
      val reversed = cards.reverse
      val optPlayedCard = reversed.headOption
      (for {
        lastCard <- optPlayedCard
        matchingCards = reversed.drop(1).takeWhile(_.face == lastCard.face)
        size = matchingCards.size
        points = Map(0 -> 0, 1 -> 2, 2 -> 6, 3 -> 12)(size)
      } yield points).getOrElse(0)
    }

    def runsInPlay(cards: Cards) = {
      val reversed = cards.reverse
      val optPlayedCard = reversed.headOption
      val runLengths = reversed.size to 3 by -1
      (for {
        playedCard <- optPlayedCard
        bestRunLength <- runLengths.dropWhile(length => !makesRun(playedCard, reversed.take(length))).headOption
      } yield bestRunLength).getOrElse(0)
    }

    def makesRun(playedCard: Card, cards: Cards) = isRun(cards) && cards.contains(playedCard)

    def isRun(cards: Cards) = {
      val sorted = cards.sortBy(_.rank)
      val differences = sorted.sliding(2).map { case Seq(x, y, _*) => y.rank - x.rank }
      val differencesNotByOne = differences.filterNot(_ == 1)
      differencesNotByOne.isEmpty
    }

    val scorerId = game.play.current.last.playerId
    val score = scorePlay(game.play)

    if (score != 0) Seq(PegScore(scorerId, score))
    else Seq.empty
  }

  def scorePass(game: Game) : Seq[Command] = {
    val bothPassed = game.play.passCount == 2
    val currentRunningTotal = game.play.runningTotal
    game.play.optNextToLay.flatMap { nextToLay =>
      val scorer = game.opponent(nextToLay)
      (bothPassed, currentRunningTotal) match {
        case (false, _) => None
        case (true, 31) => Some(PegScore(scorer, 2))
        case (true,  _) => Some(PegScore(scorer, 1))
      }
    }.toSeq
  }

}
