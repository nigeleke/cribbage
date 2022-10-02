package com.nigeleke.cribbage.model

type Crib = Seq[Card]

object Crib:

  def empty: Crib = Seq.empty
