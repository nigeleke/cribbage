package com.nigeleke.cribbage.model

import scala.util.Random

trait GameState

object GameState:
  case class Starting(
      players: Set[Player]
  ) extends GameState:
    override def toString: String =
      val sPlayers = players.mkString(", ")
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
    override def toString: String =
      val sScores = scores.mkString(", ")
      val sHands  = hands.mkString(", ")
      s"""Discarding(
           |  deck:        $deck
           |  scores:      $sScores
           |  hands:       $sHands
           |  dealer/pone: $dealer / $pone
           |  crib:        $crib
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
    override def toString: String =
      val sScores = scores.mkString(", ")
      val sHands  = hands.mkString(", ")
      s"""Playing(
           |  scores:      $sScores
           |  hands:       $sHands
           |  dealer/pone: $dealer / $pone
           |  crib:        $crib
           |  cut:         $cut
           |  plays:       $plays
           |)""".stripMargin

  case class Scoring(
      scores: Map[Player, Score],
      hands: Map[Player, Hand],
      dealer: Player,
      pone: Player,
      crib: Crib,
      cut: Card
  ) extends GameState:
    override def toString: String =
      val sScores = scores.mkString(", ")
      val sHands  = hands.mkString(", ")
      s"""Scoring(
           |  scores:      $sScores
           |  hands:       $sHands
           |  dealer/pone: $dealer / $pone
           |  crib:        $crib
           |  cut:         $cut
           |)""".stripMargin

  case class Finished(
      scores: Map[Player, Score]
  ) extends GameState:
    override def toString: String =
      val sScores = scores.mkString(", ")
      s"""Finished(
           |  scores:      $sScores
           |)""".stripMargin
