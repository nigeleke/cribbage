package com.nigeleke.cribbage.model

/** The points scored for different phases of the game. */
sealed trait Points

object Points:
  /** @param heals
    *   Points scored for the dealer cut. 2 for his heels, or zero.
    */
  case class Cut(heals: Int) extends Points

  /** Points scored during a play.
    * @param pairs
    *   2 for a pair, 6 for three, 12 for quad.
    * @param fifteens
    *   2 for running total of 15.
    * @param runs
    *   n for a single run of n.
    */
  case class Play(pairs: Int = 0, fifteens: Int = 0, runs: Int = 0) extends Points

  /** Points scored at end of play.
    * @param endOfPlay
    *   2, if running total is 31, otherwise 1.
    */
  case class EndPlay(endOfPlay: Int = 0) extends Points

  /** Points scored at end of all plays.
    * @param pairs
    *   2 for a pair, 6 for three, 12 for quad.
    * @param fifteens
    *   2 for card combinations totalling 15.
    * @param runs
    *   n for each run of n.
    * @param flushes
    *   4 for a hand of the same suit (+1 including cut)
    * @param heels
    *   1 if holding Jack in same suit as cut.
    */
  case class Cards(
      pairs: Int = 0,
      fifteens: Int = 0,
      runs: Int = 0,
      flushes: Int = 0,
      heels: Int = 0
  ) extends Points

extension (points: Points)
  /** @return
    *   The total of the points breakdown.
    */
  def total: Int = points match
    case Points.Cut(heals)                                   => heals
    case Points.Play(pairs, fifteens, runs)                  => pairs + fifteens + runs
    case Points.EndPlay(endOfPlay)                           => endOfPlay
    case Points.Cards(pairs, fifteens, runs, flushes, heels) =>
      pairs + fifteens + runs + flushes + heels
