package com.nigeleke.cribbage.model

import scala.util.Random

trait GameState

object GameState:
  extension (cards: Seq[Card])
    def toPrettyString = cards.map(_.toPrettyString).mkString("[", " ", "]")

  private def prettyHands(hands: Map[Player, Hand]) =
    hands.map { (p, h) => s"${p.toPrettyString}: ${h.toPrettyString}" }.mkString(", ")

  private def prettyPlayers(players: Set[Player]) = players.map(_.toPrettyString).mkString((", "))

  private def prettyScores(scores: Map[Player, Score]) =
    scores.map((p, s) => s"${p.toPrettyString}: ${s.toPrettyString}").mkString(", ")

  case class Starting(
      players: Set[Player]
  ) extends GameState:
    def toPrettyString: String =
      val sPlayers = prettyPlayers(players)
      s"""Starting(
           |  players:     $sPlayers
           |)""".stripMargin

  case class Discarding(
      deck: Deck,
      scores: Map[Player, Score],
      hands: Map[Player, Hand],
      dealer: Player,
      pone: Player,
      crib: Crib
  ) extends GameState:
    def toPrettyString: String =
      val sDeck   = deck.toPrettyString
      val sScores = prettyScores(scores)
      val sHands  = prettyHands(hands)
      val sDealer = dealer.toPrettyString
      val sPone   = pone.toPrettyString
      val sCrib   = crib.toPrettyString
      s"""Discarding(
           |  deck:        $sDeck
           |  scores:      $sScores
           |  hands:       $sHands
           |  dealer/pone: $sDealer / $sPone
           |  crib:        $sCrib
           |)""".stripMargin

  case class Playing(
      scores: Map[Player, Score],
      hands: Map[Player, Hand],
      dealer: Player,
      pone: Player,
      crib: Crib,
      cut: Card,
      plays: Plays
  ) extends GameState:
    def toPrettyString: String =
      val sScores = prettyScores(scores)
      val sHands  = prettyHands(hands)
      val sDealer = dealer.toPrettyString
      val sPone   = pone.toPrettyString
      val sCrib   = crib.toPrettyString
      val sCut    = cut.toPrettyString
      val sPlays  = plays.toPrettyString
      s"""Playing(
           |  scores:      $sScores
           |  hands:       $sHands
           |  dealer/pone: $sDealer / $sPone
           |  crib:        $sCrib
           |  cut:         $sCut
           |  plays:       $sPlays
           |)""".stripMargin

  case class Scoring(
      scores: Map[Player, Score],
      hands: Map[Player, Hand],
      dealer: Player,
      pone: Player,
      crib: Crib,
      cut: Card
  ) extends GameState:
    def toPrettyString: String =
      val sScores = prettyScores(scores)
      val sHands  = prettyHands(hands)
      val sDealer = dealer.toPrettyString
      val sPone   = pone.toPrettyString
      val sCrib   = crib.toPrettyString
      val sCut    = cut.toPrettyString
      s"""Scoring(
           |  scores:      $sScores
           |  hands:       $sHands
           |  dealer/pone: $sDealer / $sPone
           |  crib:        $sCrib
           |  cut:         $sCut
           |)""".stripMargin

  case class Finished(
      scores: Map[Player, Score]
  ) extends GameState:
    def toPrettyString: String =
      val sScores = prettyScores(scores)
      s"""Finished(
           |  scores:      $sScores
           |)""".stripMargin
