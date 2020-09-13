/*
 * Copyright (C) 2020  Nigel Eke
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

package com.nigeleke.cribbage.entity

import java.util.UUID

import akka.actor.typed.scaladsl.Behaviors
import akka.actor.typed.{ ActorRef, Behavior }
import akka.persistence.typed.PersistenceId
import akka.persistence.typed.scaladsl.{ Effect, EventSourcedBehavior, ReplyEffect }
import com.nigeleke.cribbage.entity.handlers._
import com.nigeleke.cribbage.json.CborSerializable
import com.nigeleke.cribbage.model._
import com.nigeleke.cribbage.model.{ Card, Deck, Game, Hands, Points }
import com.nigeleke.cribbage.model.Card.{ Id => CardId }
import com.nigeleke.cribbage.model.Player.{ Id => PlayerId }
import org.slf4j.Logger

// SRR: Apply actions to the GameEntity...
object GameEntity {

  type Id = UUID

  sealed trait Command extends CborSerializable { val replyTo: ActorRef[Reply] }
  final case class CreateGame(replyTo: ActorRef[Reply]) extends Command
  final case class Join(playerId: PlayerId, replyTo: ActorRef[Reply]) extends Command
  final case class DiscardCribCards(playerId: PlayerId, cardIds: CardIds, replyTo: ActorRef[Reply]) extends Command
  final case class LayCard(playerId: PlayerId, cardId: CardId, replyTo: ActorRef[Reply]) extends Command
  final case class Pass(playerId: PlayerId, replyTo: ActorRef[Reply]) extends Command

  sealed trait Reply extends CborSerializable
  final case class Accepted(command: Command) extends Reply
  final case class Rejected(command: Command, reason: String) extends Reply

  sealed trait Event extends CborSerializable
  final case class GameCreated(id: Id) extends Event
  final case class PlayerJoined(playerId: PlayerId) extends Event
  final case class DealerCutRevealed(playerId: PlayerId, card: Card) extends Event
  final case class DealerSelected(playerId: PlayerId) extends Event
  final case class HandsDealt(hands: Hands, fromDeck: Deck) extends Event
  final case class CribCardsDiscarded(playerId: PlayerId, cardIds: CardIds) extends Event
  final case class PlayCutRevealed(card: Card) extends Event
  final case class CardLaid(playerId: PlayerId, cardId: CardId) extends Event
  final case class Passed(playerId: PlayerId) extends Event
  final case object PlayCompleted extends Event
  final case object PlaysCompleted extends Event
  final case class PoneScored(playerId: PlayerId, points: Points) extends Event
  final case class DealerScored(playerId: PlayerId, points: Points) extends Event
  final case class CribScored(playerId: PlayerId, points: Points) extends Event
  final case object DealerSwapped extends Event
  final case class PointsScored(playerId: PlayerId, points: Int) extends Event
  final case class WinnerDeclared(playerId: PlayerId) extends Event

  sealed trait State extends CborSerializable {
    def applyCommand(command: Command): ReplyEffect[Event, State]
    def applyEvent(event: Event): State
  }

  final case class Idle(id: Id)(implicit log: Logger) extends State {
    override def applyCommand(command: Command): ReplyEffect[Event, State] =
      command match {
        case CreateGame(replyTo) => CreateCommandHandler(id).handle(command)
        case _ => unexpectedCommand(command)
      }

    override def applyEvent(event: Event): State =
      event match {
        case GameCreated(_) => Starting(Game())
        case _ => unexpectedEvent(event)
      }
  }

  final case class Starting(game: Game)(implicit log: Logger) extends State {
    override def applyCommand(command: Command): ReplyEffect[Event, State] =
      command match {
        case join: Join => JoinCommandHandler(join, this).handle(command)
        case _ => unexpectedCommand(command)
      }

    override def applyEvent(event: Event): State =
      event match {
        case PlayerJoined(id) => Starting(game.withPlayer(id))
        case DealerCutRevealed(_, _) => Starting(game)
        case DealerSelected(id) => Starting(game.withDealer(id).withZeroScores())
        case HandsDealt(hands, fromDeck) => Discarding(game.withDeal(hands, fromDeck))
        case _ => unexpectedEvent(game, event)
      }
  }

  final case class Discarding(game: Game)(implicit log: Logger) extends State {
    override def applyCommand(command: Command): ReplyEffect[Event, State] =
      command match {
        case discard: DiscardCribCards => DiscardCribCardsCommandHandler(discard, this).handle(command)
        case _ => unexpectedCommand(command)
      }

    override def applyEvent(event: Event): State =
      event match {
        case CribCardsDiscarded(playerId, cards) => Discarding(game.withCribDiscard(playerId, cards))
        case PlayCutRevealed(cut) => Playing(game.withCut(cut.id).withNextToLay(game.optPone.get))
        case _ => unexpectedEvent(game, event)
      }
  }

  final case class Playing(game: Game)(implicit log: Logger) extends State {
    override def applyCommand(command: Command): ReplyEffect[Event, State] =
      command match {
        case lay: LayCard => LayCardCommandHandler(lay, this).handle(command)
        case pass: Pass => PassCommandHandler(pass, this).handle(command)
        case _ => unexpectedCommand(command)
      }

    override def applyEvent(event: Event): State =
      event match {
        case CardLaid(playerId, cardId) => Playing(game.withLay(playerId, cardId))
        case Passed(playerId) => Playing(game.withPass(playerId))
        case PlayCompleted => Playing(game.withNextPlay())
        case PlaysCompleted => Scoring(game.withPlaysReturned())
        case PointsScored(playerId, points) => Playing(game.withScore(playerId, points))
        case WinnerDeclared(_) => Finished(game)
        case _ => unexpectedEvent(game, event)
      }
  }

  final case class Scoring(game: Game)(implicit log: Logger) extends State {
    override def applyCommand(command: Command): ReplyEffect[Event, State] = unexpectedCommand(command)

    override def applyEvent(event: Event): State =
      event match {
        case PoneScored(playerId, points) => Scoring(game.withScore(playerId, points.total))
        case DealerScored(playerId, points) => Scoring(game.withScore(playerId, points.total))
        case CribScored(playerId, points) => Scoring(game.withScore(playerId, points.total))
        case DealerSwapped => Discarding(game.withSwappedDealer())
        case WinnerDeclared(_) => Finished(game)
        case _ => unexpectedEvent(game, event)
      }

  }

  final case class Finished(game: Game) extends State {
    override def applyCommand(command: Command): ReplyEffect[Event, State] = unexpectedCommand(command)
    override def applyEvent(event: Event): State = this
  }

  def apply(id: Id)(implicit log: Logger): Behavior[Command] =
    apply(PersistenceId.of("game", id.toString), Idle(id))

  private[cribbage] def apply(state: State): Behavior[Command] =
    apply(PersistenceId.ofUniqueId("game-test"), state)

  private def apply(persistenceId: PersistenceId, state: State): Behavior[Command] =
    Behaviors.setup { context =>
      EventSourcedBehavior.withEnforcedReplies[Command, Event, State](
        persistenceId = persistenceId,
        emptyState = state,
        commandHandler = (state: State, command: Command) => state.applyCommand(command),
        eventHandler = (state: State, event: Event) => state.applyEvent(event))
    }

  private def unexpectedCommand(command: Command): ReplyEffect[Event, State] =
    Effect.none.thenReply(command.replyTo)(state => Rejected(command, s"Invalid command $command for state $state"))

  private def unexpectedEvent(event: Event)(implicit log: Logger) = {
    val message = s"Unexpected event $event"
    log.error(message)
    throw new IllegalStateException(message)
  }

  private def unexpectedEvent(game: Game, event: Event)(implicit log: Logger) = {
    val message = s"Unexpected event $event for game $game"
    log.error(message)
    throw new IllegalStateException(message)
  }

}
