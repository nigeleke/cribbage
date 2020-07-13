package com.nigeleke.cribbage.actors.rules

import com.nigeleke.cribbage.actors.Game._
import com.nigeleke.cribbage.suit.Face

object ScoreCutAtStartOfPlayRule extends Rule {

  override def commands(state: State): Seq[Command] = {
    val game = state.game
    (for {
      dealer <- game.optDealer
      cut <- game.optCut if cut.face == Face.Jack
    } yield PegScore(dealer, 2)).toSeq
  }

}
