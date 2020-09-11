package com.nigeleke.cribbage.entity

import akka.NotUsed
import akka.actor.typed.ActorSystem
import akka.persistence.query.PersistenceQuery
import akka.persistence.query.scaladsl._
import akka.stream.scaladsl.Source
import com.typesafe.config.Config

class GameJournal(implicit system: ActorSystem[_], config: Config) {

  type Journal = ReadJournal with CurrentPersistenceIdsQuery with PersistenceIdsQuery with CurrentEventsByPersistenceIdQuery with CurrentEventsByTagQuery with EventsByPersistenceIdQuery with EventsByTagQuery

  private val readJournal = config.getString("cribbage.read-journal")

  private val queries = PersistenceQuery(system).readJournalFor[Journal](readJournal)

  val currentGames: Source[String, NotUsed] = queries.currentPersistenceIds()
  val games: Source[String, NotUsed] = queries.persistenceIds()

}

object GameJournal {
  def apply()(implicit system: ActorSystem[_], config: Config): GameJournal = new GameJournal()
}