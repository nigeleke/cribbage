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

package com.nigeleke.cribbage.actors

import akka.actor.typed.scaladsl.Behaviors
import akka.actor.typed.Behavior
import akka.persistence.typed.PersistenceId
import akka.persistence.typed.scaladsl.{Effect, EventSourcedBehavior}
import com.nigeleke.cribbage.actors.handlers._
import com.nigeleke.cribbage.model
import com.nigeleke.cribbage.model.{Card, Cards, Deck, Hands, Points}
import com.nigeleke.cribbage.model.Status.{Id => GameId}
import com.nigeleke.cribbage.model.Player.{Id => PlayerId}
import org.slf4j.Logger

// SRR: Apply actions to the Status...
object Game {

  sealed trait Command
  final case class  Join(playerId: PlayerId) extends Command
  final case class  DiscardCribCards(playerId: PlayerId, cards: Cards) extends Command
  final case class  LayCard(playerId: PlayerId, card: Card) extends Command
  final case class  Pass(playerId: PlayerId) extends Command

  sealed trait Event
  final case class  PlayerJoined(playerId: PlayerId) extends Event
  final case class  DealerCutRevealed(playerId: PlayerId, card: Card) extends Event
  final case class  DealerSelected(playerId: PlayerId) extends Event
  final case class  HandsDealt(hands: Hands, fromDeck: Deck) extends Event
  final case class  CribCardsDiscarded(playerId: PlayerId, cards: Cards) extends Event
  final case class  PlayCutRevealed(card: Card) extends Event
  final case class  CardLaid(playerId: PlayerId, card: Card) extends Event
  final case class  Passed(playerId: PlayerId) extends Event
  final case object PlayCompleted extends Event
  final case object PlaysCompleted extends Event
  final case class  PoneScored(playerId: PlayerId, points: Points) extends Event
  final case class  DealerScored(playerId: PlayerId, points: Points) extends Event
  final case class  CribScored(playerId: PlayerId, points: Points) extends Event
  final case object DealerSwapped extends Event
  final case class  PointsScored(playerId: PlayerId, points: Int) extends Event
  final case class  WinnerDeclared(playerId: PlayerId) extends Event

  sealed trait State {
    def applyCommand(command: Command)(implicit log: Logger) : Effect[Event, State]
    def applyEvent(event: Event)(implicit log: Logger) : State
  }

  final case class Starting(game: model.Status) extends State {
    override def applyCommand(command: Command)(implicit log: Logger) : Effect[Event, State] =
      command match {
        case join: Join => JoinCommandHandler(join, this).handle()
        case _          => unexpectedCommand(game, command)
     }

    override def applyEvent(event: Event)(implicit log: Logger) : State =
      event match {
        case PlayerJoined(id)            => Starting(game.withPlayer(id))
        case DealerCutRevealed(_, _)     => Starting(game)
        case DealerSelected(id)          => Starting(game.withDealer(id).withZeroScores())
        case HandsDealt(hands, fromDeck) => Discarding(game.withDeal(hands, fromDeck))
        case _                           => unexpectedEvent(game, event)
      }
  }

  final case class Discarding(game: model.Status) extends State {
    override def applyCommand(command: Command)(implicit log: Logger) : Effect[Event, State] =
      command match {
        case discard: DiscardCribCards => DiscardCribCardsCommandHandler(discard, this).handle()
        case _                         => unexpectedCommand(game, command)
      }

    override def applyEvent(event: Event)(implicit log: Logger) : State =
      event match {
        case CribCardsDiscarded(playerId, cards) => Discarding(game.withCribDiscard(playerId, cards))
        case PlayCutRevealed(cut)                => Playing(game.withCut(cut).withNextToLay(game.optPone.get))
        case _                                   => unexpectedEvent(game, event)
      }
  }

  final case class Playing(game: model.Status) extends State {
    override def applyCommand(command: Command)(implicit log: Logger) : Effect[Event, State] =
      command match {
        case lay: LayCard => LayCardCommandHandler(lay, this).handle()
        case pass: Pass   => PassCommandHandler(pass, this).handle()
        case _            => unexpectedCommand(game, command)
      }

    override def applyEvent(event: Event)(implicit log: Logger) : State =
      event match {
        case CardLaid(playerId, card)       => Playing(game.withLay(playerId, card))
        case Passed(playerId)               => Playing(game.withPass(playerId))
        case PlayCompleted                  => Playing(game.withNextPlay())
        case PlaysCompleted                 => Scoring(game.withPlaysReturned())
        case PointsScored(playerId, points) => Playing(game.withScore(playerId, points))
        case WinnerDeclared(_)              => Finished(game)
        case _                              => unexpectedEvent(game, event)
      }
  }

  final case class Scoring(game: model.Status) extends State {
    override def applyCommand(command: Command)(implicit log: Logger) : Effect[Event, State] =
      unexpectedCommand(game, command)

    override def applyEvent(event: Event)(implicit log: Logger) : State =
      event match {
        case PoneScored(playerId, points)   => Scoring(game.withScore(playerId, points.total))
        case DealerScored(playerId, points) => Scoring(game.withScore(playerId, points.total))
        case CribScored(playerId, points)   => Scoring(game.withScore(playerId, points.total))
        case DealerSwapped                  => Discarding(game.withSwappedDealer())
        case WinnerDeclared(_)              => Finished(game)
        case _                              => unexpectedEvent(game, event)
      }

  }

  final case class Finished(game: model.Status) extends State {
    override def applyCommand(cmd: Command)(implicit log: Logger) : Effect[Event, State] = Effect.unhandled
    override def applyEvent(event: Event)(implicit log: Logger) : State = this
  }

  def apply(id: GameId) : Behavior[Command] = Game(id, Starting(model.Status(id)))

  private [cribbage] def apply(id: GameId, state: State) : Behavior[Command] = Behaviors.setup { context =>
    implicit val log = context.log
    EventSourcedBehavior[Command, Event, State](
      persistenceId = PersistenceId("game", id.toString),
      emptyState = state,
      commandHandler = (state: State, command: Command) => state.applyCommand(command),
      eventHandler = (state: State, event: Event) => state.applyEvent(event))
  }

  private def unexpectedCommand(game: model.Status, command: Command)(implicit log: Logger): Effect[Event, State] = {
    log.warn(s"Unexpected command [$command] for game [$game]")
    Effect.unhandled
  }

  private def unexpectedEvent(game: model.Status, event: Event)(implicit log: Logger) = {
    val message = s"Unexpected event [$event] for game [$game]"
    log.error(message)
    throw new IllegalStateException(message)
  }

}
