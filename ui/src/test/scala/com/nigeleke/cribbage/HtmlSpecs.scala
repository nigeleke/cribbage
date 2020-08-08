package com.nigeleke.cribbage

import java.util.UUID

import com.nigeleke.cribbage.html.{ Html, RevealedCard }
import com.nigeleke.cribbage.model.{ Card, Face, Suit }
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

class HtmlSpecs extends AnyWordSpec with Matchers {

  "A RevealedCard" should {

    val id = UUID.randomUUID()
    val card = Card(id, Face.Ace, Suit.Spades)
    val revealedCard = RevealedCard(card)
    val html = Html.toHtmlString(revealedCard)

    "create a <span> with the card.id" in {
      println(html)
    }

  }

}
