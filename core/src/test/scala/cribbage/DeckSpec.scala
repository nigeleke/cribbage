package cribbage

import cribbage.model.Card
import cribbage.model.Cards.*
import org.scalatest.*
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

class DeckSpec extends AnyWordSpec with Matchers {

  val deck: Deck = shuffledDeck

  "A Full Deck" should {

    "contain 52 Cards" in {
      deck.size should be(52)
    }

    "contain all Cards for all Suits and Faces" in {
      val allCards = (for {
        face <- Card.faces
        suit <- Card.suits
      } yield (face, suit)).map(Card(_, _))

      deck.toSeq should contain theSameElementsAs (allCards)
    }

  }

  "A Deck" should {
    "allow a random card to be cut" in {
      val (remaining, cut) = deck.cut
      deck.toSeq should contain(cut)
      remaining.toSeq should not contain (cut)
      remaining.size should be(deck.size - 1)
      deck.toSeq should contain allElementsOf (remaining.toSeq)
    }
  }

}
