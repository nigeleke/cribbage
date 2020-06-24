package com.nigeleke.cribbage.actors

import java.util.UUID

import akka.actor.typed.eventstream.EventStream.{Publish, Subscribe}
import akka.actor.typed.scaladsl.Behaviors
import akka.actor.typed.{ActorRef, Behavior}
import com.nigeleke.cribbage.actors.states.game.StartingGame
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
  final case class GameCreated(gameId: GameId) extends Event
  final case class PlayerJoined(gameId: GameId, playerId: PlayerId) extends Event
  final case class DealerCutRevealed(gameId: GameId, playerId: PlayerId, card: Card) extends Event
  final case class DealerSelected(gameId: GameId, playerId: PlayerId) extends Event

  def apply(id: GameId) : Behavior[Command] = Behaviors.setup { context =>
    val eventsAdaptor = context.messageAdapter[Event](WrappedEvent(_))
    context.system.eventStream ! Subscribe(eventsAdaptor)

    context.spawn(RuleBook(id), s"rule-book-$id")
    context.system.eventStream ! Publish(GameCreated(id))

    Behaviors.receiveMessage {
      case WrappedEvent(event) => event match {
        case GameCreated(id) =>
          context.spawn(StartingGame(id), s"starting-$id")
          Behaviors.same

        case other =>
          println(s"Unhandled event: $other")
          Behaviors.stopped
      }

      case other =>
        println(s"Unhandled command: $other")
        Behaviors.same
    }
  }
}
