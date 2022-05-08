package com.nigeleke.cribbage

import model.*
import Card.*

import org.scalatest.*
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

class DeckSpec extends AnyWordSpec with Matchers {

  "A Full Deck" should {

    val fullDeck = Deck.shuffledDeck

    "contain 52 Cards" in {
      fullDeck.size should be(52)
    }

    "contain all Cards for all Suits and Faces" in {
      val allCards = (for {
        face <- Face.values
        suit <- Suit.values
      } yield (face, suit)).toSeq

      fullDeck.toCardSeq.map(card => (card.face, card.suit)) should contain theSameElementsAs (allCards)
    }

  }

}
