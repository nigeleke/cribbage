package com.nigeleke.cribbage.model

import java.util.UUID

import com.nigeleke.cribbage.suit.{Face, Suit}

import scala.util.Random
import scala.language.implicitConversions

case class Deck(cards: Seq[Card])

object Deck {

  def apply() : Deck = Deck(cards)

  def shuffled() : Deck = Deck(Random.shuffle(cards))

  private lazy val cards = (for {
    face <- Face.values()
    suit <- Suit.values()
  } yield Card(UUID.randomUUID, face, suit)).toSeq

  implicit def deckToSeqCard(deck: Deck) : Seq[Card] = deck.cards

}
