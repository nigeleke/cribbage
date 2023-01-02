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
package model

import Cards.*

/** The plays being made in a Playing game.
  * @param nextPlayer
  *   The next player to play or pass.
  * @param inPlay
  *   The cards in the current play.
  * @param played
  *   The cards previously played.
  */
final case class Plays(
    nextPlayer: Player,
    inPlay: Seq[Plays.Play],
    played: Seq[Plays.Play]
)

object Plays:
  /** Create a fresh plays.
    * @param player
    *   The first player to play.
    * @return
    *   The plays.
    */
  def apply(player: Player): Plays =
    new Plays(nextPlayer = player, Seq.empty, Seq.empty)

  /** A single play, recording the player and their card laid or their pass.
    */
  sealed trait Play
  final case class Laid(player: Player, card: Card) extends Play
  final case class Pass(player: Player)             extends Play

  extension (plays: Plays)
    /** Running total for the current play. Max 31. */
    def runningTotal =
      plays.inPlay.map { play =>
        play match
          case Plays.Laid(_, card) => card.value
          case Plays.Pass(_)       => 0
      }.sum

    def mustPass(hand: Hand) =
      val currentTotal = plays.runningTotal
      !hand.containsAny(_.value + currentTotal <= PlayTarget)

    /** The set of players that have Passed in the current play. */
    private def passedPlayers = plays.inPlay
      .collect { case p: Plays.Pass => p }
      .map(_.player)
      .toSet

    /** @return
      *   The number of players that have Passed in the current play.
      */
    private def passCount = plays.passedPlayers.size

    def allPassed: Boolean = plays.passCount == NumberOfPlayersInGame

    def play(player: Player, card: Card): Plays =
      val updatedInPlay = plays.inPlay :+ Plays.Laid(player, card)
      plays.copy(inPlay = updatedInPlay)

    def pass(player: Player): Plays =
      val updatedInPlay = plays.inPlay :+ Plays.Pass(player)
      plays.copy(inPlay = updatedInPlay)

    def allPlaysCompleted: Boolean = allLaid.size == AllLaidCardCount

    private def allLaid: Seq[Plays.Laid] = (plays.inPlay ++ plays.played).collect {
      case play: Plays.Laid => play
    }

    def withNextPlayer(player: Player): Plays =
      plays.copy(nextPlayer = player)

    def restarted: Plays =
      val updatedPlayed = plays.played ++ plays.inPlay
      plays.copy(inPlay = Seq.empty, played = updatedPlayed)

    /** @return
      *   The current laid cards, without the Pass detail.
      */
    def cardsInPlay: Seq[Card] = plays.inPlay.collect { case Plays.Laid(_, card) => card }

    def regather: Map[Player, Hand] =
      allLaid.groupMap(_.player)(_.card).map((p, cs) => (p, Cards.handOf(cs)))
