package com.nigeleke.cribbage.actors.validate

import com.nigeleke.cribbage.model.Player.{Id => PlayerId}
import com.nigeleke.cribbage.model._

case class PlayerNotAlreadyJoinedGame(id: PlayerId, game: Status) extends Validation {

  def validate =
    if (!game.players.contains(id)) None
    else Some(s"Player ${id} already joined game ${game.id}")

}
