package com.nigeleke.cribbage

import akka.http.scaladsl.server.Directives._
import akka.http.scaladsl.server.Route

import akka.actor.typed.ActorRef
import akka.actor.typed.ActorSystem
import akka.util.Timeout
import com.nigeleke.cribbage.entity.GameEntity._

class GameServiceRoutes(gameSupervisor: ActorRef[Command])(implicit val system: ActorSystem[_])
  extends JsonFormats {

  import akka.http.scaladsl.marshallers.sprayjson.SprayJsonSupport._

  private implicit val timeout = Timeout.create(system.settings.config.getDuration("cribbage.routes.ask-timeout"))

  //  def getGames(): Future[Games] = gameSupervisor.ask(GetGames)
  //
  val routes: Route =
    pathPrefix("games") {
      //      pathEnd { get { complete(getGames()) } }
      pathEnd { get { complete("done") } }
    }

}
