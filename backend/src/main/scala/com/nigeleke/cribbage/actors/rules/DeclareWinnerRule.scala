package com.nigeleke.cribbage.actors.rules

import com.nigeleke.cribbage.actors.Game._

object DeclareWinnerRule extends Rule {

  override def commands(state: State): Seq[Command] = {
    val game = state.game
    (for {
      (playerId, score) <- game.scores if score.front >= 121
    } yield DeclareWinner(playerId)).toSeq
  }

}
