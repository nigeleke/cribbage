package com.nigeleke.cribbage.services

import java.util.UUID

import akka.NotUsed
import akka.actor.typed.ActorSystem
import akka.persistence.query.{ EventEnvelope, PersistenceQuery }
import akka.persistence.query.scaladsl._
import akka.stream.scaladsl.Source
import com.nigeleke.cribbage.entity.GameEntity.{ Id => GameId }
import com.typesafe.config.Config

import scala.language.implicitConversions

class GameJournal(system: ActorSystem[_])(implicit config: Config) {

  type Journal = ReadJournal with CurrentPersistenceIdsQuery with PersistenceIdsQuery with CurrentEventsByPersistenceIdQuery with CurrentEventsByTagQuery with EventsByPersistenceIdQuery with EventsByTagQuery

  private val readJournal = config.getString("cribbage.read-journal")

  private val queries = PersistenceQuery(system).readJournalFor[Journal](readJournal)

  val currentGames: Source[GameId, NotUsed] = queries.currentPersistenceIds().map(UUID.fromString)
  val games: Source[GameId, NotUsed] = queries.persistenceIds().map(UUID.fromString)

  private implicit def uuidToString(id: UUID): String = id.toString

  def currentGameEvents(gameId: GameId): Source[EventEnvelope, NotUsed] =
    queries.currentEventsByPersistenceId(gameId, 0L, Long.MaxValue)

  def gameEvents(gameId: GameId): Source[EventEnvelope, NotUsed] =
    queries.eventsByPersistenceId(gameId, 0L, Long.MaxValue)

}
