package com.nigeleke.cribbage

import akka.actor.testkit.typed.scaladsl.ScalaTestWithActorTestKit
import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit
import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit.SerializationSettings
import com.nigeleke.cribbage.entity.GameEntity
import com.nigeleke.cribbage.entity.GameEntity._
import com.nigeleke.cribbage.model.Face._
import com.nigeleke.cribbage.model.Suit._
import com.nigeleke.cribbage.TestModel._
import com.nigeleke.cribbage.model.{ Game, Lay }
import com.typesafe.config.ConfigFactory
import org.scalatest.BeforeAndAfterEach
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpecLike

class PlayingGameSpec
  extends ScalaTestWithActorTestKit(EventSourcedBehaviorTestKit.config.withFallback(ConfigFactory.load()))
  with AnyWordSpecLike
  with BeforeAndAfterEach
  with Matchers {

  implicit val log = system.log

  private val hand1 = cardIdsOf(Seq((Ten, Hearts), (Ten, Clubs), (Ten, Diamonds), (Ten, Spades), (Five, Hearts), (Four, Clubs)))
  private val hand2 = cardIdsOf(Seq((King, Hearts), (King, Clubs), (King, Diamonds), (King, Spades), (Eight, Diamonds), (Seven, Spades)))
  private val initialAttributes = Game()
    .withPlayer(player1Id)
    .withPlayer(player2Id)
    .withDealer(player1Id)
    .withZeroScores()
    .withDeal(Map(player1Id -> hand1, player2Id -> hand2), deck)

  private val eventSourcedTestKit =
    EventSourcedBehaviorTestKit[Command, Event, State](
      system,
      GameEntity(Discarding(initialAttributes)),
      SerializationSettings.disabled)

  override protected def beforeEach(): Unit = {
    super.beforeEach()
    eventSourcedTestKit.clear()
  }

  def playingGame(f: Game => Unit) = {
    val commands = Seq(
      DiscardCribCards(player1Id, cardIdsOf(Seq((Ten, Hearts), (Ten, Clubs))), _),
      DiscardCribCards(player2Id, cardIdsOf(Seq((King, Hearts), (King, Clubs))), _))
    val result = commands.map(eventSourcedTestKit.runCommand(_)).last
    result.state should be(a[Playing])
    f(result.stateOfType[Playing].game)
  }

  "A PlayingGame" should {

    "initially make Pone the NextToLay" in playingGame { game =>
      game.play.optNextToLay should be(game.optPone)
    }

    "allow the next Player to Lay" when {
      "they have at least one valid cardId for the CurrentPlay" in playingGame { game =>
        val dealer = game.optDealer.head
        val pone = game.optPone.head
        val card = game.hands(pone).head
        val command = LayCard(pone, card, _)
        val result = eventSourcedTestKit.runCommand(command)
        result.reply.isSuccess should be(true)
        result.event should be(CardLaid(pone, card))
        result.state should be(Playing(game.withLay(pone, card).withNextToLay(dealer)))
      }
    }

    "not allow the next Player to Lay" when {
      "if it's not their turn" in playingGame { game =>
        val dealer = game.optDealer.head
        val cardId = game.hands(dealer).head
        val command = LayCard(dealer, cardId, _)
        val result = eventSourcedTestKit.runCommand(command)
        result.reply.isError should be(true)
        result.hasNoEvents should be(true)
        result.state should be(Playing(game))
      }

      "they have no valid cards for the CurrentPlay" in playingGame { game =>
        eventSourcedTestKit.runCommand(LayCard(player2Id, cardIdOf(King, Diamonds), _))
        eventSourcedTestKit.runCommand(LayCard(player1Id, cardIdOf(Ten, Diamonds), _))

        val result0 = eventSourcedTestKit.runCommand(LayCard(player2Id, cardIdOf(King, Spades), _))
        result0.reply.isSuccess should be(true)
        result0.event should be(CardLaid(player2Id, cardIdOf(King, Spades)))
        result0.state should be(Playing(
          game
            .withLay(player2Id, cardIdOf(King, Diamonds))
            .withLay(player1Id, cardIdOf(Ten, Diamonds))
            .withLay(player2Id, cardIdOf(King, Spades))))

        val result1 = eventSourcedTestKit.runCommand(LayCard(player1Id, cardIdOf(Five, Hearts), _))
        result1.reply.isError should be(true)
        result1.hasNoEvents should be(true)
        result1.state should be(result0.state)
      }
    }

    "allow the next Player to Pass" when {
      "they have no valid cards for the CurrentPlay" in playingGame { game =>
        eventSourcedTestKit.runCommand(LayCard(player2Id, cardIdOf(King, Diamonds), _))
        eventSourcedTestKit.runCommand(LayCard(player1Id, cardIdOf(Ten, Diamonds), _))

        val result0 = eventSourcedTestKit.runCommand(LayCard(player2Id, cardIdOf(King, Spades), _))
        result0.reply.isSuccess should be(true)
        result0.event should be(CardLaid(player2Id, cardIdOf(King, Spades)))
        result0.state should be(Playing(
          game
            .withLay(player2Id, cardIdOf(King, Diamonds))
            .withLay(player1Id, cardIdOf(Ten, Diamonds))
            .withLay(player2Id, cardIdOf(King, Spades))))

        val result1 = eventSourcedTestKit.runCommand(Pass(player1Id, _))
        result1.reply.isSuccess should be(true)
        result1.event should be(Passed(player1Id))
        result1.stateOfType[Playing] should be(Playing(result0.stateOfType[Playing].game.withPass(player1Id)))
      }
    }

    "not allow the next Player to Pass" when {
      "they have at least one valid cardId for the CurrentPlay" in playingGame { game =>
        val result = eventSourcedTestKit.runCommand(Pass(player2Id, _))
        result.reply.isError should be(true)
        result.hasNoEvents should be(true)
        result.state should be(Playing(game))
      }
    }

    "score the Lay" when { // Full lay scoring in ScoreLayRuleSpec
      "a Card is laid" in playingGame { game =>
        eventSourcedTestKit.runCommand(LayCard(player2Id, cardIdOf(King, Diamonds), _))
        val result = eventSourcedTestKit.runCommand(LayCard(player1Id, cardIdOf(Five, Hearts), _))
        result.reply.isSuccess should be(true)
        result.events should contain theSameElementsInOrderAs (Seq(
          CardLaid(player1Id, cardIdOf(Five, Hearts)),
          PointsScored(player1Id, 2)))
        result.state should be(Playing(game
          .withLay(player2Id, cardIdOf(King, Diamonds))
          .withLay(player1Id, cardIdOf(Five, Hearts))
          .withScore(player1Id, 2)))
      }
    }

    "score the end of Play" when {
      "play finishes with runningTotal less than 31" in playingGame { game =>
        eventSourcedTestKit.runCommand(LayCard(player2Id, cardIdOf(King, Diamonds), _))
        eventSourcedTestKit.runCommand(LayCard(player1Id, cardIdOf(Ten, Diamonds), _))
        eventSourcedTestKit.runCommand(LayCard(player2Id, cardIdOf(King, Spades), _))
        eventSourcedTestKit.runCommand(Pass(player1Id, _))
        val result = eventSourcedTestKit.runCommand(Pass(player2Id, _))
        result.reply.isSuccess should be(true)
        result.event should be(Passed(player2Id))
      }

      "play finishes with runningTotal exactly 31" in playingGame { game =>
        eventSourcedTestKit.runCommand(LayCard(player2Id, cardIdOf(King, Diamonds), _))
        eventSourcedTestKit.runCommand(LayCard(player1Id, cardIdOf(Ten, Diamonds), _))
        eventSourcedTestKit.runCommand(LayCard(player2Id, cardIdOf(Seven, Spades), _))
        val result = eventSourcedTestKit.runCommand(LayCard(player1Id, cardIdOf(Four, Clubs), _))
        result.reply.isSuccess should be(true)
        result.event should be(CardLaid(player1Id, cardIdOf(Four, Clubs)))
      }
    }

    "start the next Play" when {
      "both Players have Passed" in playingGame { game =>
        eventSourcedTestKit.runCommand(LayCard(player2Id, cardIdOf(King, Diamonds), _))
        eventSourcedTestKit.runCommand(LayCard(player1Id, cardIdOf(Ten, Diamonds), _))
        eventSourcedTestKit.runCommand(LayCard(player2Id, cardIdOf(King, Spades), _))
        eventSourcedTestKit.runCommand(Pass(player1Id, _))
        val result = eventSourcedTestKit.runCommand(Pass(player2Id, _))
        result.reply.isSuccess should be(true)
        result.events should contain theSameElementsInOrderAs (Seq(
          Passed(player2Id),
          PointsScored(player2Id, 1),
          PlayCompleted))

        val updatedGame = result.stateOfType[Playing].game
        updatedGame.play.passCount should be(0)
        updatedGame.play.optNextToLay should be(Some(player1Id))
        updatedGame.play.current should be(empty)
        updatedGame.play.previous.head should contain theSameElementsInOrderAs (Seq(
          Lay(player2Id, cardIdOf(King, Diamonds)),
          Lay(player1Id, cardIdOf(Ten, Diamonds)),
          Lay(player2Id, cardIdOf(King, Spades))))
      }

      "current Play finished on 31" in playingGame { game =>
        eventSourcedTestKit.runCommand(LayCard(player2Id, cardIdOf(King, Diamonds), _))
        eventSourcedTestKit.runCommand(LayCard(player1Id, cardIdOf(Ten, Diamonds), _))
        eventSourcedTestKit.runCommand(LayCard(player2Id, cardIdOf(Seven, Spades), _))
        val result = eventSourcedTestKit.runCommand(LayCard(player1Id, cardIdOf(Four, Clubs), _))
        result.reply.isSuccess should be(true)
        result.events should contain theSameElementsInOrderAs (Seq(
          CardLaid(player1Id, cardIdOf(Four, Clubs)),
          PointsScored(player1Id, 2),
          PlayCompleted))

        val updatedGame = result.stateOfType[Playing].game
        updatedGame.play.passCount should be(0)
        updatedGame.play.optNextToLay should be(Some(player2Id))
        updatedGame.play.current should be(empty)
        updatedGame.play.previous.head should contain theSameElementsInOrderAs (Seq(
          Lay(player2Id, cardIdOf(King, Diamonds)),
          Lay(player1Id, cardIdOf(Ten, Diamonds)),
          Lay(player2Id, cardIdOf(Seven, Spades)),
          Lay(player1Id, cardIdOf(Four, Clubs))))
      }
    }

    "start Scoring" when {
      "all Plays completed" in playingGame { game =>
        val lays = Seq(
          LayCard(player2Id, cardIdOf(King, Diamonds), _),
          LayCard(player1Id, cardIdOf(Ten, Diamonds), _),
          LayCard(player2Id, cardIdOf(Eight, Diamonds), _),
          Pass(player1Id, _),
          Pass(player2Id, _),
          LayCard(player1Id, cardIdOf(Five, Hearts), _),
          LayCard(player2Id, cardIdOf(King, Spades), _),
          LayCard(player1Id, cardIdOf(Ten, Spades), _),
          Pass(player2Id, _),
          LayCard(player1Id, cardIdOf(Four, Clubs), _),
          Pass(player2Id, _),
          Pass(player1Id, _),
          LayCard(player2Id, cardIdOf(Seven, Spades), _))

        val result = lays.map(eventSourcedTestKit.runCommand(_)).last
        result.reply.isSuccess should be(true)
        val poneScored = result.events.find(_.isInstanceOf[PoneScored]).get.asInstanceOf[PoneScored]
        val dealerScored = result.events.find(_.isInstanceOf[DealerScored]).get.asInstanceOf[DealerScored]
        val cribScored = result.events.find(_.isInstanceOf[CribScored]).get.asInstanceOf[CribScored]
        result.events should be(Seq(
          CardLaid(player2Id, cardIdOf(Seven, Spades)),
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
