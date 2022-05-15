package com.nigeleke.cribbage.model

import Card.{Face, Suit}

import java.util.UUID
import scala.util.Random

type Deck = Seq[Card]

object Deck:
  // Must be def, not val, so new CardIds are generated...
  private[model] def fullSetOfCards: Seq[Card] = for
    face <- Face.values
    suit <- Suit.values
  yield Card(face, suit)

  val handsToDeal = Game.maxPlayers
  val cardsPerHandToDeal = 6

  val fullSize: Int = fullSetOfCards.size

  def shuffledDeck: Deck = Random.shuffle(fullSetOfCards)

  def deal: (Deck, Seq[Hand]) =
    val cards = shuffledDeck
    val hands = (1 to handsToDeal).map(i => cards.drop((handsToDeal - i) * cardsPerHandToDeal).take(cardsPerHandToDeal))
    val remainder = cards.drop(handsToDeal * cardsPerHandToDeal)
    (remainder, hands)

extension (deck: Deck)
  def cut: (Deck, Card) =
    val card = deck.head
    (deck.filterNot(_ == card), card)
