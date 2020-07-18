package com.nigeleke.cribbage

import model.Player.{Id => PlayerId}

package object model {

  type Players = Set[PlayerId]
  type Deck    = Seq[Card]
  type Cards   = Seq[Card]
  type Hand    = Seq[Card]
  type Hands   = Map[PlayerId, Hand]
  type Crib    = Seq[Card]
  type Lays    = Seq[Lay]
  type Scores  = Map[PlayerId, Score]

}
