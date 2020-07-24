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

class PlayingGameSpec
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
      playCutEventWith((Two,Clubs))

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

  "A PlayingGame" should {

    "initially make Pone the NextToLay" in playingGame { game =>
      game.play.optNextToLay should be(game.optPone)
    }

    "allow the next Player to Lay" when {
      "they have at least one valid card for the CurrentPlay" in playingGame { game =>
        val dealer = game.optDealer.head
        val pone = game.optPone.head
        val card = game.hands(pone).head
        val command = LayCard(pone, card)
        val result = eventSourcedTestKit.runCommand(command)
        result.command should be(command)
        result.event should be(CardLaid(pone, card))
        result.state should be(Playing(game.withLay(pone, card).withNextToLay(dealer)))
      }
    }

    "not allow the next Player to Lay" when {
      "if it's not their turn" in playingGame { game =>
        val dealer = game.optDealer.head
        val card = game.hands(dealer).head
        val command = LayCard(dealer, card)
        val result = eventSourcedTestKit.runCommand(command)
        result.command should be(command)
        result.events should be(empty)
        result.state should be(Playing(game))
      }

      "they have no valid cards for the CurrentPlay" in playingGame { game =>
        eventSourcedTestKit.runCommand(LayCard(player2Id, cardOf(King,Diamonds)))
        eventSourcedTestKit.runCommand(LayCard(player1Id, cardOf(Ten,Diamonds)))

        val result0 = eventSourcedTestKit.runCommand(LayCard(player2Id, cardOf(King,Spades)))
        result0.command should be(LayCard(player2Id, cardOf(King,Spades)))
        result0.event should be(CardLaid(player2Id, cardOf(King,Spades)))
        result0.state should be(Playing(
          game
            .withLay(player2Id, cardOf(King,Diamonds))
            .withLay(player1Id, cardOf(Ten,Diamonds))
            .withLay(player2Id, cardOf(King,Spades))
        ))

        val result = eventSourcedTestKit.runCommand(LayCard(player1Id, cardOf(Five,Hearts)))
        result.command should be(LayCard(player1Id, cardOf(Five,Hearts)))
        result.events should be(empty)
        result.state should be(result0.state)
      }
    }

    "allow the next Player to Pass" when {
      "they have no valid cards for the CurrentPlay" in playingGame { game =>
        eventSourcedTestKit.runCommand(LayCard(player2Id, cardOf(King,Diamonds)))
        eventSourcedTestKit.runCommand(LayCard(player1Id, cardOf(Ten,Diamonds)))

        val result0 = eventSourcedTestKit.runCommand(LayCard(player2Id, cardOf(King,Spades)))
        result0.command should be(LayCard(player2Id, cardOf(King,Spades)))
        result0.event should be(CardLaid(player2Id, cardOf(King,Spades)))
        result0.state should be(Playing(
          game
            .withLay(player2Id, cardOf(King,Diamonds))
            .withLay(player1Id, cardOf(Ten,Diamonds))
            .withLay(player2Id, cardOf(King,Spades))
        ))

        val result = eventSourcedTestKit.runCommand(Pass(player1Id))
        drain()

        result.command should be(Pass(player1Id))
        result.event should be(Passed(player1Id))
        result.state should be(Playing(result0.state.game.withPass(player1Id)))
      }
    }

    "not allow the next Player to Pass" when {
      "they have at least one valid card for the CurrentPlay" in playingGame { game =>
        val result = eventSourcedTestKit.runCommand(Pass(player2Id))
        result.command should be(Pass(player2Id))
        result.events should be(empty)
        result.state should be(Playing(game))
      }
    }

    "score the Lay" when { // Full lay scoring in ScoreLayRuleSpec
      "a Card is laid" in playingGame { game =>
        eventSourcedTestKit.runCommand(LayCard(player2Id, cardOf(King,Diamonds)))
        val result = eventSourcedTestKit.runCommand(LayCard(player1Id, cardOf(Five,Hearts)))
        result.command should be(LayCard(player1Id, cardOf(Five,Hearts)))
        result.event should be(CardLaid(player1Id, cardOf(Five,Hearts)))
        result.state should be(Playing(game
          .withLay(player2Id, cardOf(King,Diamonds))
          .withLay(player1Id, cardOf(Five,Hearts))
        ))

        drain()
        persisted should contain theSameElementsInOrderAs(initialEvents ++ Seq(
          CardLaid(player2Id, cardOf(King,Diamonds)),
          CardLaid(player1Id, cardOf(Five,Hearts)),
          PointsScored(player1Id, 2)
        ))
      }
    }

    "score the end of Play" when {
      "play finishes with runningTotal less than 31" in playingGame { game =>
        eventSourcedTestKit.runCommand(LayCard(player2Id, cardOf(King,Diamonds)))
        eventSourcedTestKit.runCommand(LayCard(player1Id, cardOf(Ten,Diamonds)))
        eventSourcedTestKit.runCommand(LayCard(player2Id, cardOf(King,Spades)))
        eventSourcedTestKit.runCommand(Pass(player1Id))
        val result = eventSourcedTestKit.runCommand(Pass(player2Id))
        result.command should be(Pass(player2Id))
        result.event should be(Passed(player2Id))

        drain()
        persisted should contain theSameElementsInOrderAs(initialEvents ++ Seq(
          CardLaid(player2Id, cardOf(King,Diamonds)),
          CardLaid(player1Id, cardOf(Ten,Diamonds)),
          CardLaid(player2Id, cardOf(King,Spades)),
          Passed(player1Id),
          Passed(player2Id),
          PointsScored(player2Id, 1),
          PlayCompleted
        ))
      }

      "play finishes with runningTotal exactly 31" in playingGame { game =>
        eventSourcedTestKit.runCommand(LayCard(player2Id, cardOf(King,Diamonds)))
        eventSourcedTestKit.runCommand(LayCard(player1Id, cardOf(Ten,Diamonds)))
        eventSourcedTestKit.runCommand(LayCard(player2Id, cardOf(Seven,Spades)))
        val result = eventSourcedTestKit.runCommand(LayCard(player1Id, cardOf(Four,Clubs)))
        result.command should be(LayCard(player1Id, cardOf(Four,Clubs)))
        result.event should be(CardLaid(player1Id, cardOf(Four,Clubs)))

        drain()
        persisted should contain theSameElementsInOrderAs(initialEvents ++ Seq(
          CardLaid(player2Id, cardOf(King,Diamonds)),
          CardLaid(player1Id, cardOf(Ten,Diamonds)),
          CardLaid(player2Id, cardOf(Seven,Spades)),
          CardLaid(player1Id, cardOf(Four,Clubs)),
          PointsScored(player1Id, 2),
          PlayCompleted
        ))
      }
    }

    "start the next Play" when {
      "both Players have Passed" in playingGame { game =>
        eventSourcedTestKit.runCommand(LayCard(player2Id, cardOf(King,Diamonds)))
        eventSourcedTestKit.runCommand(LayCard(player1Id, cardOf(Ten,Diamonds)))
        eventSourcedTestKit.runCommand(LayCard(player2Id, cardOf(King,Spades)))
        eventSourcedTestKit.runCommand(Pass(player1Id))
        eventSourcedTestKit.runCommand(Pass(player2Id))

        drain()
        persisted should contain(PlayCompleted)
      }

      "current Play finished on 31" in playingGame { game =>
        eventSourcedTestKit.runCommand(LayCard(player2Id, cardOf(King,Diamonds)))
        eventSourcedTestKit.runCommand(LayCard(player1Id, cardOf(Ten,Diamonds)))
        eventSourcedTestKit.runCommand(LayCard(player2Id, cardOf(Seven,Spades)))
        eventSourcedTestKit.runCommand(LayCard(player1Id, cardOf(Four,Clubs)))

        drain()
        persisted should contain(PlayCompleted)
      }
    }

    "start Scoring" when {
      "all Plays completed" in playingGame { game =>
        eventSourcedTestKit.runCommand(LayCard(player2Id, cardOf(King,Diamonds)))
        eventSourcedTestKit.runCommand(LayCard(player1Id, cardOf(Ten,Diamonds)))
        eventSourcedTestKit.runCommand(LayCard(player2Id, cardOf(Eight,Diamonds)))
        eventSourcedTestKit.runCommand(Pass(player1Id))
        eventSourcedTestKit.runCommand(Pass(player2Id))
        drain()

        eventSourcedTestKit.runCommand(LayCard(player1Id, cardOf(Five,Hearts)))
        eventSourcedTestKit.runCommand(LayCard(player2Id, cardOf(King,Spades)))
        eventSourcedTestKit.runCommand(LayCard(player1Id, cardOf(Ten,Spades)))
        eventSourcedTestKit.runCommand(Pass(player2Id))
        drain()

        eventSourcedTestKit.runCommand(LayCard(player1Id, cardOf(Four,Clubs)))
        eventSourcedTestKit.runCommand(Pass(player2Id))
        eventSourcedTestKit.runCommand(Pass(player1Id))
        drain()

        val result = eventSourcedTestKit.runCommand(LayCard(player2Id, cardOf(Seven,Spades)))
        result.command should be(LayCard(player2Id, cardOf(Seven,Spades)))
        result.event should be(CardLaid(player2Id, cardOf(Seven,Spades)))

        drain()
        val expectedEvents = initialEvents ++ Seq(
          CardLaid(player2Id, cardOf(King,Diamonds)),
          CardLaid(player1Id, cardOf(Ten,Diamonds)),
          CardLaid(player2Id, cardOf(Eight,Diamonds)),
          Passed(player1Id),
          Passed(player2Id),
          PointsScored(player2Id, 1),
          PlayCompleted,

          CardLaid(player1Id, cardOf(Five,Hearts)),
          CardLaid(player2Id, cardOf(King,Spades)),
          PointsScored(player2Id, 2),
          CardLaid(player1Id, cardOf(Ten,Spades)),
          Passed(player2Id),
          CardLaid(player1Id, cardOf(Four,Clubs)),
          Passed(player2Id),
          Passed(player1Id),
          PointsScored(player1Id, 1),
          PlayCompleted,

          CardLaid(player2Id, cardOf(Seven,Spades)),
          PointsScored(player2Id, 1),
          PlayCompleted,
          PlaysCompleted,
          PointsScored(player2Id,4),
          PointsScored(player1Id,6),
          PointsScored(player1Id,4),
          DealerSwapped
        )
        persisted should contain theSameElementsInOrderAs(expectedEvents)

        persistenceTestKit.clearAll()
        persistenceTestKit.persistForRecovery(persistenceId, expectedEvents)
        val resetResult = eventSourcedTestKit.restart()
        resetResult.state should be(a[Discarding])
      }
    }

  }

}
