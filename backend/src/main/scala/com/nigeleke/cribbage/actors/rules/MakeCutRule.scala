package com.nigeleke.cribbage.actors.rules

import com.nigeleke.cribbage.actors.Game._

object MakeCutRule extends Rule {

  override def commands(state: State): Seq[Command] = {
    val game = state.game
    val needToCut = game.crib.size == 4

    if (needToCut) Seq(MakeCut)
    else Seq.empty
  }

}
