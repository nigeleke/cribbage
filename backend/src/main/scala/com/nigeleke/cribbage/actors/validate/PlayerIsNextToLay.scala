package com.nigeleke.cribbage.actors.validate

import com.nigeleke.cribbage.model.Player.{Id => PlayerId}
import com.nigeleke.cribbage.model._

case class PlayerIsNextToLay(id: PlayerId, game: Status) extends Validation {

  def validate =
    if (id == game.play.optNextToLay.get) None
    else Option(s"Player $id's opponent is the next player to lay a card")

}
