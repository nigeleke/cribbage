package cribbage
package model

trait Draw

object Draw:
  final case class Decided(draws: Map[Player, Card], dealer: Player, pone: Player) extends Draw
  final case class Undecided(draws: Map[Player, Card])                             extends Draw
