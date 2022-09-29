//package com.nigeleke.cribbage.actors
//
//import akka.actor.typed.*
//import akka.actor.typed.scaladsl.Behaviors
//import akka.persistence.typed.scaladsl.EventSourcedBehavior
//import akka.persistence.typed.PersistenceId
//import akka.persistence.typed.scaladsl.Effect
//import akka.persistence.typed.scaladsl.ReplyEffect
//import com.fasterxml.jackson.databind.util.ArrayIterator
//
//import java.time.ZonedDateTime
//import java.util.UUID
//
//object UserService:
//
//  type UserId = String
//  type HashedSecret = String
//
//  sealed trait Command
//  final case class RegisterUser(user: UserCredentials, replyTo: ActorRef[Option[UserId]]) extends Command
//  final case class DeregisterUser(userId: UserId, replyTo: ActorRef[Option[UserId]]) extends Command
//  final case class GetSecret(userId: UserId, replyTo: ActorRef[Option[HashedSecret]]) extends Command
//  final case class LoginUser(userId: UserId, replyTo: ActorRef[Option[AuthToken]]) extends Command
//  final case class LogoutUser(userId: UserId, replyTo: ActorRef[Option[UserId]]) extends Command
//  final case class GetAuthToken(userId: UserId, replyTo: ActorRef[Option[AuthToken]]) extends Command
//
//  sealed trait Event
//  final case class UserRegistered(user: UserCredentials) extends Event
//  final case class UserDeregistered(userId: UserId) extends Event
//  final case class UserLoggedIn(userId: UserId) extends Event
//  final case class UserLoggedOut(userId: UserId) extends Event
//  final case class AuthTokenProvided(userId: UserId) extends Event
//
//  final case class UserCredentials(id: UserId, secret: HashedSecret)
//  final case class AuthToken(accessToken: String = UUID.randomUUID().toString, tokenType: String = "bearer", expiresIn: Int = 3600)
//  final case class AuthTimestamp(token: AuthToken = AuthToken(), loggedIn: ZonedDateTime = ZonedDateTime.now())
//
//  case class State(validUsers: Map[UserId, HashedSecret], loggedInUsers: Map[UserId, AuthTimestamp])
//
//  def apply(): Behavior[Command] =
//    EventSourcedBehavior.withEnforcedReplies(
//      persistenceId = PersistenceId.ofUniqueId("user"),
//      emptyState = State(Map.empty, Map.empty),
//      commandHandler = onCommand,
//      eventHandler = onEvent
//    )
//
//  def onCommand(state: State, command: Command): ReplyEffect[Event, State] =
//    command match
//      case RegisterUser(credentials, replyTo) => registerUser(state, credentials, replyTo)
//      case DeregisterUser(userId, replyTo)    => deregisterUser(state, userId, replyTo)
//      case GetSecret(userId, replyTo)         => getSecret(state, userId, replyTo)
//      case LoginUser(userId, replyTo)         => loginUser(state, userId, replyTo)
//      case LogoutUser(userId, replyTo)        => logoutUser(state, userId, replyTo)
//      case GetAuthToken(userId, replyTo)      => getAuthToken(state, userId, replyTo)
//
//  private def registerUser(
//      state: State,
//      credentials: UserCredentials,
//      replyTo: ActorRef[Option[UserId]]
//  ): ReplyEffect[Event, State] =
//    val maybeValidUser = state.validUsers.get(credentials.id)
//    if maybeValidUser.isEmpty
//    then Effect.persist(UserRegistered(credentials)).thenReply(replyTo)(_ => Some(credentials.id))
//    else Effect.reply(replyTo)(None)
//
//  private def deregisterUser(state: State, userId: UserId, replyTo: ActorRef[Option[UserId]]): ReplyEffect[Event, State] =
//    val isValidUser = state.validUsers.contains(userId)
//    val isLoggedInUser = state.loggedInUsers.contains(userId)
//    if isValidUser && !isLoggedInUser
//    then Effect.persist(UserDeregistered(userId)).thenReply(replyTo)(_ => Some(userId))
//    else Effect.reply(replyTo)(None)
//
//  private def getSecret(state: State, userId: UserId, replyTo: ActorRef[Option[HashedSecret]]): ReplyEffect[Event, State] =
//    Effect.reply(replyTo)(state.validUsers.get(userId))
//
//  private def loginUser(state: State, userId: UserId, replyTo: ActorRef[Option[AuthToken]]): ReplyEffect[Event, State] =
//    val isValidUser = state.validUsers.contains(userId)
//    if isValidUser
//    then Effect.persist(UserLoggedIn(userId)).thenReply(replyTo)(maybeAuthToken(userId))
//    else Effect.reply(replyTo)(None)
//
//  private def maybeAuthToken(userId: UserId) = (state: State) => state.loggedInUsers.get(userId).map(_.token)
//
//  private def logoutUser(state: State, userId: UserId, replyTo: ActorRef[Option[UserId]]): ReplyEffect[Event, State] =
//    val isLoggedInUser = state.loggedInUsers.contains(userId)
//    if isLoggedInUser
//    then Effect.persist(UserLoggedOut(userId)).thenReply(replyTo)(_ => Some(userId))
//    else Effect.reply(replyTo)(None)
//
//  private def getAuthToken(state: State, userId: UserId, replyTo: ActorRef[Option[AuthToken]]): ReplyEffect[Event, State] =
//    val isLoggedInUser = state.loggedInUsers.contains(userId)
//    if isLoggedInUser
//    then Effect.persist(AuthTokenProvided(userId)).thenReply(replyTo)(maybeAuthToken(userId))
//    else Effect.reply(replyTo)(None)
//
//  def onEvent(state: State, event: Event): State =
//    event match
//      case UserRegistered(user)      => recordUserRegistered(state, user)
//      case UserDeregistered(userId)  => recordUserDeegistered(state, userId)
//      case UserLoggedIn(userId)      => recordUserLoggedIn(state, userId)
//      case UserLoggedOut(userId)     => recordUserLoggedOut(state, userId)
//      case AuthTokenProvided(userId) => recordAuthTokenProvided(state, userId)
//
//  private def recordUserRegistered(state: State, credentials: UserCredentials) =
//    state.copy(validUsers = state.validUsers.updated(credentials.id, credentials.secret))
//
//  private def recordUserDeegistered(state: State, userId: UserId) =
//    state.copy(validUsers = state.validUsers - userId)
//
//  private def recordUserLoggedIn(state: State, userId: UserId) =
//    state.copy(loggedInUsers = state.loggedInUsers.updated(userId, AuthTimestamp()))
//
//  private def recordUserLoggedOut(state: State, userId: UserId) =
//    state.copy(loggedInUsers = state.loggedInUsers - userId)
//
//  private def recordAuthTokenProvided(state: State, userId: UserId) =
//    state.copy(loggedInUsers = state.loggedInUsers.updated(userId, AuthTimestamp()))
