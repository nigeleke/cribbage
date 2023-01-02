package cribbage
package model

/** A playing card.
  *
  * @param face
  *   The card's face.
  * @param suit
  *   The card's suit.
  */
final case class Card(face: Card.Face, suit: Card.Suit)

object Card:
  /** A Card.Face, providing the face id, the run & cut ranking, and a short string for printing.
    */
  enum Face(val value: Int, val rank: Int):
    case Ace   extends Face(1, 1)
    case Two   extends Face(2, 2)
    case Three extends Face(3, 3)
    case Four  extends Face(4, 4)
    case Five  extends Face(5, 5)
    case Six   extends Face(6, 6)
    case Seven extends Face(7, 7)
    case Eight extends Face(8, 8)
    case Nine  extends Face(9, 9)
    case Ten   extends Face(10, 10)
    case Jack  extends Face(10, 11)
    case Queen extends Face(10, 12)
    case King  extends Face(10, 13)

  /** A Card.Suit providing a symbolic string and suit colour for printing.
    */
  enum Suit:
    case Clubs, Diamonds, Hearts, Spades

  /** Expose the faces as a collection. */
  val faces = Face.values

  /** Expose the suits as a collection. */
  val suits = Suit.values

  extension (card: Card)
    /** Expose the card's rank. */
    def rank = card.face.rank

    /** Expose the card's face id. */
    def value = card.face.value
