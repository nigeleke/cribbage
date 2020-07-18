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

class DiscardingGameSpec
  extends ScalaTestWithActorTestKit(EventSourcedBehaviorTestKit.config)
    with AnyWordSpecLike
    with BeforeAndAfterEach
    with LogCapturing
    with Matchers {

  val gameId = randomId
  val persistenceId = s"game|$gameId"

  implicit val implicitTestKit = testKit

  private val eventSourcedTestKit =
    EventSourcedBehaviorTestKit[Command, Event, State](
      system,
      Game(gameId),
      SerializationSettings.disabled)

  private val persistenceTestKit = eventSourcedTestKit.persistenceTestKit
  private def persisted = persistenceTestKit.persistedInStorage(persistenceId)

  private val deck = Deck()
  private val (player1Id, player2Id) = (randomId, randomId)
  private val initialEvents: Seq[Event] =
    Seq(
      DeckAllocated(deck),
      PlayerJoined(player1Id),
      PlayerJoined(player2Id),
      DealerSelected(player1Id),
      HandDealt(player1Id, deck.take(6)),
      HandDealt(player2Id, deck.drop(6).take(6)),
      HandsDealt)

  override protected def beforeEach(): Unit = {
    super.beforeEach()
    eventSourcedTestKit.clear()
    persistenceTestKit.persistForRecovery(persistenceId, initialEvents)
  }

  def discardingGame(f: model.Game => Unit) = {
    val restart = eventSourcedTestKit.restart()
    val state = restart.state
    state should be(a[Discarding])
    f(state.game)
  }

  "A DiscardingGame" should {

    "allow a player to discard cards into the crib" in discardingGame { game =>
      val playerId = game.players.head
      val discards = game.hands(playerId).take(2)

      val result = eventSourcedTestKit.runCommand(DiscardCribCards(playerId, discards))
      result.command should be(DiscardCribCards(playerId, discards))
      result.event should be(CribCardsDiscarded(playerId, discards))
      result.state.game.hands(playerId) should not contain allElementsOf(discards)
      result.state.game.crib should contain allElementsOf(discards)

      persisted should contain theSameElementsInOrderAs(initialEvents :+ CribCardsDiscarded(playerId, discards))
    }

    "not allow a discard" when {

      "the discard contains cards not owned by the player" in discardingGame { game =>
        val player1Id = game.players.head
        val discards = game.hands(player1Id).take(2)

        val player2Id = game.players.last
        val result = eventSourcedTestKit.runCommand(DiscardCribCards(player2Id, discards))
        result.command should be(DiscardCribCards(player2Id, discards))
        result.events should be(empty)
        result.state.game.hands(player1Id) should contain allElementsOf(discards)
        result.state.game.hands(player2Id) should not contain allElementsOf(discards)
        result.state.game.crib should be(empty)

        persisted should contain theSameElementsInOrderAs(initialEvents)
      }

      "the discard contains too few cards" in discardingGame { game =>
        val playerId = game.players.head
        val discards = game.hands(playerId).take(1)

        val result = eventSourcedTestKit.runCommand(DiscardCribCards(playerId, discards))
        result.command should be(DiscardCribCards(playerId, discards))
        result.events should be(empty)
        result.state.game.hands(playerId) should contain allElementsOf(discards)
        result.state.game.crib should not contain allElementsOf(discards)

        persisted should contain theSameElementsInOrderAs(initialEvents)
      }

      "the discard contains too many cards" in discardingGame { game =>
        val playerId = game.players.head
        val discards = game.hands(playerId).take(3)

        val result = eventSourcedTestKit.runCommand(DiscardCribCards(playerId, discards))
        result.command should be(DiscardCribCards(playerId, discards))
        result.events should be(empty)
        result.state.game.hands(playerId) should contain allElementsOf(discards)
        result.state.game.crib should not contain allElementsOf(discards)

        persisted should contain theSameElementsInOrderAs(initialEvents)
      }

    }

    "start the Lay" when {

      "both Players have discarded" in discardingGame { game =>
        val player1Id = game.players.head
        val discards1 = game.hands(player1Id).take(2)

        val player2Id = game.players.last
        val discards2 = game.hands(player2Id).take(2)

        eventSourcedTestKit.runCommand(DiscardCribCards(player1Id, discards1))
        eventSourcedTestKit.runCommand(DiscardCribCards(player2Id, discards2))
        drain()

        persisted should contain allElementsOf(
          Seq(CribCardsDiscarded(player1Id, discards1),
            CribCardsDiscarded(player2Id, discards2)))

        val isPlayCutRevealed = persisted.last.isInstanceOf[PlayCutRevealed]
        val isPointsScored = persisted.last.isInstanceOf[PointsScored] // Allow for his heels...
        (isPlayCutRevealed || isPointsScored) should be(true)
      }

    }

  }

}
