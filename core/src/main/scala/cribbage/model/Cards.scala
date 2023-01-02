package cribbage
package model

import scala.util.Random

/** Cards object manages sequence of cards behind the Crib, Deck and Hand types. */
object Cards:
  opaque type Crib = Seq[Card]
  opaque type Deck = Seq[Card]
  opaque type Hand = Seq[Card]

  /** Create a Crib, Deck or Hand from a sequence of cards.
    * @param cards
    *   The cards to be included in the resulant id.
    * @tparam T
    *   The required Crib, Deck or Hand type.
    * @return
    *   An instance of T containing the Cards.
    */
  private def cardsOf[T <: Crib | Deck | Hand](cards: Seq[Card]): T = cards.asInstanceOf[T]

  /** Create a Crib with the given cards.
    * @constructor
    * @param cards
    *   The cards.
    * @return
    *   The Crib.
    */
  def cribOf(cards: Seq[Card]): Crib = cardsOf[Crib](cards)

  /** An empty Crib. */
  val emptyCrib: Crib = Seq.empty

  /** Create a Deck with the given cards.
    * @constructor
    * @param cards
    *   The cards.
    * @return
    *   The Deck.
    */
  def deckOf(cards: Seq[Card]): Deck = cardsOf[Deck](cards)

  /** A full Deck of cards.
    */
  val fullDeck: Deck = (for
    face <- Card.faces
    suit <- Card.suits
  yield Card(face, suit)).toIndexedSeq

  /** @return
    *   A shuffled Deck of cards.
    * @note
    *   A different shuffled Deck is returned on each call.
    */
  def shuffledDeck: Deck = Random.shuffle(fullDeck)

  /** Create a Hand with the given cards.
    *
    * @constructor
    * @param cards
    *   The cards.
    * @return
    *   The Hand.
    */
  def handOf(cards: Seq[Card]): Hand = cardsOf[Hand](cards)

  /** An empty Hand. */
  val emptyHand: Hand = Seq.empty

  extension (cards: Crib | Deck | Hand)
    /** @return Return the number of cards contained in the collection. */
    def size: Int = cards.size

    /** @return Return the cards in a non-descriptive collection. */
    def toSeq: Seq[Card] = cards

  extension (crib: Crib)
    /** @return True if the correct number of cards have been discarded. */
    def isFull: Boolean = crib.size == CardsRequiredInCrib

    /** Add the provided cards to the Crib.
      * @return
      *   The new Crib including the new Cards.
      */
    def addAll(these: Seq[Card]): Crib = crib ++ these

  extension (hand: Hand)
    /** Check if the Hand contains the provided Card.
      *
      * @param card
      *   The Card.
      * @return
      *   True, if the Hand contains the Card, false otherwise.
      */
    def contains(card: Card): Boolean = hand.contains(card)

    /** Check if the Hand contains any Card the satisfies the rule predicate. */
    def containsAny(rule: Card => Boolean): Boolean = hand.filter(rule).nonEmpty

    /** Check if the Hand contains the provided Cards.
      *
      * @param these
      *   The Cards to be checked.
      * @return
      *   True, if the Hand contains the Card, false otherwise.
      */
    def containsAll(these: Seq[Card]): Boolean = these.forall(hand.contains)

    /** Remove the provided card from the Hand.
      * @param card
      *   The Card.
      * @return
      *   The updated Hand. If the Card is not present the original Hand will be returned.
      */
    def remove(card: Card): Hand = removeAll(Seq(card))

    /** Remove the provided cards from the Hand.
      * @param these
      *   The Cards to be removed.
      * @return
      *   The updated Hand. If the Card is not present the original Hand will be returned.
      */
    def removeAll(these: Seq[Card]): Hand = hand.filterNot(these.contains)

  extension (deck: Deck)

    /** Deal cards from the current Deck.
      * @param numberOfHands
      *   The number of hands to deal.
      * @param cardsPerHand
      *   The number of cards to deal in each hand.
      * @return
      *   A tuple of a) cards remaining from the Deck after the and b) a sequence of the Hands
      *   dealt.
      * @throws RuntimeException
      *   if not enough cards in the deck.
      */
    def deal(numberOfHands: Int, cardsPerHand: Int): (Deck, Seq[Hand]) =
      require(deck.size >= numberOfHands * cardsPerHand)
      def dealHand(n: Int): Hand =
        deck
          .drop((numberOfHands - n) * cardsPerHand)
          .take(cardsPerHand)
      val hands: Seq[Hand]       = (1 to numberOfHands).map(dealHand)
      val remainder: Deck        = deck.drop(numberOfHands * cardsPerHand)
      (remainder, hands)

    /** Cut a card from the Deck.
      * @return
      *   A tuple of a) cards remaining after the cut, and b) the cut card itself.
      * @throws RuntimeException
      *   if not enough cards in the deck.
      */
    def cut: (Deck, Card) =
      require(!deck.isEmpty)
      val card = deck.head
      (deck.filterNot(_ == card), card)
