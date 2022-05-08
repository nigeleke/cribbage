package com.nigeleke.cribbage

import model.*
import Card.Face.*
import Card.Suit.*
import util.*

import org.scalatest.*
import matchers.should.Matchers
import wordspec.AnyWordSpec

class ScorerSpec extends AnyWordSpec with Matchers {

  type FaceSuit = (Face, Suit)
  type TestData = (Seq[FaceSuit], Points)

  val playerId = Player.newId

  def faceSuitToCard(fs: FaceSuit): Card = Card(fs._1, fs._2)

  def faceSuitsToCards(fss: Seq[FaceSuit]): Seq[Card] = fss.map(faceSuitToCard)

  def faceSuitsToPlay(faceSuits: Seq[FaceSuit]): Seq[Play] =
    faceSuitsToCards(faceSuits).map(card => Plays.Play.Laid(playerId, card))

  "ScorePlay" should {

    def checkPlays(tests: Seq[TestData]) = tests.foreach { test =>
      val faceSuits = test._1
      val expectedScore = test._2
      assertScore(faceSuits, expectedScore.asInstanceOf[PlayPoints])
    }

    def assertScore(faceSuits: Seq[FaceSuit], expectedScore: PlayPoints) =
      val play = faceSuitsToPlay(faceSuits)
      Scorer.forPlay(play) should be(expectedScore)

    "score totals of fifteen in a play" in {
      val plays = Seq(
        Seq((Ten, Clubs), (Five, Clubs)) -> PlayPoints(fifteens = 2),
        Seq((Jack, Clubs), (Five, Clubs)) -> PlayPoints(fifteens = 2),
        Seq((Nine, Clubs), (Six, Clubs)) -> PlayPoints(fifteens = 2),
        Seq((Eight, Clubs), (Seven, Clubs)) -> PlayPoints(fifteens = 2),
        Seq((Ace, Clubs), (Four, Clubs), (Ten, Clubs)) -> PlayPoints(fifteens = 2),
        Seq((Two, Clubs), (Three, Clubs), (Ten, Clubs)) -> PlayPoints(fifteens = 2),
        Seq((Two, Clubs), (Three, Clubs), (Ace, Clubs), (Three, Spades), (Ace, Spades), (Five, Hearts)) -> PlayPoints(fifteens = 2),
        Seq((Two, Clubs), (Three, Clubs), (Ace, Clubs), (Three, Spades), (Ace, Spades), (Six, Hearts)) -> PlayPoints()
      )
      checkPlays(plays)
    }

    "score pairs in a play" in {
      val plays = Seq(
        Seq((Ten, Clubs), (Ten, Hearts)) -> PlayPoints(pairs = 2),
        Seq((Ace, Clubs), (Ace, Hearts)) -> PlayPoints(pairs = 2),
        Seq((Ten, Spades), (Jack, Spades)) -> PlayPoints(pairs = 0),
        Seq((Ace, Clubs), (Ten, Clubs), (Ten, Hearts)) -> PlayPoints(pairs = 2)
      )
      checkPlays(plays)
    }

    "score repeated pairs in a play" in {
      val plays = Seq(
        Seq((Ace, Clubs), (Ace, Hearts)) -> PlayPoints(pairs = 2),
        Seq((Ace, Clubs), (Ace, Hearts), (Ace, Diamonds)) -> PlayPoints(pairs = 6),
        Seq((Ace, Clubs), (Ace, Hearts), (Ace, Diamonds), (Ace, Spades)) -> PlayPoints(pairs = 12),
        Seq((Two, Diamonds), (Ace, Clubs), (Ace, Hearts)) -> PlayPoints(pairs = 2),
        Seq((Two, Diamonds), (Ace, Clubs), (Ace, Hearts), (Ace, Diamonds)) -> PlayPoints(pairs = 6),
        Seq((Two, Diamonds), (Ace, Clubs), (Ace, Hearts), (Ace, Diamonds), (Ace, Spades)) -> PlayPoints(pairs = 12),
        Seq((Two, Diamonds), (Ace, Clubs), (Two, Hearts), (Ace, Hearts)) -> PlayPoints(),
        Seq((Two, Diamonds), (Ace, Clubs), (Two, Hearts), (Ace, Hearts), (Ace, Diamonds)) -> PlayPoints(pairs = 2),
        Seq((Two, Diamonds), (Ace, Clubs), (Ace, Hearts), (Two, Hearts), (Ace, Diamonds)) -> PlayPoints(),
        Seq((Two, Diamonds), (Ace, Clubs), (Two, Hearts), (Ace, Hearts), (Ace, Diamonds), (Ace, Spades)) -> PlayPoints(pairs = 6),
        Seq((Two, Diamonds), (Ace, Clubs), (Ace, Hearts), (Two, Hearts), (Ace, Diamonds), (Ace, Spades)) -> PlayPoints(pairs = 2),
        Seq((Two, Diamonds), (Ace, Clubs), (Ace, Hearts), (Ace, Diamonds), (Two, Hearts), (Ace, Spades)) -> PlayPoints()
      )
      checkPlays(plays)
    }

    "score runs formed during a play" in {
      val plays = Seq(
        Seq((Ace, Clubs), (Two, Hearts), (Three, Diamonds)) -> PlayPoints(runs = 3),
        Seq((Ace, Clubs), (Three, Diamonds), (Two, Hearts)) -> PlayPoints(runs = 3),
        Seq((Four, Spades), (Ace, Clubs), (Three, Diamonds), (Two, Hearts)) -> PlayPoints(runs = 4),
        Seq((Four, Spades), (Ace, Clubs), (Three, Diamonds), (Five, Hearts)) -> PlayPoints(),
        Seq((Five, Spades), (Two, Clubs), (Four, Diamonds), (Six, Hearts), (Three, Clubs)) -> PlayPoints(runs = 5)
      )
      checkPlays(plays)
    }
  }

