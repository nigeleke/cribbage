package com.nigeleke.cribbage.model

type Play = Plays.Play

final case class Plays(optNextToPlay: Option[PlayerId], current: Seq[Play], passedPlayers: Set[PlayerId], previous: Seq[Play])

object Plays:
  def apply(): Plays = new Plays(None, Seq.empty, Set.empty, Seq.empty)
  def apply(optFirstPlayerId: Option[PlayerId]): Plays = new Plays(optFirstPlayerId, Seq.empty, Set.empty, Seq.empty)

  enum Play:
    case Laid(playerId: PlayerId, card: Card)
    case Pass(playerId: PlayerId)

extension (plays: Plays)
  def runningTotal =
    plays.current.map { play =>
      play match
        case Plays.Play.Laid(_, card) => card.face.value
        case Plays.Play.Pass(_)       => 0
    }.sum
  def passCount = plays.passedPlayers.size
  def nextPlay = plays.copy(
    optNextToPlay = plays.firstPassed,
    current = Seq.empty,
    passedPlayers = Set.empty,
    previous = plays.previous ++ plays.current
  )
  private def firstPassed: Option[PlayerId] = plays.current.collect { case Plays.Play.Pass(id) => id }.headOption
  def laidSoFar: Seq[Plays.Play.Laid] = (plays.current ++ plays.previous).collect { case play @ Plays.Play.Laid(_, _) => play }
