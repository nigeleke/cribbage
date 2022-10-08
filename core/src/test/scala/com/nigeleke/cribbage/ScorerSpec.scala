package com.nigeleke.cribbage

import model.*
import Card.*
import Face.*, Suit.*
import Cards.*
import Points.*
import util.*

import org.scalatest.*
import matchers.should.Matchers
import wordspec.AnyWordSpec

class ScorerSpec extends AnyWordSpec with Matchers {

  type FaceSuit = (Face, Suit)
  type TestData = (Seq[FaceSuit], Points)

  val player = Player.createPlayer

  def faceSuitToCard(fs: FaceSuit): Card = Card(fs._1, fs._2)

  def faceSuitsToCards(fss: Seq[FaceSuit]): Seq[Card] = fss.map(faceSuitToCard)

  def faceSuitsToPlay(faceSuits: Seq[FaceSuit]): Seq[Plays.Play] =
    faceSuitsToCards(faceSuits).map(card => Plays.Laid(player, card))

  "ScoreCut" should {
    "score 2 for his heals" in {
      fullDeck.toSeq.foreach { card =>
        val expectedPoints = Cut(heals = if card.face == Card.Face.Jack then 2 else 0)
        Scorer.forCut(card) should be(expectedPoints)
      }
    }

  }

  "ScorePlay" should {

    def checkPlays(tests: Seq[TestData]) = tests.foreach { test =>
      val faceSuits     = test._1
      val expectedScore = test._2
      assertScore(faceSuits, expectedScore.asInstanceOf[Play])
    }

    def assertScore(faceSuits: Seq[FaceSuit], expectedScore: Play) =
      val play = faceSuitsToPlay(faceSuits)
      Scorer.forPlay(play) should be(expectedScore)

    "score totals of fifteen in a play" in {
      // format: off
      val plays = Seq(
        Seq((Ten, Clubs), (Five, Clubs))                 -> Play(fifteens = 2),
        Seq((Jack, Clubs), (Five, Clubs))                -> Play(fifteens = 2),
        Seq((Nine, Clubs), (Six, Clubs))                 -> Play(fifteens = 2),
        Seq((Eight, Clubs), (Seven, Clubs))              -> Play(fifteens = 2),
        Seq((Ace, Clubs), (Four, Clubs), (Ten, Clubs))   -> Play(fifteens = 2),
        Seq((Two, Clubs), (Three, Clubs), (Ten, Clubs))  -> Play(fifteens = 2),
        Seq((Two, Clubs), (Three, Clubs), (Ace, Clubs),
          (Three, Spades), (Ace, Spades), (Five, Hearts)) -> Play(fifteens = 2),
        Seq((Two, Clubs), (Three, Clubs), (Ace, Clubs),
          (Three, Spades), (Ace, Spades), (Six, Hearts))  -> Play()
      )
      // format: on
      checkPlays(plays)
    }

    "score pairs in a play" in {
      val plays = Seq(
        Seq((Ten, Clubs), (Ten, Hearts))               -> Play(pairs = 2),
        Seq((Ace, Clubs), (Ace, Hearts))               -> Play(pairs = 2),
        Seq((Ten, Spades), (Jack, Spades))             -> Play(pairs = 0),
        Seq((Ace, Clubs), (Ten, Clubs), (Ten, Hearts)) -> Play(pairs = 2)
      )
      checkPlays(plays)
    }

    "score repeated pairs in a play" in {
      // format: off
      val plays = Seq(
        Seq((Ace, Clubs), (Ace, Hearts))                                   -> Play(pairs = 2),
        Seq((Ace, Clubs), (Ace, Hearts), (Ace, Diamonds))                  -> Play(pairs = 6),
        Seq((Ace, Clubs), (Ace, Hearts), (Ace, Diamonds), (Ace, Spades))   -> Play(pairs = 12),
        Seq((Two, Diamonds), (Ace, Clubs), (Ace, Hearts))                  -> Play(pairs = 2),
        Seq((Two, Diamonds), (Ace, Clubs), (Ace, Hearts), (Ace, Diamonds)) -> Play(pairs = 6),
        Seq((Two, Diamonds), (Ace, Clubs), (Ace, Hearts), (Ace, Diamonds),
          (Ace, Spades))                                                   -> Play(pairs = 12),
        Seq((Two, Diamonds), (Ace, Clubs), (Two, Hearts), (Ace, Hearts))   -> Play(),
        Seq((Two, Diamonds), (Ace, Clubs), (Two, Hearts), (Ace, Hearts),
          (Ace, Diamonds))                                                 -> Play(pairs = 2),
        Seq((Two, Diamonds), (Ace, Clubs), (Ace, Hearts), (Two, Hearts),
          (Ace, Diamonds))                                                 -> Play(),
        Seq((Two, Diamonds), (Ace, Clubs), (Two, Hearts), (Ace, Hearts),
          (Ace, Diamonds), (Ace, Spades))                                  -> Play(pairs = 6),
        Seq((Two, Diamonds), (Ace, Clubs), (Ace, Hearts), (Two, Hearts),
          (Ace, Diamonds), (Ace, Spades))                                  -> Play(pairs = 2),
        Seq((Two, Diamonds), (Ace, Clubs), (Ace, Hearts), (Ace, Diamonds),
          (Two, Hearts), (Ace, Spades))                                    -> Play()
      )
      // format: on
      checkPlays(plays)
    }

    "score runs formed during a play" in {
      // format: off
      val plays = Seq(
        Seq((Ace, Clubs), (Two, Hearts), (Three, Diamonds))                  -> Play(runs = 3),
        Seq((Ace, Clubs), (Three, Diamonds), (Two, Hearts))                  -> Play(runs = 3),
        Seq((Four, Spades), (Ace, Clubs), (Three, Diamonds), (Two, Hearts))  -> Play(runs = 4),
        Seq((Four, Spades), (Ace, Clubs), (Three, Diamonds), (Five, Hearts)) -> Play(),
        Seq((Five, Spades), (Two, Clubs), (Four, Diamonds), (Six, Hearts),
          (Three, Clubs))                                                    -> Play(runs = 5)
      )
      // format: on
      checkPlays(plays)
    }
  }

