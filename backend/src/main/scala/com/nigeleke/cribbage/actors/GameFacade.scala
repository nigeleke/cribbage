package com.nigeleke.cribbage.actors

import akka.actor.typed.{ActorRef, Behavior}
import com.nigeleke.cribbage.actors.states.game.StartingGame
import com.nigeleke.cribbage.model.Card.{Id => CardId}
import com.nigeleke.cribbage.model.Player.{Id => PlayerId}

object GameFacade {

  sealed trait Command
  final case class Join(playerId: PlayerId) extends Command
  final case object SelectDealer extends Command
  final case object DealHands extends Command
  final case class DiscardCribCards(playerId: PlayerId, cards: Seq[CardId]) extends Command
  final case class PlayCard(playerId: PlayerId, cardId: CardId) extends Command
  final case class Pass(playerId: PlayerId) extends Command
  final case object CompletePlay extends Command
  final case object CompletePlays extends Command
  final case object ScorePoneHand extends Command
  final case object ScoreDealerHand extends Command
  final case object ScoreCrib extends Command
  final case object SwapDealer extends Command

  sealed trait Query extends Command
  final case class GetPlayers(replyTo: ActorRef[Set[PlayerId]]) extends Query

  sealed trait Event
  final case class PlayerJoined(playerId: PlayerId) extends Event

  def apply() : Behavior[Command] = StartingGame()

}
