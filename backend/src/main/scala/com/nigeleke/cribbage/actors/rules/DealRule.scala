package com.nigeleke.cribbage.actors.rules

import com.nigeleke.cribbage.actors.Game._

object DealRule extends Rule {

  override def commands(state: State): Seq[Command] = {
    require(state.isInstanceOf[Starting])

    val game = state.game
    val dealRequired = game.optDealer.isDefined && game.hands.isEmpty

    if (dealRequired) Seq(DealHands)
    else Seq.empty
  }

}
