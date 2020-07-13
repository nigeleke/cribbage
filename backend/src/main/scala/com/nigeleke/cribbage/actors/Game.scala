package com.nigeleke.cribbage.actors

import akka.actor.InvalidMessageException
import akka.actor.typed.scaladsl.Behaviors
import akka.actor.typed.{ActorRef, Behavior}
import akka.persistence.typed.PersistenceId
import akka.persistence.typed.scaladsl.{Effect, EventSourcedBehavior}
import com.nigeleke.cribbage.actors.handlers._
import com.nigeleke.cribbage.actors.rules._
import com.nigeleke.cribbage.model
import com.nigeleke.cribbage.model.{Card, Cards, Deck, Hand}
import com.nigeleke.cribbage.model.Game.{Id => GameId}
import com.nigeleke.cribbage.model.Player.{Id => PlayerId}

// SRR: Apply actions to the Game...
object Game {

  sealed trait Command
  final case class Join(playerId: PlayerId) extends Command
  final case object CutForDeal extends Command
  final case object DealHands extends Command
  final case class DiscardCribCards(playerId: PlayerId, cards: Cards) extends Command
  final case object CutAtStartOfPlay extends Command
  final case class PegScore(playerId: PlayerId, points: Int) extends Command
  final case class DeclareWinner(playerId: PlayerId) extends Command
//  final case class PlayCard(playerId: PlayerId, cardId: CardId) extends Command
//  final case class Pass(playerId: PlayerId) extends Command
//  final case object CompletePlay extends Command
//  final case object CompletePlays extends Command
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
  final case class PointsScored(playerId: PlayerId, points: Int) extends Event
  final case class WinnerDeclared(playerId: PlayerId) extends Event

  sealed trait State { def game: model.Game }
  final case class Uninitialised(game: model.Game) extends State
  final case class Starting(game: model.Game) extends State
  final case class Discarding(game: model.Game) extends State
  final case class Playing(game: model.Game) extends State
  final case class Finished(game: model.Game) extends State

  def apply(id: GameId) : Behavior[Command] = Behaviors.setup { context =>
    implicit val notify = context.self
    EventSourcedBehavior[Command, Event, State](
      persistenceId = PersistenceId("game", id.toString),
      emptyState = Uninitialised(model.Game(id)),
      commandHandler = onCommand,
      eventHandler = onEvent)
  }

  def onCommand(state: State, command: Command)(implicit notify: ActorRef[Command]) : Effect[Event, State] =
    state match {
      case Uninitialised(_) => InitialisationHandler(state).thenRun(_ => notify ! command)
      case Starting(_)      => handleStartingCommands(state, command)
      case Discarding(_)    => handleDiscardingCommands(state, command)
      case Playing(_)       => handlePlayingCommands(state, command)
    }

  private def handleStartingCommands(state: State, command: Command)(implicit notify: ActorRef[Command]) : Effect[Event, State] = {
    command match {
      case join: Join => JoinHandler(state, join).thenRun(CutForDealRule(_))
      case CutForDeal => CutForDealHandler(state).thenRun(DealRule(_))
      case DealHands  => DealHandsHandler(state)
      case _          => unexpectedCommand(state, command)
    }
  }

  private def handleDiscardingCommands(state: State, command: Command)(implicit notify: ActorRef[Command]) : Effect[Event, State] = {
    command match {
      case discard: DiscardCribCards => DiscardCribCardsHandler(state, discard).thenRun(CutAtStartOfPlayRule(_))
      case CutAtStartOfPlay          => CutAtStartOfPlayHandler(state).thenRun(ScoreCutAtStartOfPlayRule(_))
      case _                         => unexpectedCommand(state, command)
    }
  }

  private def handlePlayingCommands(state: State, command: Command)(implicit notify: ActorRef[Command]) : Effect[Event, State] = {
    command match {
      case peg: PegScore         => PegScoreHandler(state, peg).thenRun(DeclareWinnerRule(_))
      case winner: DeclareWinner => DeclareWinnerHandler(state, winner)
      case _                     => unexpectedCommand(state, command)
    }
  }

  private def unexpectedCommand(state: State, command: Command) =
    throw InvalidMessageException(s"Unexpected command [$command] in state [$state]")

  def onEvent(state: State, event: Event) : State =
    state match {
      case Uninitialised(game) => handleUninitialisedEvents(state, event, game)
      case Starting(game)      => handleStartingEvents(state, event, game)
      case Discarding(game)    => handleDiscardEvents(state, event, game)
      case Playing(game)       => handlePlayingEvents(state, event, game)
    }

  private def handleUninitialisedEvents(state: State, event: Event, game: model.Game) : State = {
    event match {
      case DeckAllocated(deck) => Starting(game.withDeck(deck))
      case _                   => illegalState(state, event)
    }
  }

  private def handleStartingEvents(state: State, event: Event, game: model.Game) : State = {
    event match {
      case PlayerJoined(id)        => Starting(game.withPlayer(id))
      case DealerCutRevealed(_, _) => state
      case DealerSelected(id)      => Starting(game.withDealer(id))
      case HandDealt(id, hand)     => Starting(game.withHand(id, hand))
      case HandsDealt              => Discarding(game)
      case _                       => illegalState(state, event)
    }
  }

  private def handleDiscardEvents(state: State, event: Event, game: model.Game) : State = {
    event match {
      case CribCardsDiscarded(playerId, cards) => Discarding(game.withCribDiscard(playerId, cards))
      case PlayCutRevealed(cut)                => Playing(game.withCut(cut))
      case _                                   => illegalState(state, event)
    }
  }

  private def handlePlayingEvents(state: State, event: Event, game: model.Game) : State = {
    event match {
      case PointsScored(playerId, points) => Playing(game.withScore(playerId, points))
      case WinnerDeclared(_)              => Finished(game)
      case _                              => illegalState(state, event)
    }
  }

  private def illegalState(state: State, event: Event) =
    throw new IllegalStateException(s"Unexpected event [$event] in state [$state]")

}
