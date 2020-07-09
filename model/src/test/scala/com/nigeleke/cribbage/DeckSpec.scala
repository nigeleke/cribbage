package com.nigeleke.cribbage

import com.nigeleke.cribbage.model.Deck
import com.nigeleke.cribbage.suit.{Face, Suit}
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

class DeckSpec extends AnyWordSpec with Matchers {

  val allCards = (for {
    face <- Face.values()
    suit <- Suit.values()
  } yield (face, suit)).toSeq

  "A Deck" should {

    "contain 52 cards" in {
      Deck().cards.size should be(52)
    }

    "contain all cards for all Suits and Faces" when {

      "pristine" in {
        Deck().cards.map(card => (card.face, card.suit)) should contain theSameElementsInOrderAs(allCards)
      }

      "shuffled" in {
        val deck = Deck()
        deck.shuffled.cards should contain theSameElementsAs(deck.cards)
      }

    }

  }

}
