package com.nigeleke.cribbage.model

import java.util.UUID

final case class Game(id: Game.Id)

object Game {
  type Id = UUID
}
