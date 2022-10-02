package com.nigeleke.cribbage

import com.nigeleke.cribbage.model.*
import GameState.*

import org.scalatest.*
import org.scalatest.matchers.should.*
import org.scalatest.wordspec.*

class TestSpec extends AnyWordSpec with Matchers:

  "A TestSpec" should {

    "display pretty GameStates" when {
      val players        = Set(Player.createPlayer, Player.createPlayer)
      val (dealer, pone) = (players.head, players.last)
      val sPlayers       = players.mkString(", ")
      val deck           = Deck.fullDeck
      val sDeck          = deck.mkString("ArraySeq(", ", ", ")")
      val scores         = players.map((_, Score.zero)).toMap
      val sScores        = scores.mkString(", ")
      val hands          = players.map((_, deck.take(2))).toMap
      val sHands         = hands.mkString(", ")
      val crib           = deck.take(4)
      val sCrib          = crib.mkString("ArraySeq(", ", ", ")")
      val cut            = deck.last
      val sCut           = cut.toString
      val plays          = Plays(dealer, Seq.empty, Seq.empty)
      val sPlays         = plays.toString

      "Starting" in {
        Starting(players).toString should be(s"""Starting(
             |  players:     $sPlayers
             |)""".stripMargin)
      }

      "Discarding" in {
        Discarding(deck, scores, hands, dealer, pone, crib).toString should be(s"""Discarding(
             |  deck:        $sDeck
             |  scores:      $sScores
             |  hands:       $sHands
             |  dealer/pone: $dealer / $pone
             |  crib:        $sCrib
             |)""".stripMargin)
      }

      "Playing" in {
        Playing(scores, hands, dealer, pone, crib, cut, plays).toString should be(
          s"""Playing(
             |  scores:      $sScores
             |  hands:       $sHands
             |  dealer/pone: $dealer / $pone
             |  crib:        $sCrib
             |  cut:         $sCut
             |  plays:       $sPlays
             |)""".stripMargin
        )
      }

      "Scoring" in {
        Scoring(scores, hands, dealer, pone, crib, cut).toString should be(
          s"""Scoring(
             |  scores:      $sScores
             |  hands:       $sHands
             |  dealer/pone: $dealer / $pone
             |  crib:        $sCrib
             |  cut:         $sCut
             |)""".stripMargin
        )
      }

      "Finished" in {
        Finished(scores).toString should be(
          s"""Finished(
             |  scores:      $sScores
             |)""".stripMargin
        )
      }
    }

  }
