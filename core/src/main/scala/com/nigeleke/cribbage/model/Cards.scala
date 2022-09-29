package com.nigeleke.cribbage.model

import java.util.UUID
import scala.util.Random

type Cards = Seq[Card]

extension (cards: Deck | Crib | Hand)
  def asCards: Cards = cards
  def cardValues: Seq[Int] = cards.map(_.face.value)

extension (cards: Cards) implicit def toString(): String = cards.mkString(" ")
