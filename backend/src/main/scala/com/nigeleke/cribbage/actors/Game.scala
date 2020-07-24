package com.nigeleke.cribbage.actors

import akka.actor.InvalidMessageException
import akka.actor.typed.scaladsl.Behaviors
import akka.actor.typed.{ActorRef, Behavior}
import akka.persistence.typed.PersistenceId
import akka.persistence.typed.scaladsl.{Effect, EventSourcedBehavior}
import com.nigeleke.cribbage.actors.handlers.CommandHandlers._
import com.nigeleke.cribbage.model
import com.nigeleke.cribbage.model.{Card, Cards, Deck, Hand}
import com.nigeleke.cribbage.model.Game.{Id => GameId}
import com.nigeleke.cribbage.model.Player.{Id => PlayerId}

// SRR: Apply actions to the Game...
object Game {

  sealed trait Command
  final case class  Join(playerId: PlayerId) extends Command
  final case object CutForDeal extends Command
  final case object DealHands extends Command
  final case class  DiscardCribCards(playerId: PlayerId, cards: Cards) extends Command
  final case object CutAtStartOfPlay extends Command
  final case class  LayCard(playerId: PlayerId, card: Card) extends Command
  final case class  PegScore(playerId: PlayerId, points: Int) extends Command
  final case class  DeclareWinner(playerId: PlayerId) extends Command
  final case class  Pass(playerId: PlayerId) extends Command
  final case object CompletePlay extends Command
  final case object CompletePlays extends Command
//  final case object ScorePoneHand extends Command
//  final case object ScoreDealerHand extends Command
//  final case object ScoreCrib extends Command
//  final case object SwapDealer extends Command

//  sealed trait Query extends Command
//  final case class GetState(replyTo: ActorRef[model.Game]) extends Query
//  final case class Players(replyTo: ActorRef[Set[PlayerId]]) extends Query

  sealed trait Event
  final case class DeckAllocated(deck: Deck) extends Event
  final case class PlayerJoined(playerId: PlayerId) extends Event
  final case class DealerCutRevealed(playerId: PlayerId, card: Card) extends Event
  final case class DealerSelected(playerId: PlayerId) extends Event
  final case class HandDealt(playerId: PlayerId, hand: Hand) extends Event
  final case object HandsDealt extends Event
  final case class CribCardsDiscarded(playerId: PlayerId, cards: Cards) extends Event
  final case class PlayCutRevealed(card: Card) extends Event
  final case class CardLaid(playerId: PlayerId, card: Card) extends Event
  final case class Passed(playerId: PlayerId) extends Event
  final case object PlayCompleted extends Event
  final case object PlaysCompleted extends Event
  final case class PointsScored(playerId: PlayerId, points: Int) extends Event
  final case class WinnerDeclared(playerId: PlayerId) extends Event

  sealed trait State { def game: model.Game }
  final case class Uninitialised(game: model.Game) extends State
  final case class Starting(game: model.Game) extends State
  final case class Discarding(game: model.Game) extends State
  final case class Playing(game: model.Game) extends State
  final case class Scoring(game: model.Game) extends State
  final case class Finished(game: model.Game) extends State

  def apply(id: GameId) : Behavior[Command] = Behaviors.setup { context =>
    implicit val notify = context.self
    EventSourcedBehavior[Command, Event, State](
      persistenceId = PersistenceId("game", id.toString),
      emptyState = Uninitialised(model.Game(id)),
      commandHandler = onCommand,
      eventHandler = onEvent)
  }

  def onCommand(state: State, command: Command)(implicit notify: ActorRef[Command]) : Effect[Event, State] = {
    println(s"Command: $command")
    state match {
      case Uninitialised(game) => uninitialisedCommands(game, command)
      case Starting(game)      => handleStartingCommands(game, command)
      case Discarding(game)    => handleDiscardingCommands(game, command)
      case Playing(game)       => handlePlayingCommands(game, command)
      case Scoring(game)       => handleScoringCommands(game, command)
      case Finished(_)         => ???
    }
  }

