package cribbage
package model

trait Rule

object Rule:
  final case class NonParticipatingPlayer(player: Player)            extends Rule
  final case class NotYourTurn(player: Player)                       extends Rule
  final case class PlayTargetAttainable(player: Player)              extends Rule
  final case class PlayTargetExceeded(player: Player, card: Card)    extends Rule
  final case class TooManyDiscards(player: Player, cards: Seq[Card]) extends Rule
  final case class UnheldCard(player: Player, card: Card)            extends Rule
  final case class UnheldCards(player: Player, cards: Seq[Card])     extends Rule
