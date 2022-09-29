//package com.nigeleke.cribbage.server
//
//import com.nigeleke.cribbage.actors.UserService
//import akka.actor.typed.*
//import akka.actor.typed.scaladsl.*
//import akka.actor.typed.scaladsl.AskPattern.Askable
//import akka.http.scaladsl.model.StatusCodes
//import akka.http.scaladsl.server.Directives
//import akka.http.scaladsl.server.Directives.*
//import akka.http.scaladsl.server.directives.Credentials.{Missing, Provided}
//import akka.http.scaladsl.server.directives.Credentials
//import akka.util.Timeout
//import akka.NotUsed
//
//import java.util.UUID
//import scala.concurrent.{ExecutionContext, Future}
//import scala.concurrent.duration.*
//
//object GameServer extends Directives:
//
//  given Timeout = Timeout(5.seconds)
//
//  def basicAuthenticator(
//      credentials: Credentials
//  )(using system: ActorSystem[Nothing], userService: ActorRef[UserService.Command]): Future[Option[String]] =
//    given Scheduler = system.scheduler
//    credentials match
//      case p @ Credentials.Provided(id) => userService.ask(UserService.GetSecret(id, _))
//      case Missing                      => Future.successful(None)
//
//  def authRoutes(using system: ActorSystem[_], userService: ActorRef[UserService.Command]) =
//    given Scheduler = system.scheduler
//    path("auth") {
//      authenticateBasicAsync(realm = "auth", basicAuthenticator) { user =>
//        post {
//          userService.ask(UserService.LoginUser(user, _))
//          complete(StatusCodes.BadRequest)
//        }
//      }
//    }
//
//  def apiRoutes(using system: ActorSystem[_], userService: ActorRef[UserService.Command]) =
//    authenticateBasicAsync(realm = "api", basicAuthenticator) { user =>
//      extractRequestContext { ctx =>
//        path("game") {
//          get {
//            ctx.log.debug(s"GET /game $user ${ctx.request}")
//            complete(???)
//          } ~
//            post {
//              ctx.log.debug(s"POST /game $user ${ctx.request}")
//              complete(???)
//            }
//        }
//      } ~
//        path("game/:id") {
//          get { complete("Ok") }
//        }
//    }
//
//  val routes = extractActorSystem { classicSystem =>
//    import akka.actor.typed.scaladsl.adapter.*
//    given system: ActorSystem[Nothing] = classicSystem.toTyped
//    given ActorRef[UserService.Command] = system.systemActorOf(UserService(), "user-service")
//    authRoutes
//  }
