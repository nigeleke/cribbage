package com.nigeleke.cribbage

import akka.actor.testkit.typed.scaladsl.ScalaTestWithActorTestKit
import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit
import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit._
import com.nigeleke.cribbage.TestModel._
import com.nigeleke.cribbage.entity.GameEntity
import com.nigeleke.cribbage.entity.GameEntity._
import com.nigeleke.cribbage.model.Face._
import com.nigeleke.cribbage.model.{Game, Score}
import com.nigeleke.cribbage.model.Suit._
import org.scalatest.BeforeAndAfterEach
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpecLike

import scala.annotation.tailrec

class CutAtStartOfPlayRuleSpec
  extends ScalaTestWithActorTestKit(EventSourcedBehaviorTestKit.config)
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

  @tailrec
  private def playingGame(withJackCut: Boolean)(f: Game => Unit): Unit = {
    val commands = Seq(
      DiscardCribCards(player1Id, cardIdsOf(Seq((Ten, Hearts), (Ten, Clubs))), _),
      DiscardCribCards(player2Id, cardIdsOf(Seq((King, Hearts), (King, Clubs))), _))
    val result = commands.map(eventSourcedTestKit.runCommand(_)).last
    val game = result.stateOfType[Playing].game

    val cutWasAsRequested = {
      game.optCut.exists { cutId =>
        {
          val cut = game.card(cutId)
          (withJackCut && cut.face == Jack) || (!withJackCut && (cut.face != Jack))
        }
      }
    }

    if (cutWasAsRequested) f(result.stateOfType[Playing].game)
    else {
      eventSourcedTestKit.clear()
      playingGame(withJackCut)(f)
    }
  }

  "The CutAtStartOfPlayRule" should {

    "score two points for the Dealer" when {
      "a Jack is cut" in playingGame(true) { game =>
        val dealerId = game.optDealer.get
        val score = game.scores(dealerId)
        score should be(Score(0, 2))
      }
    }

    "not score points" when {
      "anything other than Jack is cut" in playingGame(false) { game =>
        val dealerId = game.optDealer.get
        val score = game.scores(dealerId)
        score should be(Score(0, 0))
      }
    }

  }
}
