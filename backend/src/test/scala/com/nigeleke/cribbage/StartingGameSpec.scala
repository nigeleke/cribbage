package com.nigeleke.cribbage

import akka.actor.testkit.typed.scaladsl.{ LogCapturing, ScalaTestWithActorTestKit }
import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit
import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit.SerializationSettings
import com.nigeleke.cribbage.actors.Game
import com.nigeleke.cribbage.actors.Game._
import com.nigeleke.cribbage.TestModel._
import com.nigeleke.cribbage.model.Attributes
import org.scalatest.BeforeAndAfterEach
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpecLike

class StartingGameSpec
  extends ScalaTestWithActorTestKit(EventSourcedBehaviorTestKit.config)
  with AnyWordSpecLike
  with BeforeAndAfterEach
  with LogCapturing
  with Matchers {

  val eventSourcedTestKit =
    EventSourcedBehaviorTestKit[Command, Event, State](
      system,
      Game(),
      SerializationSettings.disabled)

  override protected def beforeEach(): Unit = {
    super.beforeEach()
    eventSourcedTestKit.clear()
  }

  def startingGame(f: Attributes => Unit) = {
    val commands = Seq(CreateGame(gameId))
    val result = commands.map(eventSourcedTestKit.runCommand(_)).last
    result.state should be(a[Starting])
    f(result.stateOfType[Starting].game)
  }

  "A Starting Attributes" should {

    "allow a new player to join" in startingGame { _ =>
      val result = eventSourcedTestKit.runCommand(Join(player1Id))
      result.command should be(Join(player1Id))
      result.events should contain theSameElementsInOrderAs (Seq(PlayerJoined(player1Id)))
      result.state should be(a[Starting])
      result.stateOfType[Starting].game.players should contain(player1Id)
    }

    "allow two new players to join" in startingGame { _ =>
      eventSourcedTestKit.runCommand(Join(player1Id))
      val result = eventSourcedTestKit.runCommand(Join(player2Id))
      result.command should be(Join(player2Id))
      result.events.size should be >= (5)
      result.events should contain(PlayerJoined(player2Id))
      result.events.count(_.isInstanceOf[DealerCutRevealed]) should be >= (2)
      result.events.count(_.isInstanceOf[DealerCutRevealed]) % 2 should be(0)
      result.events.count(_.isInstanceOf[DealerSelected]) should be(1)
      result.events.count(_.isInstanceOf[HandsDealt]) should be(1)
      result.stateOfType[Discarding].game.players should contain theSameElementsAs (Seq(player1Id, player2Id))
      result.stateOfType[Discarding].game.optDealer should be(defined)
      result.stateOfType[Discarding].game.hands.size should be(2)
      result.stateOfType[Discarding].game.hands.values.foreach(_.size should be(6))
    }

    "not allow three players to join" in startingGame { _ =>
      val player3Id = randomId

      eventSourcedTestKit.runCommand(Join(player1Id))
      val result0 = eventSourcedTestKit.runCommand(Join(player2Id))
      result0.state should be(a[Discarding])

      val result1 = eventSourcedTestKit.runCommand(Join(player3Id))
      result1.command should be(Join(player3Id))
      result1.events should be(empty)
      result1.state should be(result0.state)
    }

    "not allow the same player to join twice" in startingGame { _ =>
      val result0 = eventSourcedTestKit.runCommand(Join(player1Id))
      val result1 = eventSourcedTestKit.runCommand(Join(player1Id))
      result1.command should be(Join(player1Id))
      result1.events should be(empty)
      result1.state should be(result0.state)
    }

    "deal hands" when {
      "two players have joined" in startingGame { _ =>
        eventSourcedTestKit.runCommand(Join(player1Id))
        val result = eventSourcedTestKit.runCommand(Join(player2Id))
        result.events.count(_.isInstanceOf[HandsDealt]) should be(1)

        val game = result.stateOfType[Discarding].game
        game.players.foreach(game.hands(_).size should be(6))
      }
    }

    "remove cards from deck" when {
      "players cards have in dealt" in startingGame { _ =>
        eventSourcedTestKit.runCommand(Join(player1Id))
        val result = eventSourcedTestKit.runCommand(Join(player2Id))

        val game = result.stateOfType[Discarding].game
        game.players.foreach { id =>
          game.hands(id).size should be(6)
          game.deck should not contain allElementsOf(game.hands(id))
        }
        game.deck.size should be(40)
      }
    }
  }

}
