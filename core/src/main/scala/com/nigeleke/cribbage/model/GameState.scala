package com.nigeleke.cribbage.model

import Cards.*

import scala.util.Random

sealed trait GameState

object GameState:
  private def prettyHands(hands: Map[Player, Hand]) =
    hands.map { (p, h) => s"${p.toPrettyString}: ${h.toPrettyString}" }.mkString(", ")

  private def prettyPlayers(players: Set[Player]) = players.map(_.toPrettyString).mkString((", "))

  private def prettyScores(scores: Map[Player, Score]) =
    scores.map((p, s) => s"${p.toPrettyString}: ${s.toPrettyString}").mkString(", ")

  /** The Starting state for a game, while players are joining.
    * @param players
    *   The currently joined players.
    */
  case class Starting(
      players: Set[Player]
  ) extends GameState:
    def toPrettyString: String =
      val sPlayers = prettyPlayers(players)
      s"""Starting(
           |  players:     $sPlayers
           |)""".stripMargin

  /** The state of play while players are discarding into the crib,
    * @param deck
    *   The remaining deck of cards after dealing.
    * @param scores
    *   The current scores.
    * @param hands
    *   The player's hands, with discarded cards removed if applicable.
    * @param dealer
    *   The current dealer.
    * @param pone
    *   The dealer's opponent.
    * @param crib
    *   The current crib discards.
    */
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

  /** The state of the game while making plays.
    * @param scores
    *   The current scores.
    * @param hands
    *   The player's current hands, after plays have been made.
    * @param dealer
    *   The current dealer.
    * @param pone
    *   The dealer's opponent.
    * @param crib
    *   The previously discarded crib,
    * @param cut
    *   The cut that was made prior to play.
    * @param plays
    *   The current and previous plays.
    */
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

  /** The game in its Scoring state.
    * @param scores
    *   The current scores.
    * @param hands
    *   The player's hands (if not scored).
    * @param dealer
    *   The current dealer.
    * @param pone
    *   The dealer's opponent.
    * @param crib
    *   The crib (if not scored).
    * @param cut
    *   The cut that was made prior to play.
    */
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

  /** A finished game.
    * @param scores
    *   The final scores.
    */
  case class Finished(
      scores: Map[Player, Score]
  ) extends GameState:
    def toPrettyString: String =
      val sScores = prettyScores(scores)
      s"""Finished(
           |  scores:      $sScores
           |)""".stripMargin
