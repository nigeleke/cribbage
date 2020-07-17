package com.nigeleke.cribbage.model

import java.util.UUID

import com.nigeleke.cribbage.model.Card.{Id => CardId}
import com.nigeleke.cribbage.suit.{Face, Suit}

import scala.util.Random
import scala.language.implicitConversions

object Deck {

  def apply() : Deck = fullSetOfCards

  private def fullSetOfCards = (for {
    face <- Face.values()
    suit <- Suit.values()
  } yield Card(UUID.randomUUID(), face, suit)).toSeq

  implicit class DeckOps(deck: Deck) {
    val shuffled = Random.shuffle(deck)
  }

}
