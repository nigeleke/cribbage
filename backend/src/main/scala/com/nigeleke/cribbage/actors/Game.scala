package com.nigeleke.cribbage.actors

import akka.actor.typed.scaladsl.Behaviors
import akka.actor.typed.{ActorRef, Behavior}
import akka.persistence.typed.PersistenceId
import akka.persistence.typed.scaladsl.{Effect, EventSourcedBehavior}
import com.nigeleke.cribbage.actors.handlers.StartingGame
import com.nigeleke.cribbage.model.Card
import com.nigeleke.cribbage.model.Card.{Id => CardId}
import com.nigeleke.cribbage.model.Game.{Id => GameId}
import com.nigeleke.cribbage.model.Player.{Id => PlayerId}

// SRR: Manage the State of a game...
object Game {

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
  private final case class WrappedEvent(event: Event) extends Command

  sealed trait Query extends Command
  final case class Players(replyTo: ActorRef[Set[PlayerId]]) extends Query

  sealed trait Event
  final case class PlayerJoined(playerId: PlayerId) extends Event
  final case class DealerCutRevealed(playerId: PlayerId, card: Card) extends Event
  final case class DealerSelected(playerId: PlayerId) extends Event

  final case class State(ruleBook: ActorRef[RuleBook.Command], commandHandler: ActorRef[Game.Command])

  def apply(id: GameId) : Behavior[Command] = Behaviors.setup { context =>
    val eventsAdaptor = context.messageAdapter[Event](WrappedEvent)

    EventSourcedBehavior[Command, Event, State](
      persistenceId = PersistenceId("game", id.toString),
      emptyState = State(
        context.spawn(RuleBook(eventsAdaptor), s"rule-book-$id"),
        context.spawn(StartingGame(eventsAdaptor), s"starting-$id")),
      commandHandler = onCommand,
      eventHandler = onEvent
    )
  }

  def onCommand(state: State, command: Command) : Effect[Event, State] =
    Effect.none.thenRun { newState => newState.commandHandler ! command}

  def onEvent(state: State, event: Event) : State = ???
}
