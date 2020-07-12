package com.nigeleke.cribbage

import akka.actor.testkit.typed.scaladsl.{LogCapturing, ScalaTestWithActorTestKit}
import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit
import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit.SerializationSettings
import com.nigeleke.cribbage.actors.Game
import com.nigeleke.cribbage.actors.Game._
import com.nigeleke.cribbage.model.Deck
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

  implicit val implicitTestKit = testKit

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

    "start with a new Deck of Cards" in {
      val playerId = randomId

      eventSourcedTestKit.runCommand(Join(playerId))
      drain()

      persisted.head should be(a[DeckAllocated])
      persisted.tail should contain theSameElementsInOrderAs(Seq(PlayerJoined(playerId)))

      val result = eventSourcedTestKit.restart()
      result.state.game.deck.map(card => (card.face, card.suit)) should contain
        theSameElementsInOrderAs(Deck().map(card => (card.face, card.suit)))
    }

    "allow a new player to join" in {
      val playerId = randomId

      eventSourcedTestKit.runCommand(Join(playerId))
      drain()

      persisted.head should be(a[DeckAllocated])
      persisted.tail should contain theSameElementsInOrderAs(Seq(PlayerJoined(playerId)))

      val result = eventSourcedTestKit.restart()
      result.state.game.players should contain(playerId)
    }

    "allow two new players to join" in {
      val (player1Id, player2Id) = (randomId, randomId)

      eventSourcedTestKit.runCommand(Join(player1Id))
      eventSourcedTestKit.runCommand(Join(player2Id))
      drain()

      persisted.tail should contain
        theSameElementsInOrderAs(Seq(PlayerJoined(player1Id), PlayerJoined(player2Id)))

      val result = eventSourcedTestKit.restart()
      result.state.game.players should contain allElementsOf(Seq(player1Id, player2Id))
    }

    "not allow three players to join" in {
      val (player1Id, player2Id, player3Id) = (randomId, randomId, randomId)

      eventSourcedTestKit.runCommand(Join(player1Id))
      eventSourcedTestKit.runCommand(Join(player2Id))

      val result = eventSourcedTestKit.runCommand(Join(player3Id))
      result.command should be(Join(player3Id))
      result.state.game.players should contain theSameElementsAs(Seq(player1Id, player2Id))

      persisted.tail should not contain(PlayerJoined(player3Id))
    }

    "not allow the same player to join twice" in {
      val playerId = randomId
      eventSourcedTestKit.runCommand(Join(playerId))
      drain()

      val result = eventSourcedTestKit.runCommand(Join(playerId))
      result.command should be(Join(playerId))
      result.events should not contain(PlayerJoined(playerId))
      result.state.game.players should contain theSameElementsAs(Seq(playerId))

      persisted.count(_ == PlayerJoined(playerId)) should be(1)
    }

    "deal hands" when {
      "two players have joined" in {
        val (player1Id, player2Id) = (randomId, randomId)

        eventSourcedTestKit.runCommand(Join(player1Id))
        eventSourcedTestKit.runCommand(Join(player2Id))
        drain()

        persisted.count(_.isInstanceOf[HandsDealt.type]) should be(1)
      }
    }

    "remove cards from deck" when {
      "players cards have in dealt" in {
        val (player1Id, player2Id) = (randomId, randomId)

        eventSourcedTestKit.runCommand(Join(player1Id))
        eventSourcedTestKit.runCommand(Join(player2Id))
        drain()

        val result = eventSourcedTestKit.restart()
        val state = result.state
        val game = state.game
        game.players.foreach { id =>
          game.hands(id).size should be(6)
          game.deck should not contain allElementsOf(game.hands(id))
        }
        game.deck.size should be(40)
      }
    }
  }

}
