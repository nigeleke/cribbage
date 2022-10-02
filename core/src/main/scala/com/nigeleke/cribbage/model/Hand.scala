package com.nigeleke.cribbage.model

type Hand = Seq[Card]

object Hand:
  def empty: Hand = Seq.empty
