package com.nigeleke.cribbage.actors

import akka.NotUsed
import akka.actor.typed.ActorSystem
import akka.persistence.jdbc.query.scaladsl.JdbcReadJournal
import akka.persistence.query.PersistenceQuery
import akka.stream.scaladsl.Source

class GameJournal(implicit system: ActorSystem[_]) {

  private val queries = PersistenceQuery(system).readJournalFor[JdbcReadJournal](JdbcReadJournal.Identifier)

  val currentGames: Source[String, NotUsed] = queries.currentPersistenceIds()
  val games: Source[String, NotUsed] = queries.persistenceIds()

}
