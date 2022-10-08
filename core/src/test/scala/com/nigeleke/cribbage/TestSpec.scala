package com.nigeleke.cribbage

import com.nigeleke.cribbage.model.*
import Cards.*
import GameState.*

import org.scalatest.*
import org.scalatest.matchers.should.*
import org.scalatest.wordspec.*

class TestSpec extends AnyWordSpec with Matchers:

  "A TestSpec" should {

    "display pretty GameStates" when {
      val players        = Set(Player.createPlayer, Player.createPlayer)
      val (dealer, pone) = (players.head, players.last)
      val deck           = fullDeck
      val scores         = players.map((_, Score.zero)).toMap
      val hands          = players.map((_, handOf(deck.toSeq.take(2)))).toMap
      val crib           = cribOf(deck.toSeq.take(4))
      val cut            = deck.toSeq.last
      val plays          = Plays(dealer, Seq.empty, Seq.empty)

      val sPlayers         = players.map(_.toPrettyString).mkString(", ")
      val (sDealer, sPone) = (dealer.toPrettyString, pone.toPrettyString)
      val sDeck            = deck.toPrettyString
      val sScores          = scores.map((p, s) => s"${p.toPrettyString}: ${s.toPrettyString}").mkString(", ")
      val sHands           = hands.map((p, h) => s"${p.toPrettyString}: ${h.toPrettyString}").mkString(", ")
      val sCrib            = crib.toPrettyString
      val sCut             = cut.toPrettyString
      val sPlays           = plays.toPrettyString

      "Starting" in {
        Starting(players).toPrettyString should be(s"""Starting(
             |  players:     $sPlayers
             |)""".stripMargin)
      }

      "Discarding" in {
        Discarding(deck, scores, hands, dealer, pone, crib).toPrettyString should be(s"""Discarding(
             |  deck:        $sDeck
             |  scores:      $sScores
             |  hands:       $sHands
             |  dealer/pone: $sDealer / $sPone
             |  crib:        $sCrib
             |)""".stripMargin)
      }

      "Playing" in {
        Playing(scores, hands, dealer, pone, crib, cut, plays).toPrettyString should be(
          s"""Playing(
             |  scores:      $sScores
             |  hands:       $sHands
             |  dealer/pone: $sDealer / $sPone
             |  crib:        $sCrib
             |  cut:         $sCut
             |  plays:       $sPlays
             |)""".stripMargin
        )
      }

      "Scoring" in {
        Scoring(scores, hands, dealer, pone, crib, cut).toPrettyString should be(
          s"""Scoring(
             |  scores:      $sScores
             |  hands:       $sHands
             |  dealer/pone: $sDealer / $sPone
             |  crib:        $sCrib
             |  cut:         $sCut
             |)""".stripMargin
        )
      }

      "Finished" in {
        Finished(scores).toPrettyString should be(
          s"""Finished(
             |  scores:      $sScores
             |)""".stripMargin
        )
      }
    }

  }