  "ScoreEndOfPlay" should {
    "score nothing if not and of play" in {
      val play = Plays(
        player,
        faceSuitsToPlay(
          Seq((Five, Spades), (Two, Clubs), (Four, Diamonds), (Six, Hearts), (Three, Clubs))
        ),
        Seq.empty
      )
      Scorer.forEndPlay(play) should be(EndPlay())
    }

    val passes = Seq(Plays.Pass(player), Plays.Pass(Player.createPlayer))

    "score one if at end of play and running total less than 31" when {
      "interim end of play" in {
        val play = Plays(
          player,
          faceSuitsToPlay(Seq((Five, Spades), (Two, Clubs))) ++ passes,
          faceSuitsToPlay(Seq((Four, Diamonds), (Six, Hearts), (Three, Clubs))) ++ passes
        )
        Scorer.forEndPlay(play) should be(EndPlay(1))
      }

      "final end of play" in {
        val play = Plays(
          player,
          faceSuitsToPlay(Seq((Five, Spades), (Two, Clubs), (Four, Diamonds), (Six, Hearts))),
          faceSuitsToPlay(Seq((Three, Clubs), (Two, Hearts), (Two, Diamonds), (Two, Spades))) ++
            passes
        )
        Scorer.forEndPlay(play) should be(EndPlay(1))
      }
    }

    "score two if at end of play and running total is exactly 31" when {
      "interim end of play" in {
        val play = Plays(
          player,
          faceSuitsToPlay(Seq((Ten, Spades), (Ten, Clubs), (Ten, Diamonds), (Ace, Spades))) ++
            passes,
          faceSuitsToPlay(Seq((Four, Diamonds), (Six, Hearts), (Three, Clubs))) ++ passes
        )
        Scorer.forEndPlay(play) should be(EndPlay(2))
      }

      "final end of play" in {
        val play = Plays(
          player,
          faceSuitsToPlay(Seq((Ten, Spades), (Ten, Clubs), (Ten, Diamonds), (Ace, Spades))),
          faceSuitsToPlay(Seq((Four, Diamonds), (Six, Hearts), (Three, Clubs), (King, Clubs))) ++
            passes
        )
        Scorer.forEndPlay(play) should be(EndPlay(2))
      }
    }
  }

