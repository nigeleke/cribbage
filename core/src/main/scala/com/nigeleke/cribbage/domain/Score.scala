package com.nigeleke.cribbage.domain

case class Score(back: Int, front: Int):
  def points: Int = front

  def add(points: Int): Score =
    if points != 0
    then copy(back = front, front = front + points)
    else this

object Score:
  val zero = Score(0, 0)
