package com.nigeleke.cribbage.model

import scala.util.Random

type Deck = Seq[Card]

object Deck:
  val fullDeck: Deck = (for
    face <- Card.faces
    suit <- Card.suits
  yield Card(face, suit)).toIndexedSeq

  def shuffledDeck: Deck = Random.shuffle(fullDeck)

extension (deck: Deck)
  def deal(numberOfHands: Int, cardsPerHand: Int): (Deck, Seq[Hand]) =
    require(deck.size >= numberOfHands * cardsPerHand)
    def dealHand(n: Int): Hand =
      deck
        .drop((numberOfHands - n) * cardsPerHand)
        .take(cardsPerHand)
    val hands: Seq[Hand]       = (1 to numberOfHands).map(dealHand)
    val remainder: Deck        = deck.drop(numberOfHands * cardsPerHand)
    (remainder, hands)

  def cut: (Deck, Card) =
    val card = deck.head
    (deck.filterNot(_ == card), card)
