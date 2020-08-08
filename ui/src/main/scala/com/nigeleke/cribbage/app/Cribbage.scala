package com.nigeleke.cribbage.app

import com.nigeleke.cribbage.html.RevealedCard
import com.nigeleke.cribbage.model.Deck
import org.scalajs.dom.document
import org.scalajs.dom.html.Div

object Cribbage {

  def main(args: Array[String]): Unit = {
    val cardsDiv = document.getElementById("cards").asInstanceOf[Div]
    val deck = Deck()
    val revealedDeck = deck.map(RevealedCard(_))
    cardsDiv.innerHTML = revealedDeck.mkString("")
    println(cardsDiv)
  }

}
