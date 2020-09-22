package com.nigeleke.cribbage.app

import akka.actor.typed.ActorSystem
import akka.actor.typed.scaladsl.Behaviors

object CribbageApp extends App {

  //  private def startHttpServer(routes: Route, system: ActorSystem[_]): Unit = {
  //
  //    def logStarted(binding: Http.ServerBinding) = {
  //      val address = binding.localAddress
  //      system.log.info("Server online at http://{}:{}/", address.getHostString, address.getPort)
  //    }
  //
  //    def logNotStarted(ex: Throwable) = {
  //      system.log.error("Failed to bind HTTP endpoint, terminating system", ex)
  //      system.terminate()
  //    }

  //    implicit val classicSystem: akka.actor.ActorSystem = system
  //    import system.executionContext

  //    val fBinding = Http().newServerAt("localhost", 8080).bindFlow(routes)
  //    fBinding.onComplete {
  //      case Success(binding) => logStarted(binding)
  //      case Failure(ex) => logNotStarted(ex)
  //    }
  //  }

  val root = Behaviors.setup[Nothing] { context =>
    //      implicit val log = context.log
    //
    //      val gameId = UUID.randomUUID()
    //      val gameSupervisor = context.spawn(GameEntity(gameId), "game")
    //      context.watch(gameSupervisor)
    //
    //      val routes = new GameServiceRoutes(gameSupervisor)(context.system)
    //      startHttpServer(routes.routes, context.system)
    //
    Behaviors.empty
  }

  ActorSystem[Nothing](root, "cribbage-server")

}
