/*
 * Copyright (C) 2020  Nigel Eke
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

package com.nigeleke.cribbage.entity.handlers

import akka.pattern.StatusReply
import akka.pattern.StatusReply._
import akka.persistence.typed.scaladsl.{ Effect, EffectBuilder, ReplyEffect }
import com.nigeleke.cribbage.entity.GameEntity._
import com.nigeleke.cribbage.model._
import com.nigeleke.cribbage.model.Card.{ Id => CardId }
import com.nigeleke.cribbage.model.Deck._
import com.nigeleke.cribbage.model.Face
import com.nigeleke.cribbage.model.Player.{ Id => PlayerId }

import scala.language.implicitConversions

trait CommandHandler {

  def handle(command: Command): ReplyEffect[Event, State] =
    if (canDo) acceptedEffect.thenReply(command.replyTo)(state => acceptedReply)
    else Effect.reply(command.replyTo)(Error(ErrorMessage(s"Rejected $command: rejectionReasons")))

  def canDo: Boolean
  def rejectionReasons: String
  def acceptedEffect: EffectBuilder[Event, State]
  def acceptedReply: StatusReply[_] = Ack

}

object CommandHandler {

  def scoreCutAtStartOfPlay(game: Game): Seq[Event] = {
    val deck = game.deck.shuffled
    val cut = deck.head // Will always be Some[Card]
    val dealer = game.optDealer.get
    val points = if (cut.face == Face.Jack) 2 else 0
    val cutEvent: Event = PlayCutRevealed(cut)
    val scoreEvent: Seq[Event] = if (points != 0) Seq(PointsScored(dealer, points)) else Seq.empty
    val winnerEvent: Seq[Event] = checkWinner(game, dealer, points)
    (cutEvent +: (scoreEvent ++ winnerEvent)).toList
  }

  private def checkWinner(game: Game, playerId: PlayerId, points: Int) = {
    val currentScore = game.scores(playerId).front
    if (currentScore + points >= 121) Seq(WinnerDeclared(playerId)) else Seq.empty
  }

  def scoreLay(game: Game): Seq[Event] = {
    val play = game.play
    val currentCardIds = play.current.map(_.cardId)
    val currentCards = currentCardIds.map(game.card(_))

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

    val points = fifteensInPlay + pairsInPlay + runsInPlay
    val scorerId = play.current.last.playerId

    val scoreEvent = if (points != 0) Seq(PointsScored(scorerId, points)) else Seq.empty
    val winnerEvent: Seq[Event] = checkWinner(game, scorerId, points)

    scoreEvent ++ winnerEvent
  }

  def endPlay(game: Game): Seq[Event] = {
    val play = game.play
    val playerId = play.current.last.playerId

    val twoFormalPasses = play.passCount == 2
    val allCardsLaid = game.hands.forall(_._2.isEmpty)

    val playEndedAt31 = game.runningTotal == 31
    val playEndedBelow31 = (twoFormalPasses || allCardsLaid) && !playEndedAt31
    val playEnded = playEndedAt31 || playEndedBelow31

    val points = {
      implicit def booleanToInt(b: Boolean): Int = if (b) 1 else 0
      playEndedBelow31 * 1 + playEndedAt31 * 2
    }

    if (playEnded) Seq(PointsScored(playerId, points), PlayCompleted)
    else Seq.empty
  }

  def endPlays(game: Game): Seq[Event] = {
    val allCardsLaid = game.hands.forall(_._2.isEmpty)
    if (allCardsLaid) PlaysCompleted +: scoreHands(game.withPlaysReturned())
    else Seq.empty
  }

  def scoreHands(game: Game): Seq[Event] = {
    val cutId = game.optCut.get

    def scoreWithWinner(scorerId: PlayerId, points: Int, scoreEvent: Event): Seq[Event] = {
      val winnerEvent =
        if (game.scores(scorerId).front + points >= 121) Seq(WinnerDeclared(scorerId))
        else Seq.empty
      scoreEvent +: winnerEvent
    }

    lazy val scorePone = {
      val poneId = game.optPone.get
      val points = scoreFor(game.hands(poneId), cutId)
      scoreWithWinner(poneId, points.total, PoneScored(poneId, points))
    }

    lazy val scoreDealer = {
      val dealerId = game.optDealer.get
      val handPoints = scoreFor(game.hands(dealerId), cutId)
      val cribPoints = scoreFor(game.crib, cutId)
      scoreWithWinner(dealerId, handPoints.total, DealerScored(dealerId, handPoints)) ++
        scoreWithWinner(dealerId, handPoints.total + cribPoints.total, CribScored(dealerId, cribPoints))
    }

    def scoreFor(cardIds: CardIds, cutId: CardId): Points = {

      val cards = cardIds.map(game.card)
      val cut = game.card(cutId)
      val allCards = cards :+ cut

      val fifteens = {
        val nCards = 2 to 5
        val nFifteens = for {
          n <- nCards
          c <- allCards.combinations(n)
          total = c.map(_.value).sum if total == 15
        } yield ("fifteen: ", c)
        nFifteens.size * 2
      }

      val pairs = {
        val nPairs = for {
          c <- allCards.combinations(2)
          c1 <- c.headOption
          c2 <- c.lastOption
          isPair = c1.face == c2.face if isPair
        } yield ("pair: ", c)
        nPairs.size * 2
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

      val heels = (for {
        card <- cards if card.face == Face.Jack && card.suit == cut.suit
      } yield card).length

      val flushes = {
        val allFlush = (cards :+ cut).groupBy(_.suit).size == 1
        val cardsFlush = cards.groupBy(_.suit).size == 1
        if (allFlush) 5
        else if (cardsFlush) 4
        else 0
      }

      Points(pairs = pairs, fifteens = fifteens, runs = runs, flushes = flushes, heels = heels)
    }

    scorePone ++ scoreDealer :+ DealerSwapped
  }

  private def isRun(cards: Cards) = {
    val sorted = cards.sortBy(_.rank)
    val differences = sorted.sliding(2).map { case Seq(x, y, _*) => y.rank - x.rank }
    val differencesNotByOne = differences.filterNot(_ == 1)
    differencesNotByOne.isEmpty
  }

  private def makesRun(playedCard: Card, cards: Cards) = isRun(cards) && cards.contains(playedCard)

}
