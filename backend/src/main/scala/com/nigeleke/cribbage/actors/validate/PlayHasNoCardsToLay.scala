package com.nigeleke.cribbage.actors.validate

import com.nigeleke.cribbage.model.Player.{Id => PlayerId}
import com.nigeleke.cribbage.model._

case class PlayHasNoCardsToLay(id: PlayerId, game: Status) extends Validation {

  def validate = {
    val runningTotal = game.play.runningTotal
    val playableCards = game.hands(id).filter(runningTotal + _.value <= 31)
    if (playableCards.isEmpty) None
    else Some(s"Player $id cannot pass; they hold cards that can be laid")
  }

}
