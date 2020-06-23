package com.nigeleke.cribbage.model

import java.util.UUID

import com.nigeleke.cribbage.suit.{Face, Suit}

final case class Card(id: Card.Id, face: Face, suit: Suit) {
  lazy val rank = face.rank
  lazy val value = face.value

  override lazy val toString = s"$face of $suit"
}

object Card {
  type Id = UUID
}
