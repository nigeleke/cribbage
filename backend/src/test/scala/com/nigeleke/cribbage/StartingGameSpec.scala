package com.nigeleke.cribbage

import akka.actor.testkit.typed.scaladsl.{LogCapturing, ScalaTestWithActorTestKit}
import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit
import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit.SerializationSettings
import com.nigeleke.cribbage.actors.Game
import com.nigeleke.cribbage.actors.Game._
import org.scalatest.BeforeAndAfterEach
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpecLike

class StartingGameSpec
  extends ScalaTestWithActorTestKit(EventSourcedBehaviorTestKit.config)
    with AnyWordSpecLike
    with BeforeAndAfterEach
    with LogCapturing
    with Matchers {

  private val gameId = randomId
  private val persistenceId = s"game|$gameId"

  val eventSourcedTestKit =
    EventSourcedBehaviorTestKit[Command, Event, State](
      system,
      Game(gameId),
      SerializationSettings.disabled)

  private val persistenceTestKit = eventSourcedTestKit.persistenceTestKit
  private def persisted = persistenceTestKit.persistedInStorage(persistenceId)

  override protected def beforeEach(): Unit = {
    super.beforeEach()
    eventSourcedTestKit.clear()
    persistenceTestKit.clearAll()
  }

  override protected def afterEach(): Unit = {
    super.afterEach()
    testKit.createTestProbe().expectNoMessage() // drain Commands...
  }

  "A Starting Game" should {

    "allow a new player to join" in {
      val playerId = randomId

      val result = eventSourcedTestKit.runCommand(Join(playerId))
      result.command should be(Join(playerId))
      result.event should be(PlayerJoined(playerId))
      result.state should be(Starting(model.Game(gameId).withPlayer(playerId)))

      persisted should contain theSameElementsInOrderAs(Seq(PlayerJoined(playerId)))
    }

    "allow two new players to join" in {
      val (player1Id, player2Id) = (randomId, randomId)
      eventSourcedTestKit.runCommand(Join(player1Id))

      val result = eventSourcedTestKit.runCommand(Join(player2Id))
      result.command should be(Join(player2Id))
      result.event should be(PlayerJoined(player2Id))
      result.state should be(Starting(model.Game(gameId).withPlayer(player1Id).withPlayer(player2Id)))

      persisted should contain theSameElementsInOrderAs(Seq(PlayerJoined(player1Id), PlayerJoined(player2Id)))
    }

    "not allow three players to join" in {
      val (player1Id, player2Id, player3Id) = (randomId, randomId, randomId)
      eventSourcedTestKit.runCommand(Join(player1Id))
      eventSourcedTestKit.runCommand(Join(player2Id))

      val result = eventSourcedTestKit.runCommand(Join(player3Id))
      result.command should be(Join(player3Id))
      result.events should not contain(PlayerJoined(player3Id))
      result.state.game.players should contain theSameElementsAs(Seq(player1Id, player2Id))

      persisted should not contain(PlayerJoined(player3Id))
    }

    "not allow the same player to join twice" in {
      val playerId = randomId
      eventSourcedTestKit.runCommand(Join(playerId))

      val result = eventSourcedTestKit.runCommand(Join(playerId))
      result.command should be(Join(playerId))
      result.events should not contain(PlayerJoined(playerId))
      result.state should be(Starting(model.Game(gameId).withPlayer(playerId)))

      persisted.count(_ == PlayerJoined(playerId)) should be(1)
    }

    "deal hands" when {
      "two players have joined" in {
        val (player1Id, player2Id) = (randomId, randomId)
        eventSourcedTestKit.runCommand(Join(player1Id))
        eventSourcedTestKit.runCommand(Join(player2Id))

        testKit.createTestProbe().expectNoMessage() // Allow knock-on events to be processed
        persisted.count(_.isInstanceOf[HandsDealt.type]) should be(1)
      }
    }
  }

}
