package com.nigeleke.cribbage.v1

import akka.http.scaladsl.server.Directives._
import akka.http.scaladsl.server.Route
import akka.actor.typed.ActorSystem
import akka.util.Timeout
import com.nigeleke.cribbage.services.{ GameJournal, GameService }
import com.typesafe.config.Config

class GameServiceRoutes(system: ActorSystem[_])(implicit config: Config) {

  private implicit val timeout = Timeout.create(system.settings.config.getDuration("cribbage.routes.ask-timeout"))

  val service = new GameService(system)
  val journal = new GameJournal(system)

  def getGames() = journal.currentGames

  /*
  val eventsRoute =
  path("events" / uuidRegex) { persistenceId =>
    onSuccess {
      queries.currentEventsByPersistenceId(persistenceId, 0L, Long.MaxValue)
        .map(_.event)
        .runWith(Sink.seq)
        .mapTo[Seq[Event]]
    } { events =>
      complete(events)
    }
  }
   */

  val routes: Route = {
    pathPrefix("api" / "v1") {
      pathPrefix("games") {
        pathEndOrSingleSlash { complete("ok") }
      }
    }
  }

}
