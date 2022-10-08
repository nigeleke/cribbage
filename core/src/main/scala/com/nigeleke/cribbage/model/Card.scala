package com.nigeleke.cribbage.model

/** A playing card.
  * @param face
  *   The card's face.
  * @param suit
  *   The card's suit.
  */
final case class Card(face: Card.Face, suit: Card.Suit):
  import Card.Ansi.*
  lazy val toPrettyString = s"${suit.ansiColor}${face.toPrettyString}${suit.toPrettyString}$reset"

object Card:
  /** A Card.Face, providing the face id, the run & cut ranking, and a short string for printing.
    */
  enum Face(val value: Int, val rank: Int, val smallFace: String):
    case Ace   extends Face(1, 1, "A")
    case Two   extends Face(2, 2, "2")
    case Three extends Face(3, 3, "3")
    case Four  extends Face(4, 4, "4")
    case Five  extends Face(5, 5, "5")
    case Six   extends Face(6, 6, "6")
    case Seven extends Face(7, 7, "7")
    case Eight extends Face(8, 8, "8")
    case Nine  extends Face(9, 9, "9")
    case Ten   extends Face(10, 10, "T")
    case Jack  extends Face(10, 11, "J")
    case Queen extends Face(10, 12, "Q")
    case King  extends Face(10, 13, "K")
    val toPrettyString = smallFace

  /** A Card.Suit providing a symbolic string and suit colour for printing.
    */
  enum Suit(val smallSuit: String, val ansiColor: String):
    case Clubs    extends Suit(Ansi.club, Ansi.black)
    case Diamonds extends Suit(Ansi.diamond, Ansi.red)
    case Hearts   extends Suit(Ansi.heart, Ansi.red)
    case Spades   extends Suit(Ansi.spade, Ansi.black)
    val toPrettyString = smallSuit

  /** Expose the faces as a collection. */
  val faces = Face.values

  /** Expose the suits as a collection. */
  val suits = Suit.values

  extension (card: Card)
    /** Expose the card's rank. */
    def rank = card.face.rank

    /** Expose the card's face id. */
    def value = card.face.value

  /** ANSI escape codes for pretty printing to console. */
  object Ansi:
    private val escape = "\u001b"
    val red            = s"$escape[31m"
    val black          = s"$escape[30m"
    val reset          = s"$escape[0m"
    val club           = "\u2663"
    val diamond        = "\u2666"
    val heart          = "\u2665"
    val spade          = "\u2660"
