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

/** A player's Score. Traditionally scored by a back peg leaping a front peg by the appropriate
  * number of points.
  * @param back
  *   The back peg.
  * @param front
  *   The front peg, i.e. the current points.
  */
final case class Score(back: Int, front: Int)

object Score:

  /** @constructor
    * @return
    *   The initial score.
    */
  val zero = Score(0, 0)

  extension (score: Score)

    /** @return The current points. */
    def points: Int = score.front

    /** Add points to the current score. If adding zero points the front & back remain as-was.
      * @param points
      *   The points to add.
      * @return
      *   The updated score, with the previous front peg now in the back.
      */
    def add(points: Int): Score =
      if points != 0
      then score.copy(back = score.front, front = score.front + points)
      else score

    def isWinningScore = score.points >= WinningScore
