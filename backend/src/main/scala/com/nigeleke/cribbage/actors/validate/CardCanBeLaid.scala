package com.nigeleke.cribbage.actors.validate

import com.nigeleke.cribbage.model._

case class CardCanBeLaid(card: Card, play: Play) extends Validation {

  def validate: Option[String] = {
    if (play.runningTotal + card.value <= 31) None
    else Some(s"Card $card cannot be laid in play $play (makes total > 31)")
  }

}
