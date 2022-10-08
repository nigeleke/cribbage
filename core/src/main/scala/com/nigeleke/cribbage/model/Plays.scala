package com.nigeleke.cribbage.model

/** The plays being made in a Playing game.
  * @param nextPlayer
  *   The next player to play or pass.
  * @param inPlay
  *   The cards in the current play.
  * @param played
  *   The cards previously played.
  */
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
  /** Create a fresh plays.
    * @param player
    *   The first player to play.
    * @return
    *   The plays.
    */
  def apply(player: Player): Plays =
    new Plays(nextPlayer = player, Seq.empty, Seq.empty)

  /** A single play, recording the player and their card laid or their pass.
    */
  sealed trait Play
  case class Laid(player: Player, card: Card) extends Play
  case class Pass(player: Player)             extends Play

extension (plays: Plays)

  /** Running total for the current play. Max 31. */
  def runningTotal =
    plays.inPlay.map { play =>
      play match
        case Plays.Laid(_, card) => card.face.value
        case Plays.Pass(_)       => 0
    }.sum

  /** The set of players that have Passed in the current play. */
  def passedPlayers = plays.inPlay
    .collect { case p: Plays.Pass => p }
    .map(_.player)
    .toSet

  /** @return
    *   The number of players that have Passed in the current play.
    */
  def passCount = plays.passedPlayers.size

  /** @return
    *   The current laid cards, without the Pass detail.
    */
  def laidSoFar: Seq[Plays.Laid] = (plays.inPlay ++ plays.played).collect {
    case play @ Plays.Laid(_, _) => play
  }
