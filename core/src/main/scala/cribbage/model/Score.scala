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
