package com.nigeleke.cribbage

import akka.actor.testkit.typed.scaladsl.ScalaTestWithActorTestKit
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
  with Matchers {

  implicit val log = system.log

  val eventSourcedTestKit =
    EventSourcedBehaviorTestKit[Command, Event, State](
      system,
      GameEntity(),
      SerializationSettings.disabled)

  override protected def beforeEach(): Unit = {
    super.beforeEach()
    eventSourcedTestKit.clear()
  }

  def startingGame(f: Game => Unit) = {
    val commands = Seq(CreateGame(_))
    val result = commands.map(eventSourcedTestKit.runCommand(_)).last
    result.state should be(a[Starting])
    f(result.stateOfType[Starting].game)
  }

  "A Starting Game" should {

    "allow a new player to join" in startingGame { _ =>
      val result = eventSourcedTestKit.runCommand(Join(player1Id, _))
      result.reply.isSuccess should be(true)
      result.events should contain theSameElementsInOrderAs (Seq(PlayerJoined(player1Id)))
      result.state should be(a[Starting])
      result.stateOfType[Starting].game.players should contain(player1Id)
    }

    "allow two new players to join" in startingGame { _ =>
      eventSourcedTestKit.runCommand(Join(player1Id, _))
      val result = eventSourcedTestKit.runCommand(Join(player2Id, _))
      result.reply.isSuccess should be(true)
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

      val results = Seq(player1Id, player2Id).map(id => Join(id, _)).map(eventSourcedTestKit.runCommand(_))
      results.last.state should be(a[Discarding])

      val result1 = eventSourcedTestKit.runCommand(Join(player3Id, _))
      result1.reply.isError should be(true)
      result1.hasNoEvents should be(true)
      result1.state should be(results.last.state)
    }

    "not allow the same player to join twice" in startingGame { _ =>
      val result0 = eventSourcedTestKit.runCommand(Join(player1Id, _))
      val result1 = eventSourcedTestKit.runCommand(Join(player1Id, _))
      result1.reply.isError should be(true)
      result1.hasNoEvents should be(true)
      result1.state should be(result0.state)
    }

    "deal hands" when {
      "two players have joined" in startingGame { _ =>
        eventSourcedTestKit.runCommand(Join(player1Id, _))
        val result = eventSourcedTestKit.runCommand(Join(player2Id, _))
        result.events.count(_.isInstanceOf[HandsDealt]) should be(1)

        val game = result.stateOfType[Discarding].game
        game.players.foreach(game.hands(_).size should be(6))
      }
    }

    "remove cards from deck" when {
      "players cards have in dealt" in startingGame { _ =>
        eventSourcedTestKit.runCommand(Join(player1Id, _))
        val result = eventSourcedTestKit.runCommand(Join(player2Id, _))

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
