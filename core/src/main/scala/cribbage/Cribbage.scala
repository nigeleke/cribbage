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

import model.*
import model.Cards.*
import model.Draw.*
import model.Rule.*

import cribbage.scorer.Scorer.*

type Cribbage[A] = A

object Cribbage:
  def newGame: Cribbage[Draw] = drawForPlayers(Set(Player.newPlayer, Player.newPlayer))

  private def drawForPlayers(players: Set[Player]) =
    val shuffled          = shuffledDeck
    val (remaining, cut1) = shuffled.cut
    val (_, cut2)         = remaining.cut
    val cuts              = Seq(cut1, cut2).sortBy(_.rank)
    val playerByCut       = cuts.zip(players).toMap
    val cutsByPlayer      = for ((k, v) <- playerByCut) yield (v, k)
    val sameRank          = cut1.rank == cut2.rank
    if sameRank
    then Draw.Undecided(cutsByPlayer)
    else Draw.Decided(cutsByPlayer, playerByCut(cuts.head), playerByCut(cuts.last))

  def redraw: Undecided => Draw =
    case Undecided(draws) => drawForPlayers(draws.keySet)

  def dealFirstHand: Decided => Discarding =
    case Decided(_, dealer, pone) =>
      val initialScores = Set(dealer, pone).map((_, Score.zero)).toMap
      newDiscarding(initialScores, dealer, pone)

  private def newDiscarding(scores: Map[Player, Score], dealer: Player, pone: Player): Discarding =
    val (deck, hands) = shuffledDeck.deal(NumberOfPlayersInGame, CardsDealtPerHand)
    val dealtHands    = Set(pone, dealer).zip(hands).toMap
    val crib          = emptyCrib
    Discarding(deck, scores, dealtHands, dealer, pone, crib)

  def discardToCrib(player: Player, discards: Seq[Card]): Discarding => Discarding | Playing |
    Finished =
    case discarding @ Discarding(deck, scores, hands, dealer, pone, crib) =>
      require(Set(dealer, pone).contains(player), NonParticipatingPlayer(player))
      require(hands(player).containsAll(discards), UnheldCards(player, discards))
      require(
        hands(player).removeAll(discards).size >= CardsKeptPerHand,
        TooManyDiscards(player, discards)
      )
      val updatedHand  = hands(player).removeAll(discards)
      val updatedHands = hands.updated(player, updatedHand)
      val updatedCrib  = crib.addAll(discards)
      val (_, cut)     = deck.cut
      if updatedCrib.isFull
      then scoreHisHeels(Playing(scores, updatedHands, dealer, pone, updatedCrib, cut, Plays(pone)))
      else discarding.copy(hands = updatedHands, crib = updatedCrib)

  def play(player: Player, card: Card): Playing => Playing | Discarding | Finished =
    case playing @ Playing(_, hands, dealer, pone, _, _, plays) =>
      def opponent(player: Player): Player = if player == dealer then pone else dealer

      require(Set(dealer, pone).contains(player), NonParticipatingPlayer(player))
      require(plays.nextPlayer == player, NotYourTurn(player))
      require(hands(player).contains(card), UnheldCard(player, card))
      require(plays.runningTotal + card.value <= PlayTarget, PlayTargetExceeded(player, card))

      val updatedHand  = hands(player).remove(card)
      val updatedHands = hands.updated(player, updatedHand)
      val updatedPlays = plays.play(player, card)
      scorePlay(playing.copy(hands = updatedHands, plays = updatedPlays)) match
        case scoredPlay: Playing =>
          if updatedPlays.allPlaysCompleted
          then
            val regatheredHands = updatedPlays.regather
            val resetPlays      = Plays(pone)
            scoreHands(scoredPlay.copy(hands = regatheredHands, plays = resetPlays)) match
              case scoredHands: Playing => newDiscarding(scoredHands.scores, pone, dealer)
              case finished: Finished   => finished
          else scoredPlay.copy(plays = updatedPlays.withNextPlayer(opponent(player)))
        case finished: Finished  => finished

  def pass(player: Player): Playing => Playing | Finished =
    case playing @ Playing(_, hands, dealer, pone, _, _, plays) =>
      def opponent(player: Player): Player = if player == dealer then pone else dealer

      require(Set(dealer, pone).contains(player), NonParticipatingPlayer(player))
      require(plays.nextPlayer == player, NotYourTurn(player))
      require(plays.mustPass(hands(player)), PlayTargetAttainable(player))

      val updatedPlays = plays.pass(player)

      scorePass(playing.copy(plays = updatedPlays)) match
        case scoredPlaying: Playing =>
          if updatedPlays.allPassed
          then scoredPlaying.copy(plays = updatedPlays.restarted)
          else scoredPlaying.copy(plays = updatedPlays.withNextPlayer(opponent(player)))
        case finished: Finished     => finished
