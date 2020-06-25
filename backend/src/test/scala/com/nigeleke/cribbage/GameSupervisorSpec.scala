package com.nigeleke.cribbage

import java.util.UUID

import akka.actor.testkit.typed.scaladsl.ScalaTestWithActorTestKit
import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit
import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit.SerializationSettings
import com.nigeleke.cribbage.actors.GameSupervisor
import com.nigeleke.cribbage.actors.GameSupervisor._
import org.scalatest.BeforeAndAfterEach
import org.scalatest.matchers.should
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpecLike

class GameSupervisorSpec
  extends ScalaTestWithActorTestKit(EventSourcedBehaviorTestKit.config)
    with AnyWordSpecLike
    with BeforeAndAfterEach
    with Matchers {

  private val eventSourcedTestKit =
    EventSourcedBehaviorTestKit[Command, Event, State](
      system,
      GameSupervisor(),
      SerializationSettings.disabled)

  override protected def beforeEach(): Unit = {
    super.beforeEach()
    eventSourcedTestKit.clear()
  }

  "A GameSupervisor" should {

    "maintain a list of Games" when {
      "no Games have been created" in {
        val result = eventSourcedTestKit.runCommand(GetGames)
        result.reply should be(Games(Set.empty))
        result.events should be(empty)
        result.state should be(State(Set.empty))
      }

      "a new Game is created" in {
        val gameId = UUID.randomUUID()
        val result = eventSourcedTestKit.runCommand(CreateGame(gameId))
        result.events should contain allElementsOf(Seq(GameCreated(gameId)))
        result.state should be(State(Set(gameId)))

        val result2 = eventSourcedTestKit.runCommand(GetGames)
        result2.reply should be(Games(Set(gameId)))
        result2.events should be(empty)
        result2.state should be(State(Set(gameId)))
      }

      "a Game is added more than once" in {
        val gameId = UUID.randomUUID()
        val results = Seq(CreateGame(gameId), CreateGame(gameId)).map(eventSourcedTestKit.runCommand)
        results.flatMap(_.events) should contain theSameElementsInOrderAs(Seq(GameCreated(gameId)))
        results.last.state should be(State(Set(gameId)))

        val result2 = eventSourcedTestKit.runCommand(GetGames)
        result2.reply should be(Games(Set(gameId)))
        result2.events should be(empty)
        result2.state should be(State(Set(gameId)))
      }

    }

  }

}