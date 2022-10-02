package com.nigeleke.cribbage.model

case class Score(back: Int, front: Int) {
  import Score.*
  override def toString: String = s"[$back ${Ansi.arrowRight} $front]"
}

extension (score: Score)

  def points: Int = score.front

  def add(points: Int): Score =
    if points != 0
    then score.copy(back = score.front, front = score.front + points)
    else score

object Score:

  val zero = Score(0, 0)

  object Ansi:
    val arrowRight = "\u00bb"