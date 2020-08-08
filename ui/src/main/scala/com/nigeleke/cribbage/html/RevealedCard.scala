package com.nigeleke.cribbage.html

import com.nigeleke.cribbage.model.Card
import com.nigeleke.cribbage.model.Face._
import com.nigeleke.cribbage.model.Suit._
import org.scalajs.dom.html.Element
import scalatags.JsDom
import scalatags.JsDom.all._

case class RevealedCard(card: Card) extends Html {
  import RevealedCard._
  override val html: JsDom.TypedTag[Element] = {
    span(id := s"card-${card.id}")
    //    img(src := { cardToImageFilename(card) }, alt := s"${card.face} of ${card.suit}))
  }
}

object RevealedCard {

  private val faceToString = Map(
    Ace -> "1", Two -> "2", Three -> "3", Four -> "4",
    Five -> "5", Six -> "6", Seven -> "7", Eight -> "8",
    Nine -> "9", Ten -> "T", Jack -> "J", Queen -> "Q",
    King -> "K")

  private val suitToString = Map(
    Hearts -> "H", Clubs -> "C", Diamonds -> "D", Spades -> "S")

  def cardToImageFilename(card: Card) = s"cards/${faceToString(card.face)}${suitToString(card.suit)}.png"

}