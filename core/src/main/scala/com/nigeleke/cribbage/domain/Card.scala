package com.nigeleke.cribbage.domain

import java.util.UUID

case class Card(id: Card.Id, face: Card.Face, suit: Card.Suit):
  import Card.Ansi
  override val toString = s"${suit.ansiColor}${face.smallFace}${suit.smallSuit}${Ansi.reset}"

extension (card: Card)
  def rank = card.face.rank
  def value = card.face.value

object Card:
  opaque type Id = UUID

  def apply(face: Face, suit: Suit): Card = Card(UUID.randomUUID(), face, suit)

  enum Face(val value: Int, val rank: Int, val smallFace: String):
    case Ace extends Face(1, 1, "A")
    case Two extends Face(2, 2, "2")
    case Three extends Face(3, 3, "3")
    case Four extends Face(4, 4, "4")
    case Five extends Face(5, 5, "5")
    case Six extends Face(6, 6, "6")
    case Seven extends Face(7, 7, "7")
    case Eight extends Face(8, 8, "8")
    case Nine extends Face(9, 9, "9")
    case Ten extends Face(10, 10, "T")
    case Jack extends Face(10, 11, "J")
    case Queen extends Face(10, 12, "Q")
    case King extends Face(10, 13, "K")

  enum Suit(val smallSuit: String, val ansiColor: String):
    case Clubs extends Suit(Ansi.club, Ansi.black)
    case Diamonds extends Suit(Ansi.diamond, Ansi.red)
    case Hearts extends Suit(Ansi.heart, Ansi.red)
    case Spades extends Suit(Ansi.spade, Ansi.black)

  object Ansi:
    private val escape = "\u001b"
    val red = escape + "[31m"
    val black = escape + "[30m"
    val reset = escape + "[0m"
    val club = "\u2663"
    val diamond = "\u2666"
    val heart = "\u2665"
    val spade = "\u2660"
