package com.nigeleke.cribbage.actors.rules

import com.nigeleke.cribbage.actors.Game._

object CutForDealRule extends Rule {

  override def commands(state: State): Seq[Command] = {
    val game = state.game

    val needToCutForDeal = game.players.size == 2 && game.optDealer.isEmpty

    if (needToCutForDeal) Seq(CutForDeal)
    else Seq.empty
  }

}
