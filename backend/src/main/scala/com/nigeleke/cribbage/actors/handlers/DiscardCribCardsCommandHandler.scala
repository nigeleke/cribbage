package com.nigeleke.cribbage.actors.handlers

import akka.persistence.typed.scaladsl.Effect
import com.nigeleke.cribbage.actors.Game._
import com.nigeleke.cribbage.actors.handlers.Validations._
import com.nigeleke.cribbage.actors.validate.Validation._

case class DiscardCribCardsCommandHandler(discard: DiscardCribCards, state: Discarding) extends CommandHandler {

  import CommandHandler._

  val playerId = discard.playerId
  val cards = discard.cards
  val game = state.game

  override def canDo: Option[String] =
    validate(playerInGame(playerId, game) and
      validDeal(game) and
      playerHoldsCards(playerId, cards, game) and
      discardingTwoCardsOnly(playerId, cards))

  lazy val events = CribCardsDiscarded(playerId, cards) +:
    (if (state.game.crib.size == 2) scoreCutAtStartOfPlay(game) else Seq.empty)

  override def effects: Effect[Event, State] = Effect.persist(events)

}
