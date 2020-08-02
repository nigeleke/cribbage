package com.nigeleke.cribbage.model

import java.util.UUID

import com.nigeleke.cribbage.ansi._

final case class Card(id: Card.Id, face: Face, suit: Suit) {
  val rank: Int = face.rank
  val value: Int = face.value

  private val color = if (Seq(Suit.Diamonds, Suit.Hearts).contains(suit)) red else black
  override val toString = s"$color${face.smallFace}${suit.smallSuit}$reset"
}

object Card {
  type Id = UUID
}
