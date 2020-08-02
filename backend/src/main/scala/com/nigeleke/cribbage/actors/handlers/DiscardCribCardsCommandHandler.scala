package com.nigeleke.cribbage.actors.handlers

import akka.persistence.typed.scaladsl.Effect
import com.nigeleke.cribbage.actors.Game._
import com.nigeleke.cribbage.actors.validate.Validation._
import com.nigeleke.cribbage.actors.validate._

case class DiscardCribCardsCommandHandler(discard: DiscardCribCards, state: Discarding) extends CommandHandler {

  import CommandHandler._

  val playerId = discard.playerId
  val cards = discard.cards
  val game = state.game

  override def canDo: Option[String] =
    validate(PlayerInGame(playerId, game) and
      ValidDeal(game) and
      PlayerHoldsCards(playerId, cards, game) and
      DiscardingTwoCardsOnly(playerId, cards))

  lazy val events = CribCardsDiscarded(playerId, cards) +:
    (if (state.game.crib.size == 2) scoreCutAtStartOfPlay(game) else Seq.empty)

  override def effects: Effect[Event, State] = Effect.persist(events)

}