  "ScoreEndOfPlay" should {
    "score nothing if not and of play" in {
      val play = Plays(
        Some(playerId),
        faceSuitsToPlay(Seq((Five, Spades), (Two, Clubs), (Four, Diamonds), (Six, Hearts), (Three, Clubs))),
        Set.empty,
        Seq.empty
      )
      Scorer.forEndPlay(play) should be(EndPlayPoints())
    }

    val passes = Seq.fill(2)(Plays.Play.Pass(playerId))

    "score one if at end of play and running total less than 31" when {
      "interim end of play" in {
        val play = Plays(
          Some(playerId),
          faceSuitsToPlay(Seq((Five, Spades), (Two, Clubs))) ++ passes,
          Set(playerId, Player.newId),
          faceSuitsToPlay(Seq((Four, Diamonds), (Six, Hearts), (Three, Clubs))) ++ passes
        )
        Scorer.forEndPlay(play) should be(EndPlayPoints(1))
      }

      "final end of play" in {
        val play = Plays(
          Some(playerId),
          faceSuitsToPlay(Seq((Five, Spades), (Two, Clubs), (Four, Diamonds), (Six, Hearts))),
          Set(playerId),
          faceSuitsToPlay(Seq((Three, Clubs), (Two, Hearts), (Two, Diamonds), (Two, Spades))) ++ passes
        )
        Scorer.forEndPlay(play) should be(EndPlayPoints(1))
      }
    }

    "score two if at end of play and running total is exactly 31" when {
      "interim end of play" in {
        val play = Plays(
          Some(playerId),
          faceSuitsToPlay(Seq((Ten, Spades), (Ten, Clubs), (Ten, Diamonds), (Ace, Spades))) ++ passes,
          Set(playerId, Player.newId),
          faceSuitsToPlay(Seq((Four, Diamonds), (Six, Hearts), (Three, Clubs))) ++ passes
        )
        Scorer.forEndPlay(play) should be(EndPlayPoints(2))
      }

      "final end of play" in {
        val play = Plays(
          Some(playerId),
          faceSuitsToPlay(Seq((Ten, Spades), (Ten, Clubs), (Ten, Diamonds), (Ace, Spades))),
          Set(playerId, Player.newId),
          faceSuitsToPlay(Seq((Four, Diamonds), (Six, Hearts), (Three, Clubs), (King, Clubs))) ++ passes
        )
        Scorer.forEndPlay(play) should be(EndPlayPoints(2))
      }
    }
  }

