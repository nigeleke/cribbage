package com.nigeleke.cribbage.actors.rules

import com.nigeleke.cribbage.actors.Game._
import com.nigeleke.cribbage.model.{Card, Cards, Game}
import com.nigeleke.cribbage.model.Player.{Id => PlayerId}
import com.nigeleke.cribbage.suit.Face

import scala.language.implicitConversions

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

  def endPlay(game: Game): Seq[Command] = {
    val play = game.play

    val twoFormalPasses = play.passCount == 2
    val allCardsLaid = game.hands.forall(_._2.size == 0)

    val playEndedAt31 = play.runningTotal == 31
    val playEndedBelow31 = (twoFormalPasses || allCardsLaid) && !playEndedAt31
    val playEnded = playEndedAt31 || playEndedBelow31

    val scorerId = play.current.last.playerId
    val points = {
      implicit def booleanToInt(b: Boolean) = if (b) 1 else 0
      playEndedBelow31 * 1 + playEndedAt31 * 2
    }

    if (playEnded) Seq(PegScore(scorerId, points), CompletePlay)
    else Seq.empty
  }

  def endPlays(game: Game): Seq[Command] = {
    val allCardsLaid = game.hands.forall(_._2.size == 0)
    if (allCardsLaid) Seq(CompletePlays)
    else Seq.empty
  }

  val scoreCutAtStartOfPlay : Game => Seq[Command] = { game: Game =>
    (for {
      dealer <- game.optDealer
      cut <- game.optCut if cut.face == Face.Jack
    } yield PegScore(dealer, 2)).toSeq
  }

  def scoreLay(game: Game): Seq[Command] = {

    val play = game.play
    val currentCards = play.current.map(_.card)

    val fifteensInPlay = {
      val total = currentCards.map(_.value).sum
      if (total == 15) 2 else 0
    }

    val pairsInPlay = {
      val reversed = currentCards.reverse
      val optPlayedCard = reversed.headOption
      (for {
        lastCard <- optPlayedCard
        matchingCards = reversed.drop(1).takeWhile(_.face == lastCard.face)
        size = matchingCards.size
        points = Map(0 -> 0, 1 -> 2, 2 -> 6, 3 -> 12)(size)
      } yield points).getOrElse(0)
    }

    val runsInPlay = {
      val reversed = currentCards.reverse
      val optPlayedCard = reversed.headOption
      val runLengths = reversed.size to 3 by -1
      (for {
        playedCard <- optPlayedCard
        bestRunLength <- runLengths.dropWhile(length => !makesRun(playedCard, reversed.take(length))).headOption
      } yield bestRunLength).getOrElse(0)
    }

    val score = fifteensInPlay + pairsInPlay + runsInPlay
    val scorerId = game.play.current.last.playerId

    if (score != 0) Seq(PegScore(scorerId, score))
    else Seq.empty
  }

  def scorePone(game: Game) : Seq[Command] = {
    val poneId = game.optPone.get
    val cut = game.optCut.get
    scoreCards(poneId, game.hands(poneId), cut)
  }

  def scoreDealer(game: Game) : Seq[Command] = {
    val dealerId = game.optDealer.get
    val cut = game.optCut.get
    scoreCards(dealerId, game.hands(dealerId), cut) ++ scoreCards(dealerId, game.crib, cut) :+ SwapDealer
  }

  private def scoreCards(playerId: PlayerId, cards: Cards, cut: Card) : Seq[Command] = {

    val allCards = cards :+ cut

    val fifteens = {
      val nCards = 2 to 5
      val fifteens = for {
        n <- nCards
        c <- allCards.combinations(n)
        total = c.map(_.value).foldLeft(0)(_ + _) if total == 15
      } yield ("fifteen: ", c)
      fifteens.size * 2
    }

    val pairs = {
      val pairs = for {
        c <- allCards.combinations(2)
        c1 <- c.headOption
        c2 <- c.lastOption
        isPair = c1.face == c2.face if isPair
      } yield ("pair: ", c)
      pairs.size * 2
    }

    val runs = {
      val nCards = 3 to 5
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
    }

    val hisHeels = (for {
      card <- cards if card.face == Face.Jack && card.suit == cut.suit
    } yield card).length

    val flushes = {
      val allFlush = (cards :+ cut).groupBy(_.suit).size == 1
      val cardsFlush = cards.groupBy(_.suit).size == 1
      if (allFlush) 5
      else if (cardsFlush) 4
      else 0
    }

    val points = fifteens + pairs + runs + hisHeels + flushes

    if (points != 0) Seq(PegScore(playerId, points))
    else Seq.empty
  }

  private def isRun(cards: Cards) = {
    val sorted = cards.sortBy(_.rank)
    val differences = sorted.sliding(2).map { case Seq(x, y, _*) => y.rank - x.rank }
    val differencesNotByOne = differences.filterNot(_ == 1)
    differencesNotByOne.isEmpty
  }

  private def makesRun(playedCard: Card, cards: Cards) = isRun(cards) && cards.contains(playedCard)
}
