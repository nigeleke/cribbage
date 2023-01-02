package cribbage
package scorer

import model.*
import model.Score.*

object Scorer:

  val scoreHisHeels: Playing => Playing | Finished =
    case playing @ Playing(currentScores, _, dealer, _, _, cut, _) =>
      if cut.face == Card.Face.Jack
      then scoreFor(dealer, 2)(playing)
      else playing

  private def scoreFor(player: Player, points: Int): Playing => Playing | Finished =
    case playing @ Playing(scores, _, _, _, _, _, _) =>
      val updatedScore  = scores(player).add(points)
      val updatedScores = scores.updated(player, updatedScore)
      if updatedScore.isWinningScore
      then Finished(scores = updatedScores)
      else playing.copy(scores = updatedScores)

  val scorePlay: Playing => Playing | Finished =
    case playing @ Playing(scores, hands, dealer, pone, crib, cut, plays) =>
      val fifteens   = if plays.runningTotal == 15 then 2 else 0
      val pairs      =
        val reversed      = plays.cardsInPlay.reverse
        val optPlayedCard = reversed.headOption
        (for {
          lastCard     <- optPlayedCard
          matchingCards = reversed.drop(1).takeWhile(_.face == lastCard.face)
          size          = matchingCards.size
          points        = Map(0 -> 0, 1 -> 2, 2 -> 6, 3 -> 12)(size)
        } yield points).getOrElse(0)
      val runs       =
        val reversed      = plays.cardsInPlay.reverse
        val optPlayedCard = reversed.headOption
        val runLengths    = reversed.size to 3 by -1
        (for
          playedCard    <- optPlayedCard
          bestRunLength <-
            runLengths.dropWhile(length => !makesRun(playedCard, reversed.take(length))).headOption
        yield bestRunLength).getOrElse(0)
      val play       = (plays.runningTotal, plays.allPlaysCompleted) match
        case (PlayTarget, _) => 2
        case (_, true)       => 1
        case (_, false)      => 0
      val totalScore = fifteens + pairs + runs + play
      scoreFor(plays.nextPlayer, totalScore)(playing)

  private def isRun(cards: Seq[Card]) =
    val sorted              = cards.sortBy(_.rank)
    val differences         = sorted.sliding(2).map { case Seq(x, y, _*) => y.rank - x.rank }
    val differencesNotByOne = differences.filterNot(_ == 1)
    differencesNotByOne.isEmpty

  private def makesRun(playedCard: Card, cards: Seq[Card]) =
    isRun(cards) && cards.contains(playedCard)

  val scorePass: Playing => Playing | Finished =
    case playing @ Playing(_, _, _, _, _, _, plays) =>
      val passScore = if plays.allPassed then 1 else 0
      scoreFor(plays.nextPlayer, passScore)(playing) // 2 for 31 scored in scorePlay

  val scoreHands: Playing => Playing | Finished =
    case playing: Playing => (scorePoneHand andThen scoreDealerHand andThen scoreCrib)(playing)

  private val scorePoneHand: Playing => Playing | Finished =
    case playing @ Playing(_, hands, _, pone, _, cut, _) =>
      scoreCards(pone, hands(pone).toSeq, cut, false)(playing)

  private val scoreDealerHand: Playing | Finished => Playing | Finished =
    case playing @ Playing(_, hands, dealer, _, _, cut, _) =>
      scoreCards(dealer, hands(dealer).toSeq, cut, false)(playing)
    case finished: Finished                                => finished

  private val scoreCrib: Playing | Finished => Playing | Finished =
    case playing @ Playing(_, _, dealer, _, crib, cut, _) =>
      scoreCards(dealer, crib.toSeq, cut, true)(playing)
    case finished: Finished                               => finished

  private def scoreCards(
      player: Player,
      cards: Seq[Card],
      cut: Card,
      inCrib: Boolean
  ): Playing => Playing | Finished =
    case playing: Playing =>
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
        val allFlush   = allCards.size == 5 && allCards.groupBy(_.suit).size == 1
        val cardsFlush = !inCrib && cards.size == 4 && cards.toSeq.groupBy(_.suit).size == 1
        if (allFlush) 5
        else if (cardsFlush) 4
        else 0

      scoreFor(player, pairs + fifteens + runs + flushes + heels)(playing)