  "ScoreCards" should {

    def checkCards(tests: Seq[TestData], cut: FaceSuit) = tests.foreach { test =>
      val faceSuits      = test._1
      val expectedPoints = test._2
      assertScore(faceSuits, cut, expectedPoints.asInstanceOf[Cards])
    }

    def assertScore(faceSuits: Seq[FaceSuit], cutFaceSuit: FaceSuit, expectedScore: Cards) =
      val cards = handOf(faceSuitsToCards(faceSuits))
      val cut   = faceSuitToCard(cutFaceSuit)
      Scorer.forCards(cards, cut) should be(expectedScore)

    "score cards with fifteens" in {
      val cut   = (Queen, Hearts)
      // format: off
      val cards = Seq(
        Seq((Five, Clubs), (Nine, Hearts), (Four, Diamonds), (Two, Spades)) -> Points.Cards(fifteens = 4),
        Seq((Ace, Clubs), (Four, Clubs), (Ten, Clubs), (King, Hearts))      -> Points.Cards(fifteens = 6)
      )
      // format: on
      checkCards(cards, cut)
    }

    "score cards with pairs" in {
      val cut   = (Ace, Hearts)
      // format: off
      val cards = Seq(
        Seq((Five, Clubs), (Five, Hearts), (Ace, Diamonds), (Two, Spades))     -> Points.Cards(pairs = 4),
        Seq((Queen, Clubs), (Three, Clubs), (Three, Diamonds), (King, Hearts)) -> Points.Cards(pairs = 2),
        Seq((Four, Hearts), (Four, Clubs), (Four, Diamonds), (Five, Hearts))   -> Points.Cards(pairs = 6),
        Seq((Four, Hearts), (Four, Clubs), (Four, Diamonds), (Four, Spades))   -> Points.Cards(pairs = 12)
      )
      // format: on
      checkCards(cards, cut)
    }

    "score cards with runs" in {
      val cut   = (Ace, Diamonds)
      // format: off
      val cards = Seq(
        Seq((Two, Clubs), (Jack, Hearts), (Queen, Diamonds), (King, Spades))  -> Points.Cards(runs = 3),
        Seq((Jack, Clubs), (Jack, Hearts), (Queen, Diamonds), (King, Spades)) -> Points.Cards(pairs = 2, runs = 6),
        Seq((Ten, Clubs), (Jack, Hearts), (Queen, Diamonds), (King, Spades))  -> Points.Cards(runs = 4),
        Seq((Two, Hearts), (Two, Clubs), (Three, Diamonds), (Four, Spades))   -> Points.Cards(pairs = 2, runs = 8),
        Seq((Two, Hearts), (Five, Clubs), (Three, Diamonds), (Four, Spades))  -> Points.Cards(runs = 5, fifteens = 2)
      )
      // format: on
      checkCards(cards, cut)
    }

    "score cards with one for his heels" in {
      val cut   = (Ace, Hearts)
      // format: off
      val cards = Seq(
        Seq((Two, Clubs), (Jack, Hearts), (Ten, Diamonds), (Six, Spades))   -> Points.Cards(heels = 1),
        Seq((Two, Clubs), (Jack, Diamonds), (Ten, Diamonds), (Six, Spades)) -> Points.Cards()
      )
      // format: on
      checkCards(cards, cut)
    }

    "score cards with flushes" in {
      val cut   = (Ace, Diamonds)
      // format: off
      val cards = Seq(
        Seq((Two, Diamonds), (Seven, Diamonds), (Queen, Diamonds), (King, Diamonds)) -> Points.Cards(flushes = 5),
        Seq((Two, Hearts), (Seven, Hearts), (Queen, Hearts), (King, Hearts))         -> Points.Cards(flushes = 4),
        Seq((Two, Diamonds), (Seven, Diamonds), (Queen, Diamonds), (King, Clubs))    -> Points.Cards()
      )
      // format: on
      checkCards(cards, cut)
    }

  }
}
