package com.nigeleke.cribbage.actors.validate

import com.nigeleke.cribbage.model.Player.{Id => PlayerId}
import com.nigeleke.cribbage.model._

case class PlayerHoldsCards(id: PlayerId, cards: Cards, game: Status) extends Validation {

  override def validate: Option[String] =
    if (cards.forall(game.hands(id).contains(_))) None
    else Option(s"Player $id does not hold all $cards")

}
