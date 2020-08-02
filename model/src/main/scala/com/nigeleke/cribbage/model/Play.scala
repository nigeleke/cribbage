/*
 * Copyright (C) 2020  Nigel Eke
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

package com.nigeleke.cribbage.model

import Player.{Id => PlayerId}

final case class Play(optNextToLay: Option[PlayerId],
                      current: Lays,
                      passCount: Int,
                      previous: Seq[Lays])

object Play {

  def apply() : Play = new Play(None, Seq.empty, 0, Seq.empty)

  implicit class PlayOps(play: Play) {
    def withLay(lay: Lay) : Play = play.copy(current = play.current :+ lay, passCount = 0)
    def withPass() : Play = play.copy(passCount = play.passCount + 1)
    def withNextToLay(playerId: PlayerId) : Play = play.copy(optNextToLay = Some(playerId))
    def withNextPlay() : Play = play.copy(current = Seq.empty, passCount = 0, previous = play.previous :+ play.current)
    lazy val runningTotal : Int = play.current.map(_.card.value).sum
  }

}
