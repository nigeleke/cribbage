package com.nigeleke.cribbage.actors

import akka.actor.typed.scaladsl.Behaviors
import akka.actor.typed.{ActorRef, Behavior}
import akka.persistence.typed.PersistenceId
import akka.persistence.typed.scaladsl.{Effect, EventSourcedBehavior}
import com.nigeleke.cribbage.model
import com.nigeleke.cribbage.model.Card
import com.nigeleke.cribbage.model.Card.{Id => CardId}
import com.nigeleke.cribbage.model.Game.{Id => GameId}
import com.nigeleke.cribbage.model.Player.{Id => PlayerId}

// SRR: Apply actions to the Game...
object Game {

  sealed trait Command
  final case class Join(playerId: PlayerId) extends Command
//  final case class DiscardCribCards(playerId: PlayerId, cards: Seq[CardId]) extends Command
//  final case class PlayCard(playerId: PlayerId, cardId: CardId) extends Command
//  final case class Pass(playerId: PlayerId) extends Command
//  final case object CompletePlay extends Command
//  final case object CompletePlays extends Command
//  final case object ScorePoneHand extends Command
//  final case object ScoreDealerHand extends Command
//  final case object ScoreCrib extends Command
//  final case object SwapDealer extends Command
  private final case class WrappedEvent(event: Event) extends Command

  sealed trait Query extends Command
  final case class Players(replyTo: ActorRef[Set[PlayerId]]) extends Query

  sealed trait Event
  final case class PlayerJoined(playerId: PlayerId) extends Event
  final case class DealerCutRevealed(playerId: PlayerId, card: Card) extends Event
  final case class DealerSelected(playerId: PlayerId) extends Event
  final case class HandDealt(playerId: PlayerId, hand: Seq[CardId]) extends Event

  type State = model.Game

  def apply(id: GameId) : Behavior[Command] = Behaviors.setup { context =>
    val eventAdaptor = context.messageAdapter[Event](WrappedEvent)
    val ruleBook = context.spawn(RuleBook(eventAdaptor), s"rule-book-$id")
    val commandHandler = context.spawn(CommandHandler(eventAdaptor), s"command-handler-$id")

    EventSourcedBehavior[Command, Event, State](
      persistenceId = PersistenceId("game", id.toString),
      emptyState = model.Game(id),
      commandHandler = onCommand(ruleBook, commandHandler),
      eventHandler = onEvent)
  }

  def onCommand(ruleBook: ActorRef[Event],
                commandHandler: ActorRef[Command])
               (state: State,
                command: Command) : Effect[Event, State] = {
    println(s"onCommand $command\n $state")
    command match {
      case WrappedEvent(event) => Effect.persist(event).thenRun(_ => ruleBook ! event)
      case other => Effect.none.thenRun(_ => commandHandler ! other)
    }
  }

  def onEvent(state: State, event: Event) : State = {
    println(s"onEvent $event\n from: $state")
    val newState = event match {
      case PlayerJoined(playerId) => state.copy(players = state.players + playerId)
      case DealerCutRevealed(_, _) => state
      case DealerSelected(playerId) => state.copy(optDealer = Some(playerId))

      case other =>
        println(s" >> other >> $other")
        state
    }

    println(s"   to: $newState")
    newState
  }

}
