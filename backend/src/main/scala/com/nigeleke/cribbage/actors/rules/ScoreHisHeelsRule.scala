package com.nigeleke.cribbage.actors.rules

import com.nigeleke.cribbage.actors.Game._

object ScoreHisHeelsRule extends Rule {

  override def commands(state: State): Seq[Command] = {
    val game = state.game
    // TODO:
    Seq.empty
  }

}