  "ScoreCards" should {

    def checkCards(tests: Seq[TestData], cut: FaceSuit) = tests.foreach { test =>
      val faceSuits = test._1
      val expectedPoints = test._2
      assertScore(faceSuits, cut, expectedPoints.asInstanceOf[CardsPoints])
    }

    def assertScore(faceSuits: Seq[FaceSuit], cutFaceSuit: FaceSuit, expectedScore: CardsPoints) =
      val cards = faceSuitsToCards(faceSuits)
      val cut = faceSuitToCard(cutFaceSuit)
      Scorer.forCards(cards, cut) should be(expectedScore)

    "score cards with fifteens" in {
      val cut = (Queen, Hearts)
      val cards = Seq(
        Seq((Five, Clubs), (Nine, Hearts), (Four, Diamonds), (Two, Spades)) -> CardsPoints(fifteens = 4),
        Seq((Ace, Clubs), (Four, Clubs), (Ten, Clubs), (King, Hearts)) -> CardsPoints(fifteens = 6)
      )
      checkCards(cards, cut)
    }

    "score cards with pairs" in {
      val cut = (Ace, Hearts)
      val cards = Seq(
        Seq((Five, Clubs), (Five, Hearts), (Ace, Diamonds), (Two, Spades)) -> CardsPoints(pairs = 4),
        Seq((Queen, Clubs), (Three, Clubs), (Three, Diamonds), (King, Hearts)) -> CardsPoints(pairs = 2),
        Seq((Four, Hearts), (Four, Clubs), (Four, Diamonds), (Five, Hearts)) -> CardsPoints(pairs = 6),
        Seq((Four, Hearts), (Four, Clubs), (Four, Diamonds), (Four, Spades)) -> CardsPoints(pairs = 12)
      )
      checkCards(cards, cut)
    }

    "score cards with runs" in {
      val cut = (Ace, Diamonds)
      val cards = Seq(
        Seq((Two, Clubs), (Jack, Hearts), (Queen, Diamonds), (King, Spades)) -> CardsPoints(runs = 3),
        Seq((Jack, Clubs), (Jack, Hearts), (Queen, Diamonds), (King, Spades)) -> CardsPoints(pairs = 2, runs = 6),
        Seq((Ten, Clubs), (Jack, Hearts), (Queen, Diamonds), (King, Spades)) -> CardsPoints(runs = 4),
        Seq((Two, Hearts), (Two, Clubs), (Three, Diamonds), (Four, Spades)) -> CardsPoints(pairs = 2, runs = 8),
        Seq((Two, Hearts), (Five, Clubs), (Three, Diamonds), (Four, Spades)) -> CardsPoints(runs = 5, fifteens = 2)
      )
      checkCards(cards, cut)
    }

    "score cards with one for his heels" in {
      val cut = (Ace, Hearts)
      val cards = Seq(
        Seq((Two, Clubs), (Jack, Hearts), (Ten, Diamonds), (Six, Spades)) -> CardsPoints(heels = 1),
        Seq((Two, Clubs), (Jack, Diamonds), (Ten, Diamonds), (Six, Spades)) -> CardsPoints()
      )
      checkCards(cards, cut)
    }

    "score cards with flushes" in {
      val cut = (Ace, Diamonds)
      val cards = Seq(
        Seq((Two, Diamonds), (Seven, Diamonds), (Queen, Diamonds), (King, Diamonds)) -> CardsPoints(flushes = 5),
        Seq((Two, Hearts), (Seven, Hearts), (Queen, Hearts), (King, Hearts)) -> CardsPoints(flushes = 4),
        Seq((Two, Diamonds), (Seven, Diamonds), (Queen, Diamonds), (King, Clubs)) -> CardsPoints()
      )
      checkCards(cards, cut)
    }

  }
}
