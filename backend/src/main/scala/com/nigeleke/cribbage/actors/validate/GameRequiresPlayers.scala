package com.nigeleke.cribbage.actors.validate

import com.nigeleke.cribbage.model._

case class GameRequiresPlayers(game: Status) extends Validation {

  def validate =
    if (game.players.size < 2) None
    else Option(s"Game ${game.id} has enough players")

}
