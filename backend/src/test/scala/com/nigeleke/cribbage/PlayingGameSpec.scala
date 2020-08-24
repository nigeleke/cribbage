package com.nigeleke.cribbage

import akka.actor.testkit.typed.scaladsl.{ LogCapturing, ScalaTestWithActorTestKit }
import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit
import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit.SerializationSettings
import com.nigeleke.cribbage.actors.Game
import com.nigeleke.cribbage.actors.Game._
import com.nigeleke.cribbage.model.Face._
import com.nigeleke.cribbage.model.Suit._
import com.nigeleke.cribbage.TestModel._
import com.nigeleke.cribbage.model.Attributes
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

  private val hand1 = cardsOf(Seq((Ten, Hearts), (Ten, Clubs), (Ten, Diamonds), (Ten, Spades), (Five, Hearts), (Four, Clubs)))
  private val hand2 = cardsOf(Seq((King, Hearts), (King, Clubs), (King, Diamonds), (King, Spades), (Eight, Diamonds), (Seven, Spades)))
  private val initialAttributes = Attributes()
    .withPlayer(player1Id)
    .withPlayer(player2Id)
    .withDealer(player1Id)
    .withZeroScores()
    .withDeal(Map(player1Id -> hand1, player2Id -> hand2), deck)

  private val eventSourcedTestKit =
    EventSourcedBehaviorTestKit[Command, Event, State](
      system,
      Game(gameId, Discarding(initialAttributes)),
      SerializationSettings.disabled)

  override protected def beforeEach(): Unit = {
    super.beforeEach()
    eventSourcedTestKit.clear()
  }

  def playingGame(f: Attributes => Unit) = {
    val commands = Seq(
      DiscardCribCards(player1Id, cardsOf(Seq((Ten, Hearts), (Ten, Clubs)))),
      DiscardCribCards(player2Id, cardsOf(Seq((King, Hearts), (King, Clubs)))))
    val result = commands.map(eventSourcedTestKit.runCommand(_)).last
    result.state should be(a[Playing])
    f(result.stateOfType[Playing].game)
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
        result.events should be(Seq.empty)
        result.state should be(Playing(game))
      }

      "they have no valid cards for the CurrentPlay" in playingGame { game =>
        eventSourcedTestKit.runCommand(LayCard(player2Id, cardOf(King, Diamonds)))
        eventSourcedTestKit.runCommand(LayCard(player1Id, cardOf(Ten, Diamonds)))

        val result0 = eventSourcedTestKit.runCommand(LayCard(player2Id, cardOf(King, Spades)))
        result0.command should be(LayCard(player2Id, cardOf(King, Spades)))
        result0.event should be(CardLaid(player2Id, cardOf(King, Spades)))
        result0.state should be(Playing(
          game
            .withLay(player2Id, cardOf(King, Diamonds))
            .withLay(player1Id, cardOf(Ten, Diamonds))
            .withLay(player2Id, cardOf(King, Spades))))

        val result1 = eventSourcedTestKit.runCommand(LayCard(player1Id, cardOf(Five, Hearts)))
        result1.command should be(LayCard(player1Id, cardOf(Five, Hearts)))
        result1.events should be(Seq.empty)
        result1.state should be(result0.state)
      }
    }

    "allow the next Player to Pass" when {
      "they have no valid cards for the CurrentPlay" in playingGame { game =>
        eventSourcedTestKit.runCommand(LayCard(player2Id, cardOf(King, Diamonds)))
        eventSourcedTestKit.runCommand(LayCard(player1Id, cardOf(Ten, Diamonds)))

        val result0 = eventSourcedTestKit.runCommand(LayCard(player2Id, cardOf(King, Spades)))
        result0.command should be(LayCard(player2Id, cardOf(King, Spades)))
        result0.event should be(CardLaid(player2Id, cardOf(King, Spades)))
        result0.state should be(Playing(
          game
            .withLay(player2Id, cardOf(King, Diamonds))
            .withLay(player1Id, cardOf(Ten, Diamonds))
            .withLay(player2Id, cardOf(King, Spades))))

        val result = eventSourcedTestKit.runCommand(Pass(player1Id))
        result.command should be(Pass(player1Id))
        result.event should be(Passed(player1Id))
        result.stateOfType[Playing] should be(Playing(result0.stateOfType[Playing].game.withPass(player1Id)))
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
        eventSourcedTestKit.runCommand(LayCard(player2Id, cardOf(King, Diamonds)))
        val result = eventSourcedTestKit.runCommand(LayCard(player1Id, cardOf(Five, Hearts)))
        result.command should be(LayCard(player1Id, cardOf(Five, Hearts)))
        result.events should contain theSameElementsInOrderAs (Seq(
          CardLaid(player1Id, cardOf(Five, Hearts)),
          PointsScored(player1Id, 2)))
        result.state should be(Playing(game
          .withLay(player2Id, cardOf(King, Diamonds))
          .withLay(player1Id, cardOf(Five, Hearts))
          .withScore(player1Id, 2)))
      }
    }

    "score the end of Play" when {
      "play finishes with runningTotal less than 31" in playingGame { game =>
        eventSourcedTestKit.runCommand(LayCard(player2Id, cardOf(King, Diamonds)))
        eventSourcedTestKit.runCommand(LayCard(player1Id, cardOf(Ten, Diamonds)))
        eventSourcedTestKit.runCommand(LayCard(player2Id, cardOf(King, Spades)))
        eventSourcedTestKit.runCommand(Pass(player1Id))
        val result = eventSourcedTestKit.runCommand(Pass(player2Id))
        result.command should be(Pass(player2Id))
        result.event should be(Passed(player2Id))
      }

      "play finishes with runningTotal exactly 31" in playingGame { game =>
        eventSourcedTestKit.runCommand(LayCard(player2Id, cardOf(King, Diamonds)))
        eventSourcedTestKit.runCommand(LayCard(player1Id, cardOf(Ten, Diamonds)))
        eventSourcedTestKit.runCommand(LayCard(player2Id, cardOf(Seven, Spades)))
        val result = eventSourcedTestKit.runCommand(LayCard(player1Id, cardOf(Four, Clubs)))
        result.command should be(LayCard(player1Id, cardOf(Four, Clubs)))
        result.event should be(CardLaid(player1Id, cardOf(Four, Clubs)))
      }
    }

    "start the next Play" when {
      "both Players have Passed" in playingGame { game =>
        eventSourcedTestKit.runCommand(LayCard(player2Id, cardOf(King, Diamonds)))
        eventSourcedTestKit.runCommand(LayCard(player1Id, cardOf(Ten, Diamonds)))
        eventSourcedTestKit.runCommand(LayCard(player2Id, cardOf(King, Spades)))
        eventSourcedTestKit.runCommand(Pass(player1Id))
        eventSourcedTestKit.runCommand(Pass(player2Id))
      }

      "current Play finished on 31" in playingGame { game =>
        eventSourcedTestKit.runCommand(LayCard(player2Id, cardOf(King, Diamonds)))
        eventSourcedTestKit.runCommand(LayCard(player1Id, cardOf(Ten, Diamonds)))
        eventSourcedTestKit.runCommand(LayCard(player2Id, cardOf(Seven, Spades)))
        eventSourcedTestKit.runCommand(LayCard(player1Id, cardOf(Four, Clubs)))
      }
    }

    "start Scoring" when {
      "all Plays completed" in playingGame { game =>
        val lays = Seq(
          LayCard(player2Id, cardOf(King, Diamonds)),
          LayCard(player1Id, cardOf(Ten, Diamonds)),
          LayCard(player2Id, cardOf(Eight, Diamonds)),
          Pass(player1Id),
          Pass(player2Id),
          LayCard(player1Id, cardOf(Five, Hearts)),
          LayCard(player2Id, cardOf(King, Spades)),
          LayCard(player1Id, cardOf(Ten, Spades)),
          Pass(player2Id),
          LayCard(player1Id, cardOf(Four, Clubs)),
          Pass(player2Id),
          Pass(player1Id),
          LayCard(player2Id, cardOf(Seven, Spades)))

        val result = lays.map(eventSourcedTestKit.runCommand(_)).last
        result.command should be(LayCard(player2Id, cardOf(Seven, Spades)))
        val poneScored = result.events.find(_.isInstanceOf[PoneScored]).get.asInstanceOf[PoneScored]
        val dealerScored = result.events.find(_.isInstanceOf[DealerScored]).get.asInstanceOf[DealerScored]
        val cribScored = result.events.find(_.isInstanceOf[CribScored]).get.asInstanceOf[CribScored]
        result.events should be(Seq(
          CardLaid(player2Id, cardOf(Seven, Spades)),
          PointsScored(player2Id, 1),
          PlayCompleted,
          PlaysCompleted,
          PoneScored(player2Id, poneScored.points),
          DealerScored(player1Id, dealerScored.points),
          CribScored(player1Id, cribScored.points),
          DealerSwapped))
        result.state should be(a[Discarding])
      }
    }

  }

}
