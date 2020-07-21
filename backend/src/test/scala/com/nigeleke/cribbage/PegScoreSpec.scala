package com.nigeleke.cribbage

import akka.actor.testkit.typed.scaladsl.{LogCapturing, ScalaTestWithActorTestKit}
import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit
import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit.SerializationSettings
import com.nigeleke.cribbage.actors.Game
import com.nigeleke.cribbage.actors.Game._
import com.nigeleke.cribbage.model.Score
import org.scalatest.BeforeAndAfterEach
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpecLike

class PegScoreSpec
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

  import TestEvents._
  private val initialEvents: Seq[Event] =
    deckAllocatedEvent ++ playersJoinedEvents ++ dealerSelectedEvent ++ dealEvents ++ discardEvents ++ playCutEvent

  override protected def beforeEach(): Unit = {
    super.beforeEach()
    eventSourcedTestKit.clear()
    persistenceTestKit.persistForRecovery(persistenceId, initialEvents)
  }

  def playingGame(f: model.Game => Unit) = {
    val restart = eventSourcedTestKit.restart()
    val state = restart.state
    state should be(a[Playing])
    f(state.game)
  }

  "The PegScore command" should {

    "hop the back peg over the front peg to score" in playingGame { game =>
      val command = PegScore(player1Id, 10)

      val result1 = eventSourcedTestKit.runCommand(command)
      result1.command should be(command)
      result1.event should be(PointsScored(player1Id, 10))
      val expectedGame1 = game.withScore(player1Id, 10)
      result1.state should be(Playing(expectedGame1))
      expectedGame1.scores(player1Id) should be(Score(0, 10))

      val result2 = eventSourcedTestKit.runCommand(command)
      result2.command should be(command)
      result2.event should be(PointsScored(player1Id, 10))
      val expectedGame2 = expectedGame1.withScore(player1Id, 10)
      result2.state should be(Playing(expectedGame2))
      expectedGame2.scores(player1Id) should be(Score(10, 20))

      persisted should contain allElementsOf(Seq(PointsScored(player1Id, 10), PointsScored(player1Id, 10)))
    }

    "declare winner" when {

      "player's score exceeds 121" in playingGame { game =>
        val command = PegScore(player1Id, 122)

        val result = eventSourcedTestKit.runCommand(command)
        result.command should be(command)
        result.event should be(PointsScored(player1Id, 122))
        drain()

        persisted should contain allElementsOf(Seq(PointsScored(player1Id, 122), WinnerDeclared(player1Id)))
      }

      "player's score is exactly 121" in playingGame { game =>
        val command = PegScore(player1Id, 121)

        val result = eventSourcedTestKit.runCommand(command)
        result.command should be(command)
        result.event should be(PointsScored(player1Id, 121))
        drain()

        persisted should contain allElementsOf(Seq(PointsScored(player1Id, 121), WinnerDeclared(player1Id)))
      }

    }

    // TODO: Maybe - same for ScoringState ???
  }

}