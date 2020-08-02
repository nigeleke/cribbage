package com.nigeleke.cribbage.actors.validate

import com.nigeleke.cribbage.model._

case class ValidDeal(game: Status) extends Validation {

  def validate = {
    val allCardsDealt = (game.deck ++ game.hands.flatMap(_._2) ++ game.crib).size == 52
    if (allCardsDealt) None
    else Option(
      s"""Invalid deal:
         | Deck: (${game.deck.size}) ${game.deck}
         | Deal: (${game.hands.flatMap(_._2).size}) ${game.hands}
         | Crib: (${game.crib})  ${game.crib}
         |""".stripMargin)
  }

}
