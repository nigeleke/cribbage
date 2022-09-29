package com.nigeleke.cribbage.domain

import java.util.UUID
import scala.util.Random

type Hand = Seq[Card]

object Hand:
  val undealt: Hand = Seq.empty

extension (cards: Hand)
  def remove(axed: Seq[Card]): (Seq[Card], Seq[Card]) =
    val (removed, remaining) = cards.partition(card => axed.contains(card))
    (removed, remaining)
  def values: Seq[Int] = cards.map(_.face.value)
