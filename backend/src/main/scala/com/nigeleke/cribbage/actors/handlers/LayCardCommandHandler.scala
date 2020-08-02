package com.nigeleke.cribbage.actors.handlers

import akka.persistence.typed.scaladsl.Effect
import com.nigeleke.cribbage.actors.Game._
import com.nigeleke.cribbage.actors.handlers.Validations._
import com.nigeleke.cribbage.actors.validate.Validation._

case class LayCardCommandHandler(lay: LayCard, state: Playing) extends CommandHandler {

  import CommandHandler._

  val game = state.game
  val play = game.play

  val playerId = lay.playerId
  val card = lay.card

  override def canDo: Option[String] =
    validate(playerInGame(playerId, game) and
      playerIsNextToLay(playerId, game) and
      cardCanBeLaid(lay.card, play))

  lazy val events = CardLaid(playerId, card) +: {
    val gameWithLay = game.withLay(playerId, card)
    scoreLay(gameWithLay) ++ endPlay(gameWithLay) ++ endPlays(gameWithLay)
  }

  override def effects: Effect[Event, State] = Effect.persist(events)

}
