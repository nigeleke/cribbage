package com.nigeleke.cribbage.model

type Play = Plays.Play

case class Plays(nextToPlay: PlayerId, current: Seq[Play], passedPlayers: Set[PlayerId], previous: Seq[Play])

object Plays:
  def apply(player: PlayerId): Plays = new Plays(nextToPlay = player, Seq.empty, Set.empty, Seq.empty)

  sealed trait Play
  case class Laid(playerId: PlayerId, card: Card) extends Play
  case class Pass(playerId: PlayerId) extends Play

extension (plays: Plays)
  def runningTotal =
    plays.current.map { play =>
      play match
        case Plays.Laid(_, card) => card.face.value
        case Plays.Pass(_)       => 0
    }.sum
  def passCount = plays.passedPlayers.size
  def nextPlay = plays.copy(
    nextToPlay = plays.firstPassed,
    current = Seq.empty,
    passedPlayers = Set.empty,
    previous = plays.previous ++ plays.current
  )
  private def firstPassed: PlayerId = plays.current.collect { case Plays.Pass(id) => id }.head
  def laidSoFar: Seq[Plays.Laid] = (plays.current ++ plays.previous).collect { case play @ Plays.Laid(_, _) => play }
