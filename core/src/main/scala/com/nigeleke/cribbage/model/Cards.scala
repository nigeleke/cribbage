package com.nigeleke.cribbage.model

import Card.{Face, Suit}
import java.util.UUID
import scala.util.Random

opaque type Crib = Seq[Card]

object Crib:
  def apply(): Crib = Seq.empty[Card]
  implicit def apply(cards: Seq[Card]): Crib = cards

extension (cards: Crib)
  def ++(otherCards: Seq[Card]): Seq[Card] = cards ++ otherCards
  def hasAllDiscards: Boolean = cards.size == 4

opaque type Deck = Seq[Card]

object Deck:
  val fullSize: Int = Cards.fullSetOfCards.size
  def shuffledDeck: Deck = Random.shuffle(Cards.fullSetOfCards)
  def deal: (Deck, Seq[Hand]) =
    val handsToDeal = Game.maxPlayers
    val cardsPerHandToDeal = 6
    val cards = shuffledDeck
    val hands = (1 to handsToDeal).map(i => cards.drop((handsToDeal - i) * cardsPerHandToDeal).take(cardsPerHandToDeal))
    val remainder = cards.drop(handsToDeal * cardsPerHandToDeal)
    (remainder, hands)

extension (deck: Deck) def headOption: Option[Card] = deck.headOption

opaque type Hand = Seq[Card]

object Hand:
  def apply(): Hand = Seq.empty[Card]
  implicit def apply(cards: Seq[Card]): Hand = cards

extension (cards: Hand)
  def isEmpty: Boolean = cards.isEmpty
  def remove(axed: Seq[Card]): (Seq[Card], Seq[Card]) =
    val (removed, remaining) = cards.partition(card => axed.contains(card))
    (removed, remaining)

extension (cards: Deck | Crib | Hand)
  def toCardSeq: Seq[Card] = cards
  def cardValues: Seq[Int] = cards.map(_.face.value)

extension (cards: Seq[Card]) implicit def toString(): String = cards.mkString(" ")

object Cards:
  // Must be def, not val, so new CardIds are generated...
  private[model] def fullSetOfCards: Seq[Card] = for
    face <- Face.values
    suit <- Suit.values
  yield Card(face, suit)
