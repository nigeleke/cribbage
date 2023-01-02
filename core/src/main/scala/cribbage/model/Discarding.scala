package cribbage
package model

import Cards.*

/** The game while players choosing cards to discard into the crib.
  *
  * @param deck
  *   The remaining deck of cards after dealing.
  * @param scores
  *   The current scores.
  * @param hands
  *   The player's hands, with discarded / played cards removed if applicable.
  * @param dealer
  *   The current dealer.
  * @param pone
  *   The dealer's opponent.
  * @param crib
  *   The current crib discards.
  */
final case class Discarding(
    deck: Deck,
    scores: Map[Player, Score],
    hands: Map[Player, Hand],
    dealer: Player,
    pone: Player,
    crib: Crib
)
