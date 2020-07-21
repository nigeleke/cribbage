package com.nigeleke.cribbage.model

import Player.{Id => PlayerId}

final case class Play(optNextToLay: Option[PlayerId],
                      current: Lays,
                      passCount: Int,
                      previous: Seq[Lays])

object Play {

  def apply() : Play = new Play(None, Seq.empty, 0, Seq.empty)

  implicit class PlayOps(play: Play) {
    def withLay(lay: Lay) : Play = play.copy(current = play.current :+ lay)
    def withPass() : Play = play.copy(passCount = play.passCount + 1)
    def withNextToLay(playerId: PlayerId) : Play = play.copy(optNextToLay = Some(playerId))
    lazy val runningTotal : Int = play.current.map(_.card.value).sum
  }

}
