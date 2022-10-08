package com.nigeleke.cribbage.util

import com.nigeleke.cribbage.model.*
import Cards.*
import Points.*

object Scorer:

  private def isRun(cards: Seq[Card]) =
    val sorted              = cards.sortBy(_.rank)
    val differences         = sorted.sliding(2).map { case Seq(x, y, _*) => y.rank - x.rank }
    val differencesNotByOne = differences.filterNot(_ == 1)
    differencesNotByOne.isEmpty

  private def makesRun(playedCard: Card, cards: Seq[Card]) =
    isRun(cards) && cards.contains(playedCard)

  def forCut(cut: Card): Cut = Cut(if cut.face == Card.Face.Jack then 2 else 0)

  def forPlay(play: Seq[Plays.Play]): Play =
    val currentCards: Seq[Card] =
      play.collect { case Plays.Laid(_, card) => card }

    val pairsPoints =
      val reversed      = currentCards.reverse
      val optPlayedCard = reversed.headOption
      (for {
        lastCard     <- optPlayedCard
        matchingCards = reversed.drop(1).takeWhile(_.face == lastCard.face)
        size          = matchingCards.size
        points        = Map(0 -> 0, 1 -> 2, 2 -> 6, 3 -> 12)(size)
      } yield points).getOrElse(0)

    val fifteensPoints =
      val total = currentCards.map(_.value).sum
      if (total == 15) 2 else 0

    val runsPoints =
      val reversed      = currentCards.reverse
      val optPlayedCard = reversed.headOption
      val runLengths    = reversed.size to 3 by -1
      (for
        playedCard    <- optPlayedCard
        bestRunLength <-
          runLengths.dropWhile(length => !makesRun(playedCard, reversed.take(length))).headOption
      yield bestRunLength).getOrElse(0)

    Play(pairsPoints, fifteensPoints, runsPoints)

  def forEndPlay(plays: Plays): EndPlay =
    val allCardsLaid           = plays.laidSoFar.size == 8
    val twoPassesInCurrentPlay = plays.passCount == 2
    val isEndPlay              = allCardsLaid || twoPassesInCurrentPlay
    lazy val runningTotalScore = if plays.runningTotal == 31 then 2 else 1
    val endOfPlayPoints        = if isEndPlay then runningTotalScore else 0
    EndPlay(endOfPlayPoints)

  def forCards(cards: Hand | Crib, cut: Card): Cards =
    val allCards = cards.toSeq :+ cut

    val fifteens =
      val nCards    = 2 to 5
      val nFifteens = for {
        n    <- nCards
        c    <- allCards.combinations(n)
        total = c.map(_.value).sum if total == 15
      } yield ("fifteen: ", c)
      nFifteens.size * 2

    val pairs =
      val nPairs = for {
        c     <- allCards.combinations(2)
        c1    <- c.headOption
        c2    <- c.lastOption
        isPair = c1.face == c2.face if isPair
      } yield ("pair: ", c)
      nPairs.size * 2

    val runs =
      val nCards  = 3 to 5
      val allRuns = (for {
        n <- nCards
        c <- allCards.combinations(n) if isRun(c)
      } yield c).groupBy(_.size)

      val (count, length) =
        if (allRuns.isEmpty) (0, 0)
        else {
          val max = allRuns.keySet.max
          (allRuns(max).size, max)
        }

      count * length

    val heels = (for {
      card <- cards.toSeq if card.face == Card.Face.Jack && card.suit == cut.suit
    } yield card).length

    val flushes =
      val allFlush   = allCards.groupBy(_.suit).size == 1
      val cardsFlush = cards.toSeq.groupBy(_.suit).size == 1
      if (allFlush) 5
      else if (cardsFlush) 4
      else 0

    Points.Cards(pairs = pairs, fifteens = fifteens, runs = runs, flushes = flushes, heels = heels)
