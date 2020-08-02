package com.nigeleke.cribbage.actors.validate

import com.nigeleke.cribbage.model.Player.{Id => PlayerId}
import com.nigeleke.cribbage.model._

case class DiscardingTwoCardsOnly(id: PlayerId, cards: Cards) extends Validation {

  def validate = {
    if (cards.size == 2) None
    else Some(s"Player $id must discard two cards into the crib")
  }

}
