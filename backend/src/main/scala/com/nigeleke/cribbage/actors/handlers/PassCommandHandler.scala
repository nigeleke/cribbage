package com.nigeleke.cribbage.actors.handlers

import akka.persistence.typed.scaladsl.Effect
import com.nigeleke.cribbage.actors.Game._
import com.nigeleke.cribbage.actors.validate._
import com.nigeleke.cribbage.actors.validate.Validation._

case class PassCommandHandler(pass: Pass, state: Playing) extends CommandHandler {

  import CommandHandler._

  val playerId = pass.playerId
  val game = state.game
  val play = game.play

  override def canDo: Option[String] =
    validate(
      PlayerInGame(playerId, game) and
      PlayerIsNextToLay(playerId, game) and
      PlayHasNoCardsToLay(playerId, game))

  lazy val events = Passed(playerId) +:
    (endPlay(game.withPass(playerId)) ++
      endPlays(game.withPass(playerId)))

  override def effects: Effect[Event, State] = Effect.persist(events)
}
