package com.nigeleke.cribbage.model

import Card.{Face, Suit}

import java.util.UUID
import scala.util.Random

type Hand = Seq[Card]

object Hand:
  def apply(cards: Seq[Card] = Seq.empty): Hand = cards

extension (cards: Hand)
  def isEmpty: Boolean = cards.isEmpty
  def remove(axed: Seq[Card]): (Seq[Card], Seq[Card]) =
    val (removed, remaining) = cards.partition(card => axed.contains(card))
    (removed, remaining)
