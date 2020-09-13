package com.nigeleke.cribbage

import akka.actor.testkit.typed.scaladsl.{ LogCapturing, ScalaTestWithActorTestKit }
import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit
import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit.SerializationSettings
import com.nigeleke.cribbage.entity.GameEntity
import com.nigeleke.cribbage.entity.GameEntity._
import com.nigeleke.cribbage.TestModel._
import com.nigeleke.cribbage.model.Game
import org.scalatest.BeforeAndAfterEach
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpecLike

class StartingGameSpec
  extends ScalaTestWithActorTestKit(EventSourcedBehaviorTestKit.config)
  with AnyWordSpecLike
  with BeforeAndAfterEach
  with LogCapturing
  with Matchers {

  implicit val log = system.log

  val probe = createTestProbe[Reply]()

  val eventSourcedTestKit =
    EventSourcedBehaviorTestKit[Command, Event, State](
      system,
      GameEntity(randomId),
      SerializationSettings.disabled)

  override protected def beforeEach(): Unit = {
    super.beforeEach()
    eventSourcedTestKit.clear()
  }

  def startingGame(f: Game => Unit) = {
    val commands = Seq(CreateGame(probe.ref))
    val result = commands.map(eventSourcedTestKit.runCommand(_)).last
    result.state should be(a[Starting])
    f(result.stateOfType[Starting].game)
  }

  "A Starting Game" should {

    "allow a new player to join" in startingGame { _ =>
      val result = eventSourcedTestKit.runCommand(Join(player1Id, probe.ref))
      result.command should be(Join(player1Id, probe.ref))
      result.events should contain theSameElementsInOrderAs (Seq(PlayerJoined(player1Id)))
      result.state should be(a[Starting])
      result.stateOfType[Starting].game.players should contain(player1Id)
    }

    "allow two new players to join" in startingGame { _ =>
      eventSourcedTestKit.runCommand(Join(player1Id, probe.ref))
      val result = eventSourcedTestKit.runCommand(Join(player2Id, probe.ref))
      result.command should be(Join(player2Id, probe.ref))
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

      eventSourcedTestKit.runCommand(Join(player1Id, probe.ref))
      val result0 = eventSourcedTestKit.runCommand(Join(player2Id, probe.ref))
      result0.state should be(a[Discarding])

      val result1 = eventSourcedTestKit.runCommand(Join(player3Id, probe.ref))
      result1.command should be(Join(player3Id, probe.ref))
      result1.events should be(empty)
      result1.state should be(result0.state)
    }

    "not allow the same player to join twice" in startingGame { _ =>
      val result0 = eventSourcedTestKit.runCommand(Join(player1Id, probe.ref))
      val result1 = eventSourcedTestKit.runCommand(Join(player1Id, probe.ref))
      result1.command should be(Join(player1Id, probe.ref))
      result1.events should be(empty)
      result1.state should be(result0.state)
    }

    "deal hands" when {
      "two players have joined" in startingGame { _ =>
        eventSourcedTestKit.runCommand(Join(player1Id, probe.ref))
        val result = eventSourcedTestKit.runCommand(Join(player2Id, probe.ref))
        result.events.count(_.isInstanceOf[HandsDealt]) should be(1)

        val game = result.stateOfType[Discarding].game
        game.players.foreach(game.hands(_).size should be(6))
      }
    }

    "remove cards from deck" when {
      "players cards have in dealt" in startingGame { _ =>
        eventSourcedTestKit.runCommand(Join(player1Id, probe.ref))
        val result = eventSourcedTestKit.runCommand(Join(player2Id, probe.ref))

        val game = result.stateOfType[Discarding].game
        game.players.foreach { id =>
          game.hands(id).size should be(6)
          game.availableDeck should not contain allElementsOf(game.hands(id))
        }
        game.availableDeck.size should be(40)
      }
    }
  }

}
