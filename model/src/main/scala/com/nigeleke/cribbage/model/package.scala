package com.nigeleke.cribbage

import java.util.UUID

import model.Card.{Id => CardId}
import model.Player.{Id => PlayerId}

package object model {

  type Players = Set[PlayerId]
  type Hand = Seq[CardId]
  type Hands = Map[PlayerId, Hand]
  type Crib = Seq[CardId]
  type Plays = Seq[Play]
  type Scores = Map[PlayerId, Score]

}
