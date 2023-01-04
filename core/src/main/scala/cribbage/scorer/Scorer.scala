/*
 * Copyright (c) 2022, Nigel Eke
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without modification,
 * are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * 3. Neither the name of the copyright holder nor the names of its contributors
 *    may be used to endorse or promote products derived from this software without
 *    specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON
 * ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

package cribbage
package scorer

import model.*
import model.Score.*

/** The Scorer is used to score different stages of the Cribbage game:
  *   - His Heels (when card is cut after discarding and prior to playing,
  *   - During Play (including when a Player has passed)
  *   - After all Plays, when pone / dealer hands and crib are scored.
  *
  * All scores may result in a Finished game if the points scored make the players total score
  * greater than 121.
  */
object Scorer:

  /** Score his heels - if a Jack is cut prior to Play. */
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

  /** Score the current play as a result of the last card laid. This includes fifteens, runs, pairs
    * and final card play.
    */
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

  /** Scores one for the final play, if neither player acn lay any further cards and the target 31
    * wasn't reached.
    */
  val scorePass: Playing => Playing | Finished =
    case playing @ Playing(_, _, _, _, _, _, plays) =>
      val passScore = if plays.allPassed then 1 else 0
      scoreFor(plays.nextPlayer, passScore)(playing) // 2 for 31 scored in scorePlay

  /** Scores the pone's hand, dealer's hand then crib. Any score may result in a finished game, in
    * which case subsequent hands aren't scored.
    */
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
