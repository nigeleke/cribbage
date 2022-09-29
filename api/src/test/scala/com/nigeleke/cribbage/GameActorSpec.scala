//package com.nigeleke.cribbage
//
//import com.nigeleke.cribbage.actors.*
//
//import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit
//import akka.persistence.typed.PersistenceId
//import akka.actor.testkit.typed.scaladsl.ActorTestKit
//import akka.actor.testkit.typed.scaladsl.ScalaTestWithActorTestKit
//import org.scalatest.*
//import org.scalatest.matchers.should.Matchers
//import org.scalatest.wordspec.AnyWordSpecLike
//
//class GameActorSpec
//    extends ScalaTestWithActorTestKit(EventSourcedBehaviorTestKit.config)
//    with AnyWordSpecLike
//    with Matchers
//    with BeforeAndAfterAll
//    with BeforeAndAfterEach
//
////  private val eventSourcedTestKit: EventSourcedBehaviorTestKit[GameService.Command, GameService.Event, GameService.State] = ???
