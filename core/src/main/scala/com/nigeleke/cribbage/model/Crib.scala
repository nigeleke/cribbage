package com.nigeleke.cribbage.model

import Card.{Face, Suit}

import java.util.UUID
import scala.util.Random

type Crib = Seq[Card]

object Crib:
  val expectedDiscardCount = 4

  def apply(cards: Seq[Card] = Seq.empty): Crib = cards

extension (cards: Crib) def hasAllDiscards: Boolean = cards.size == Crib.expectedDiscardCount
