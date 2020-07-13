package com.nigeleke.cribbage

import akka.actor.testkit.typed.scaladsl.{LogCapturing, ScalaTestWithActorTestKit}
import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit
import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit.SerializationSettings
import com.nigeleke.cribbage.actors.Game
import com.nigeleke.cribbage.actors.Game._
import com.nigeleke.cribbage.model.{Deck, Score}
import com.nigeleke.cribbage.model.Deck._
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

  private val deck = Deck()
  private val deckIds = deck.ids
  private val (player1Id, player2Id) = (randomId, randomId)
  private val initialEvents: Seq[Event] =
    Seq(
      DeckAllocated(deck),
      PlayerJoined(player1Id),
      PlayerJoined(player2Id),
      DealerSelected(player1Id),
      HandDealt(player1Id, deckIds.take(6)),
      HandDealt(player2Id, deckIds.drop(6).take(6)),
      HandsDealt,
      CribCardsDiscarded(player1Id, deckIds.take(2)),
      CribCardsDiscarded(player2Id, deckIds.drop(6).take(2)),
      PlayCutRevealed(deck.drop(12).take(1).head))

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