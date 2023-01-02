package cribbage
package model

import Cards.*

final case class Playing(
    scores: Map[Player, Score],
    hands: Map[Player, Hand],
    dealer: Player,
    pone: Player,
    crib: Crib,
    cut: Card,
    plays: Plays
)
