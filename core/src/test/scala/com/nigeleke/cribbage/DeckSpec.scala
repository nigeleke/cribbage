package com.nigeleke.cribbage

import model.*

import org.scalatest.*
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

class DeckSpec extends AnyWordSpec with Matchers {

  val deck: Deck = Deck.shuffledDeck

  "A Full Deck" should {

    "contain 52 Cards" in {
      deck.size should be(52)
    }

    "contain all Cards for all Suits and Faces" in {
      val allCards = (for {
        face <- Card.faces
        suit <- Card.suits
      } yield (face, suit)).map(Card(_, _))

      deck should contain theSameElementsAs (allCards)
    }

  }

  "A Deck" should {
    "allow a random card to be selected" in {
      val (remaining, cut) = deck.cut
      deck should contain(cut)
      remaining should not contain (cut)
      remaining.size should be(deck.size - 1)
      deck should contain allElementsOf (remaining)
    }
  }

}
