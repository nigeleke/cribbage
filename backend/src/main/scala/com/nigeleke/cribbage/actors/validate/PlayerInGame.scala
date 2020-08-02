package com.nigeleke.cribbage.actors.validate

import com.nigeleke.cribbage.model.Player.{Id => PlayerId}
import com.nigeleke.cribbage.model._

case class PlayerInGame(id: PlayerId, game: Status) extends Validation {

  def validate =
    if (game.players.contains(id)) None
    else Option(s"Player $id is not a member of game ${game.id}")

}
