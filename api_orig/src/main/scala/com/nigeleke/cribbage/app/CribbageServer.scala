package com.nigeleke.cribbage.app

import akka.actor.typed.ActorSystem
import akka.actor.typed.scaladsl.Behaviors
import akka.http.scaladsl.Http
import akka.http.scaladsl.server.Route
import com.nigeleke.cribbage.v1.GameServiceRoutes
import com.typesafe.config.ConfigFactory

import scala.util.{ Failure, Success }

object CribbageServer extends App:

  private def startHttpServer(routes: Route, system: ActorSystem[_]) =

    def logStarted(binding: Http.ServerBinding) =
      val address = binding.localAddress
      system.log.info("Server online at http://{}:{}/", address.getHostString, address.getPort)

    def logNotStarted(ex: Throwable) =
      system.log.error("Failed to bind HTTP endpoint, terminating system", ex)
      system.terminate()

    implicit val classicSystem: akka.actor.ActorSystem = system.classicSystem
    import system.executionContext

    val fBinding = Http().newServerAt("localhost", 8080).bindFlow(routes)
    fBinding.onComplete
      case Success(binding) => logStarted(binding)
      case Failure(ex) => logNotStarted(ex)

  implicit val config = ConfigFactory.load()

  val root = Behaviors.setup[Nothing] { context =>
    val routes = new GameServiceRoutes(context.system)
    startHttpServer(routes.routes, context.system)
    Behaviors.empty
  }

  ActorSystem[Nothing](root, "cribbage-server")
