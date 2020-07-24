package com.nigeleke.cribbage

import akka.actor.testkit.typed.scaladsl.{LogCapturing, ScalaTestWithActorTestKit}
import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit
import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit.SerializationSettings
import com.nigeleke.cribbage.actors.Game
import com.nigeleke.cribbage.actors.Game._
import com.nigeleke.cribbage.suit.Face._
import com.nigeleke.cribbage.suit.Suit._
import org.scalatest.BeforeAndAfterEach
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpecLike

class ScoringGameSpec
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
    deckAllocatedEvent ++ playersJoinedEvents ++ dealerSelectedEvent ++
      dealEventsWith(Seq(
        (Ten,Hearts), (Ten,Clubs), (Ten,Diamonds), (Ten,Spades), (Five,Hearts), (Four,Clubs),
        (King,Hearts), (King,Clubs), (King,Diamonds), (King,Spades), (Eight,Diamonds), (Seven,Spades))) ++
      discardEventsWith(Seq(
        (player1Id, Seq((Ten,Hearts), (Ten,Clubs))),
        (player2Id, Seq((King,Hearts), (King,Clubs))))) ++
      playCutEventWith((Two,Clubs)) ++
      layEvents(Seq((player2Id, (King,Diamonds)), (player1Id, (Ten,Diamonds)), (player2Id, (Eight,Diamonds)))) ++
      passEvents(player1Id, player2Id) ++
      scoreEvent(player2Id, 1) ++
      playCompletedEvent ++
      layEvents(Seq((player1Id, (Five,Hearts)), (player2Id, (King,Spades)), (player1Id, (Ten,Spades)))) ++
      passEvents(player2Id) ++
      layEvents(Seq((player1Id, (Four, Clubs)))) ++
      passEvents(player2Id, player1Id) ++
      playCompletedEvent ++
      scoreEvent(player1Id, 1)

  override protected def beforeEach(): Unit = {
    super.beforeEach()
    eventSourcedTestKit.clear()
    persistenceTestKit.persistForRecovery(persistenceId, initialEvents)
  }

  def scoringGame(f: model.Game => Unit) = {
    val restart = eventSourcedTestKit.restart()
    val state = restart.state
    state should be(a[Playing]) // TODO: Ideally get to a ScoringGame but post PlaysCompleted events are not generated.
    f(state.game)
  }

  "A ScoringGame" should {

    "process the Hand & Crib scores, then SwapDealer" in scoringGame { game =>
      val result = eventSourcedTestKit.runCommand(LayCard(player2Id, cardOf(Seven,Spades)))
      result.command should be(LayCard(player2Id, cardOf(Seven,Spades)))
      result.event should be(CardLaid(player2Id, cardOf(Seven,Spades)))

      drain()
      persisted should contain theSameElementsInOrderAs(initialEvents ++ Seq(
        CardLaid(player2Id, cardOf(Seven,Spades)),
        PointsScored(player2Id, 1),
        PlayCompleted,
        PlaysCompleted,
        PointsScored(player2Id, 4),
        PointsScored(player1Id, 6),
        PointsScored(player1Id, 4),
        DealerSwapped
      ))
    }

    "be Finished" when {
      "winner declared" ignore {}
    }

  }

}
