package com.nigeleke.cribbage.model

import java.util.UUID

final case class Card(id: Card.Id)

object Card {
  type Id = UUID
}
