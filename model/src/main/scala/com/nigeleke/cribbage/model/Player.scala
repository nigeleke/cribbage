package com.nigeleke.cribbage.model

import java.util.UUID

case class Player(id: Player.Id)

object Player {
  type Id = UUID
}
