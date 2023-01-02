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

package object cribbage {

  /** Cribbage can be a two, three or (at a push) four player game. This implementation is for two
    * players.
    */
  val NumberOfPlayersInGame = 2

  /** Six [Card]s are dealt to each Player by the current dealer. */
  val CardsDealtPerHand = 6

  /** [Player]s discard two cards each into the [Crib] leaving four for [Score]ing and [Plays]. */
  val CardsKeptPerHand = 4

  /** Each [Player] discarding two [Card]s to the [Crib] will mean four [Card]s end up there. */
  val CardsRequiredInCrib = (CardsDealtPerHand - CardsKeptPerHand) * NumberOfPlayersInGame

  /** Each [Plays.Play] cannot have a running total of more than 31. */
  val PlayTarget = 31

  /** When all [Player]s have [Laid] all [Cards] the [Score]ing can be performed. */
  val AllLaidCardCount = CardsKeptPerHand * NumberOfPlayersInGame

  /** Short games can play to 61, but it is normal to play to 121, as in this implementation. */
  val WinningScore = 121
}
