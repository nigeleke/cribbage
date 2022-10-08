package com.nigeleke.cribbage.model

case class Plays(
    nextPlayer: Player,
    inPlay: Seq[Plays.Play],
    played: Seq[Plays.Play]
):
  lazy val toPrettyString: String =
    val sInPlay = inPlay.mkString("[", ", ", "]")
    val sPlayed = played.mkString("[", ", ", "]")
    s"Plays( nextPlayer: $nextPlayer  inPlay: $sInPlay  played: $sPlayed )"

object Plays:
  def apply(player: Player): Plays =
    new Plays(nextPlayer = player, Seq.empty, Seq.empty)

  sealed trait Play
  case class Laid(player: Player, card: Card) extends Play
  case class Pass(player: Player)             extends Play

extension (plays: Plays)
  def runningTotal               =
    plays.inPlay.map { play =>
      play match
        case Plays.Laid(_, card) => card.face.value
        case Plays.Pass(_)       => 0
    }.sum
  def passedPlayers              = plays.inPlay
    .collect { case p: Plays.Pass => p }
    .map(_.player)
    .toSet
  def passCount                  = plays.passedPlayers.size
  def laidSoFar: Seq[Plays.Laid] = (plays.inPlay ++ plays.played).collect {
    case play @ Plays.Laid(_, _) => play
  }
