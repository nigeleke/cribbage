package com.nigeleke.cribbage.model

import java.util.UUID

case class Player(id: Player.Id, name: String)

object Player {
  type Id = UUID
}
