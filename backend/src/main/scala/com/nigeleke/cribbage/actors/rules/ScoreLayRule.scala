package com.nigeleke.cribbage.actors.rules

import com.nigeleke.cribbage.actors.Game._
import com.nigeleke.cribbage.model._

object ScorePlayRule extends Rule {

  override def commands(state: State): Seq[Command] = {

    def scorePlay(play: Play) = {
      val lays = play.current
      val cards = lays.map(_.card)
      fifteensInPlay(cards) + pairsInPlay(cards) + runsInPlay(cards) + thirtyOnePoints(cards)
    }

    def fifteensInPlay(cards: Cards) = {
      val total = cards.map(_.value).foldLeft(0)(_ + _)
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
      differencesNotByOne.size == 0
    }

    def thirtyOnePoints(cards: Cards) = {
      val total = cards.map(_.value).foldLeft(0)(_ + _)
      if (total == 31) 2 else 0
    }

    val game = state.game
    val scorerId = game.play.current.last.playerId
    val score = scorePlay(game.play)

    if (score != 0) Seq(PegScore(scorerId, score))
    else Seq.empty
  }

}
