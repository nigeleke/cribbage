package com.nigeleke.cribbage.model

trait Points
case class PlayPoints(pairs: Int = 0, fifteens: Int = 0, runs: Int = 0) extends Points
case class EndPlayPoints(endOfPlay: Int = 0) extends Points
case class CardsPoints(pairs: Int = 0, fifteens: Int = 0, runs: Int = 0, flushes: Int = 0, heels: Int = 0) extends Points

extension (points: Points)
  def total: Int = points match
    case PlayPoints(pairs, fifteens, runs)                  => pairs + fifteens + runs
    case EndPlayPoints(endOfPlay)                           => endOfPlay
    case CardsPoints(pairs, fifteens, runs, flushes, heels) => pairs + fifteens + runs + flushes + heels
