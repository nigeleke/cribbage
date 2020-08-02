package com.nigeleke.cribbage

import akka.actor.testkit.typed.scaladsl.{LogCapturing, ScalaTestWithActorTestKit}
import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit
import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit.SerializationSettings
import com.nigeleke.cribbage.TestModel._
import com.nigeleke.cribbage.actors.Game
import com.nigeleke.cribbage.actors.Game._
import com.nigeleke.cribbage.model.Face._
import com.nigeleke.cribbage.model.{Lay, Play, Points}
import com.nigeleke.cribbage.model.Suit._
import org.scalatest.BeforeAndAfterEach
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpecLike

class FinishedGameSpec
  extends ScalaTestWithActorTestKit(EventSourcedBehaviorTestKit.config)
    with AnyWordSpecLike
    with BeforeAndAfterEach
    with LogCapturing
    with Matchers {

  val gameId = randomId

  private val hand1 = cardsOf(Seq((Ten,Hearts), (Ten,Clubs), (Ten,Diamonds), (Ten,Spades), (Five,Hearts), (Four,Clubs)))
  private val hand2 = cardsOf(Seq((King,Hearts), (King,Clubs), (King,Diamonds), (King,Spades), (Eight,Diamonds), (Seven,Spades)))
  private val initialGame0 = model.Status(randomId)
    .withPlayer(player1Id)
    .withPlayer(player2Id)
    .withDealer(player1Id)
    .withZeroScores()
    .withScore(player1Id, 118)
    .withScore(player2Id, 118)
    .withDeal(Map(player1Id -> hand1, player2Id -> hand2), deck)
    .withCut(cardOf(Two,Clubs))
    .withCribDiscard(player1Id, hand1.take(2))
    .withCribDiscard(player2Id, hand2.take(2))

  private val initialGame1 = initialGame0
    .copy(
      play = Play(optNextToLay = Some(player2Id),
        current = Seq.empty,
        passCount = 0,
        previous = Seq(
          initialGame0.hands(player1Id).map(card => Lay(player1Id, card)),
          initialGame0.hands(player2Id).take(3).map(card => Lay(player2Id, card)))),
      hands = Map(
        player1Id -> Seq.empty,
        player2Id -> Seq(initialGame0.hands(player2Id).last)
      ))

  private val eventSourcedTestKit =
    EventSourcedBehaviorTestKit[Command, Event, State](
      system,
      Game(gameId, Playing(initialGame1)),
      SerializationSettings.disabled)

  override protected def beforeEach(): Unit = {
    super.beforeEach()
    eventSourcedTestKit.clear()
  }

  "A FinishedGame" should {

    "ignore any further commands" in {
      val result0 = eventSourcedTestKit.runCommand(LayCard(player2Id, initialGame1.hands(player2Id).last))
      result0.state should be(a[Finished])

      val command = Join(player1Id)
      val result1 = eventSourcedTestKit.runCommand(command)
      result1.command should be(command)
      result1.events should be(Seq.empty)
      result1.state should be(result0.state)
    }

    "ignore any further events" in {
      val command = LayCard(player2Id, initialGame1.hands(player2Id).last)
      val result = eventSourcedTestKit.runCommand(command)
      result.command should be(command)
      result.events should be(Seq(
        CardLaid(player2Id, initialGame1.hands(player2Id).last),
        PointsScored(player2Id, 1),
        PlayCompleted,
        PlaysCompleted,
        PoneScored(player2Id, Points(pairs = 2, fifteens = 2)),
        WinnerDeclared(player2Id),
        DealerScored(player1Id, Points(pairs = 2, fifteens = 4)),
        WinnerDeclared(player1Id),
        CribScored(player1Id, Points(pairs = 4)),
        WinnerDeclared(player1Id),
        DealerSwapped
      ))
      result.state should be(a[Finished])
    }

  }

}