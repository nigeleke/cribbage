////package com.nigeleke.cribbage.actors
//
//import com.nigeleke.cribbage.json.*
//import akka.actor.typed.*
//import akka.actor.typed.scaladsl.Behaviors
//import akka.http.scaladsl.model.DateTime
//import akka.persistence.typed.scaladsl.{Effect, EventSourcedBehavior}
//import akka.persistence.typed.PersistenceId
//
//import java.util.UUID
//
//object GameService:
//  import JsonProtocol.*
//
//  sealed trait Command
//  case object CreateGame extends Command
//
//  sealed trait Event
//  final case class GameCreated(ref: ActorRef[GameActor.Command], timestamp: DateTime) extends Event
//
//  final case class State(games: Set[ActorRef[GameActor.Command]])
//
//  def apply(): Behavior[Command] =
//    Behaviors.setup { context =>
//      EventSourcedBehavior[Command, Event, State](
//        persistenceId = PersistenceId.ofUniqueId("game-repository"),
//        emptyState = State(Set.empty),
//        commandHandler = onCommand(context),
//        eventHandler = onEvent
//      )
//    }
//
//  def onCommand(context: TypedActorContext[Command])(state: State, command: Command): Effect[Event, State] =
//    command match
//      case CreateGame =>
//        val gameRef = context.asScala.spawnAnonymous(GameActor())
//        Effect.persist(GameCreated(gameRef, DateTime.now))
//
////      case GetGames(replyTo: ActorRef[Games]) => ???
////        Effect.reply(replyTo)(Games(state.games))
//
//  def onEvent(state: State, event: Event): State =
//    event match
//      case GameCreated(ref, date) =>
//        state.copy(state.games + ref)