  private def uninitialisedCommands(game: model.Game, command: Command)(implicit notify: ActorRef[Command]) : Effect[Event, State] = {
    initialise(game).thenRun(_ => notify ! command)
  }

  private def handleStartingCommands(game: model.Game, command: Command)(implicit notify: ActorRef[Command]) : Effect[Event, State] = {
    command match {
      case Join(playerId) => join(game, playerId)
      case CutForDeal     => cutForDeal(game)
      case DealHands      => dealHands(game)
      case _              => unexpectedCommand(game, command)
    }
  }

  private def handleDiscardingCommands(game: model.Game, command: Command)(implicit notify: ActorRef[Command]) : Effect[Event, State] =
    command match {
      case DiscardCribCards(playerId, cards) => discardCribCards(game, playerId, cards)
      case CutAtStartOfPlay                  => cutAtStartOfPlay(game)
      case _                                 => unexpectedCommand(game, command)
    }

  private def handlePlayingCommands(game: model.Game, command: Command)(implicit notify: ActorRef[Command]) : Effect[Event, State] =
    command match {
      case LayCard(playerId, card)    => layCard(game, playerId, card)
      case Pass(playerId)             => pass(game, playerId)
      case CompletePlay               => completePlay(game)
      case CompletePlays              => completePlays(game)
      case PegScore(playerId, points) => pegScore(playerId, points)
      case DeclareWinner(playerId)    => declareWinner(playerId)
      case _                          => unexpectedCommand(game, command)
    }

  private def handleScoringCommands(game: model.Game, command: Command)(implicit notify: ActorRef[Command]) : Effect[Event, State] =
    command match {
      case PegScore(playerId, points) => pegScore(playerId, points)
      case _                          => unexpectedCommand(game, command)
    }

  private def unexpectedCommand(game: model.Game, command: Command) =
    throw InvalidMessageException(s"Unexpected command [$command] for game [$game]")

  def onEvent(state: State, event: Event) : State = {
    println(s"Event: $event")
    state match {
      case Uninitialised(game) => handleUninitialisedEvents(game, event)
      case Starting(game)      => handleStartingEvents(game, event)
      case Discarding(game)    => handleDiscardEvents(game, event)
      case Playing(game)       => handlePlayingEvents(game, event)
      case Scoring(game)       => handleScoringEvents(game, event)
      case Finished(_)         => ???
    }
  }

  private def handleUninitialisedEvents(game: model.Game, event: Event) : State = {
    event match {
      case DeckAllocated(deck) => Starting(game.withDeck(deck))
      case _                   => unexpectedEvent(game, event)
    }
  }

  private def handleStartingEvents(game: model.Game, event: Event) : State = {
    event match {
      case PlayerJoined(id)        => Starting(game.withPlayer(id))
      case DealerCutRevealed(_, _) => Starting(game)
      case DealerSelected(id)      => Starting(game.withDealer(id))
      case HandDealt(id, hand)     => Starting(game.withHand(id, hand))
      case HandsDealt              => Discarding(game)
      case _                       => unexpectedEvent(game, event)
    }
  }

  private def handleDiscardEvents(game: model.Game, event: Event) : State = {
    event match {
      case CribCardsDiscarded(playerId, cards) => Discarding(game.withCribDiscard(playerId, cards))
      case PlayCutRevealed(cut)                => Playing(game.withCut(cut).withNextToLay(game.optPone.get))
      case _                                   => unexpectedEvent(game, event)
    }
  }

  private def handlePlayingEvents(game: model.Game, event: Event) : State = {
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

  private def handleScoringEvents(game: model.Game, event: Event) : State = {
    event match {
      case PointsScored(playerId, points) => Scoring(game.withScore(playerId, points))
      case WinnerDeclared(_)              => Finished(game)
      case _                              => unexpectedEvent(game, event)
    }
  }

  private def unexpectedEvent(game: model.Game, event: Event) =
    throw new IllegalStateException(s"Unexpected event [$event] for game [$game]")

}
