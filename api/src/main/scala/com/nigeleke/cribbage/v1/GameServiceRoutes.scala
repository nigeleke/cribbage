//package com.nigeleke.cribbage.v1

//import akka.http.scaladsl.server.Directives._
//import akka.http.scaladsl.server.Route
//import akka.actor.typed.ActorSystem
//import akka.cluster.sharding.typed.scaladsl.EntityRef
//import akka.util.Timeout
//import com.nigeleke.cribbage.entity.GameEntity.Command
//
//class GameServiceRoutes(entity: EntityRef[Command])(implicit val system: ActorSystem[_])
//  extends JsonFormats {
//
//  private implicit val timeout = Timeout.create(system.settings.config.getDuration("cribbage.routes.ask-timeout"))
//
//  //  def getGames(): Future[Games] = gameSupervisor.ask(GetGames)
//  //
//  val routes: Route =
//    pathPrefix("games") {
//      //      pathEnd { get { complete(getGames()) } }
//      pathEnd { get { complete("done") } }
//    }
//
//}
