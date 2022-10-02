package com.nigeleke.cribbage.model

trait Points

object Points:
  case class Cut(heals: Int)                                        extends Points
  case class Play(pairs: Int = 0, fifteens: Int = 0, runs: Int = 0) extends Points
  case class EndPlay(endOfPlay: Int = 0)                            extends Points
  case class Cards(
      pairs: Int = 0,
      fifteens: Int = 0,
      runs: Int = 0,
      flushes: Int = 0,
      heels: Int = 0
  ) extends Points

extension (points: Points)
  def total: Int = points match
    case Points.Cut(heals)                                   => heals
    case Points.Play(pairs, fifteens, runs)                  => pairs + fifteens + runs
    case Points.EndPlay(endOfPlay)                           => endOfPlay
    case Points.Cards(pairs, fifteens, runs, flushes, heels) =>
      pairs + fifteens + runs + flushes + heels
