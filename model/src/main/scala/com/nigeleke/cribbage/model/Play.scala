package com.nigeleke.cribbage.model

import Player.{Id => PlayerId}

final case class Play(optNextToLay: Option[PlayerId],
                      current: Lays,
                      previous: Seq[Lays])

object Play {
  def apply() : Play = new Play(None, Seq.empty, Seq.empty)

  implicit class PlayOps(play: Play) {
    def withLay(lay: Lay) : Play = play.copy(current = play.current :+ lay)
    def withNextToLay(playerId: PlayerId) : Play = play.copy(optNextToLay = Some(playerId))
  }
}
