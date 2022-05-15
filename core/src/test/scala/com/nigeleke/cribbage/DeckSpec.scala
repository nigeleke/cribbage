package com.nigeleke.cribbage

import model.*
import Card.*

import org.scalatest.*
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

class DeckSpec extends AnyWordSpec with Matchers {

  val fullDeck = Deck.shuffledDeck

  "A Full Deck" should {

    "contain 52 Cards" in {
      fullDeck.size should be(52)
    }

    "contain all Cards for all Suits and Faces" in {
      val allCards = (for {
        face <- Face.values
        suit <- Suit.values
      } yield (face, suit)).toSeq

      fullDeck.map(card => (card.face, card.suit)) should contain theSameElementsAs (allCards)
    }

  }

  "A Deck" should {
    "allow a random card to be selected" in {
      val (remaining, cut) = fullDeck.cut
      fullDeck should contain(cut)
      remaining should not contain (cut)
      remaining.size should be(fullDeck.size - 1)
    }
  }

}
