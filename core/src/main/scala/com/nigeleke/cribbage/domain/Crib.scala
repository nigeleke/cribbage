package com.nigeleke.cribbage.domain

import java.util.UUID
import scala.util.Random

type Crib = Seq[Card]

object Crib:
  val expectedDiscardCount = 4
  val empty: Crib = Seq.empty

extension (cards: Crib) def hasAllDiscards: Boolean = cards.size == Crib.expectedDiscardCount
