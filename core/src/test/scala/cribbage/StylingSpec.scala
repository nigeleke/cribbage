package cribbage

import model.*
import model.Card.Face.*
import model.Card.Suit.*
import styling.Styling.*

import org.scalatest.*
import org.scalatest.matchers.should.*
import org.scalatest.wordspec.*

class StylingSpec extends AnyWordSpec with Matchers:

  "Styling" should {

    val players        = Set(Player.newPlayer, Player.newPlayer)
    val (dealer, pone) = (players.head, players.last)
    val scores         = players.map((_, Score.zero)).toMap
    val hands          = players.map((_, Cards.emptyHand)).toMap
    val crib           = Cards.emptyCrib
    val cut            = Card(Ace, Spades)
    val plays          =
      Plays(
        pone,
        Seq(Plays.Laid(pone, Card(Ace, Spades)), Plays.Pass(dealer)),
        Seq(Plays.Laid(pone, Card(Ace, Clubs)), Plays.Pass(dealer))
      )

    "style Discarding" in {
      val discarding = Discarding(Cards.fullDeck, scores, hands, dealer, pone, crib)
      discarding.styled should be(a[String])
    }

    "style Playing" in {
      val playing = Playing(scores, hands, dealer, pone, crib, cut, plays)
      playing.styled should be(a[String])
    }

    "style Finished" in {
      val finished = Finished(scores)
      finished.styled should be(a[String])
    }
  }
