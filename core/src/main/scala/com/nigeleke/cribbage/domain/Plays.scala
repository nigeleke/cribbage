package com.nigeleke.cribbage.domain

case class Plays(nextToPlay: Player.Id, current: Seq[Plays.Play], passedPlayers: Set[Player.Id], previous: Seq[Plays.Play])

object Plays:
  def apply(player: Player.Id): Plays = new Plays(nextToPlay = player, Seq.empty, Set.empty, Seq.empty)

  sealed trait Play
  case class Laid(playerId: Player.Id, card: Card) extends Play
  case class Pass(playerId: Player.Id) extends Play

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
  private def firstPassed: Player.Id = plays.current.collect { case Plays.Pass(id) => id }.head
  def laidSoFar: Seq[Plays.Laid] = (plays.current ++ plays.previous).collect { case play @ Plays.Laid(_, _) => play }
