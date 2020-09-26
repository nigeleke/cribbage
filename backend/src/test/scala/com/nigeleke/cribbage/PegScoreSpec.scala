package com.nigeleke.cribbage

import akka.actor.testkit.typed.scaladsl.ScalaTestWithActorTestKit
import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit
import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit.SerializationSettings
import com.nigeleke.cribbage.entity.GameEntity
import com.nigeleke.cribbage.entity.GameEntity.{ WinnerDeclared, _ }
import com.nigeleke.cribbage.model.Face._
import com.nigeleke.cribbage.model.Suit._
import com.nigeleke.cribbage.TestModel._
import com.nigeleke.cribbage.entity.handlers.CommandHandler
import com.nigeleke.cribbage.model.{ Game, Points }
import org.scalatest.BeforeAndAfterEach
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpecLike

class PegScoreSpec
  extends ScalaTestWithActorTestKit(EventSourcedBehaviorTestKit.config)
  with AnyWordSpecLike
  with BeforeAndAfterEach
  with Matchers {

  implicit val log = system.log

  val hand1 = cardIdsOf(Seq((Ten, Hearts), (Ten, Clubs), (Ten, Diamonds), (Ten, Spades), (Five, Hearts), (Four, Clubs)))
  val hand2 = cardIdsOf(Seq((King, Hearts), (King, Clubs), (King, Diamonds), (King, Spades), (Eight, Diamonds), (Seven, Spades)))
  val initialAttributes = Game()
    .withPlayer(player1Id)
    .withPlayer(player2Id)
    .withDealer(player1Id)
    .withZeroScores()
    .withDeal(Map(player1Id -> hand1, player2Id -> hand2), deck)
    .withCribDiscard(player1Id, hand1.take(2))
    .withCribDiscard(player2Id, hand2.take(2))

  lazy val eventSourcedTestKit =
    EventSourcedBehaviorTestKit[Command, Event, State](
      system,
      GameEntity(Playing(initialAttributes)),
      SerializationSettings.disabled)

  override protected def beforeEach(): Unit = {
    super.beforeEach()
    eventSourcedTestKit.clear()
  }

  "A score will be pegged" when {
    "cutting at start of Play" in {
      val events = CommandHandler.scoreCutAtStartOfPlay(initialAttributes)
      val playCut = events.head.asInstanceOf[PlayCutRevealed]
      events should be(
        if (playCut.card.face != Jack) Seq(playCut)
        else Seq(playCut, PointsScored(player1Id, 2)))
    }

    "pegging lays" in {
      val gameUnderTest = initialAttributes
        .withLay(player2Id, cardIdOf(King, Diamonds))
        .withLay(player1Id, cardIdOf(Five, Hearts))
      CommandHandler.scoreLay(gameUnderTest) should be(Seq(PointsScored(player1Id, 2)))
    }

    "scoring the hands" in {
      val gameUnderTest = initialAttributes.withCut(cardIdOf(Three, Clubs))
      CommandHandler.scoreHands(gameUnderTest) should be(Seq(
        PoneScored(player2Id, Points(pairs = 2, fifteens = 2)),
        DealerScored(player1Id, Points(pairs = 2, fifteens = 4, runs = 3)),
        CribScored(player1Id, Points(pairs = 4)),
        DealerSwapped))
    }

  }

  "A win will be pegged" when {
    "scoring exactly 121 in the cut at start of Play" in {
      val gameUnderTest = initialAttributes.withScore(player1Id, 119)
      val events = CommandHandler.scoreCutAtStartOfPlay(gameUnderTest)
      val playCut = events.head.asInstanceOf[PlayCutRevealed]
      events should be(
        if (playCut.card.face != Jack) Seq(playCut)
        else Seq(playCut, PointsScored(player1Id, 2), WinnerDeclared(player1Id)))
    }

    "scoring exactly 121 in pegging lays" in {
      val gameUnderTest = initialAttributes
        .withScore(player1Id, 119)
        .withLay(player2Id, cardIdOf(King, Diamonds))
        .withLay(player1Id, cardIdOf(Five, Hearts))
      CommandHandler.scoreLay(gameUnderTest) should be(Seq(PointsScored(player1Id, 2), WinnerDeclared(player1Id)))
    }

    "scoring exactly 121 while scoring the Pone Hand" in {
      val gameUnderTest = initialAttributes
        .withCut(cardIdOf(Three, Clubs))
        .withScore(player2Id, 117)
      CommandHandler.scoreHands(gameUnderTest) should be(Seq(
        PoneScored(player2Id, Points(pairs = 2, fifteens = 2)),
        WinnerDeclared(player2Id),
        DealerScored(player1Id, Points(pairs = 2, fifteens = 4, runs = 3)),
        CribScored(player1Id, Points(pairs = 4)),
        DealerSwapped))
    }

    "scoring exactly 121 while scoring the Dealer Hand" in {
      val gameUnderTest = initialAttributes
        .withCut(cardIdOf(Three, Clubs))
        .withScore(player1Id, 112)
      CommandHandler.scoreHands(gameUnderTest) should be(Seq(
        PoneScored(player2Id, Points(pairs = 2, fifteens = 2)),
        DealerScored(player1Id, Points(pairs = 2, fifteens = 4, runs = 3)),
        WinnerDeclared(player1Id),
        CribScored(player1Id, Points(pairs = 4)),
        WinnerDeclared(player1Id),
        DealerSwapped))
    }

    "scoring exactly 121 while scoring the Crib" in {
      val gameUnderTest = initialAttributes
        .withCut(cardIdOf(Three, Clubs))
        .withScore(player1Id, 108)
      CommandHandler.scoreHands(gameUnderTest) should be(Seq(
        PoneScored(player2Id, Points(pairs = 2, fifteens = 2)),
        DealerScored(player1Id, Points(pairs = 2, fifteens = 4, runs = 3)),
        CribScored(player1Id, Points(pairs = 4)),
        WinnerDeclared(player1Id),
        DealerSwapped))
    }
  }

  "A win will be pegged" when {
    "scoring greater than 121 in the cut at start of Play" in {
      val gameUnderTest = initialAttributes.withScore(player1Id, 120)
      val events = CommandHandler.scoreCutAtStartOfPlay(gameUnderTest)
      val playCut = events.head.asInstanceOf[PlayCutRevealed]
      events should be(
        if (playCut.card.face != Jack) Seq(playCut)
        else Seq(playCut, PointsScored(player1Id, 2), WinnerDeclared(player1Id)))
    }

    "scoring greater than 121 in pegging lays" in {
      val gameUnderTest = initialAttributes
        .withScore(player1Id, 120)
        .withLay(player2Id, cardIdOf(King, Diamonds))
        .withLay(player1Id, cardIdOf(Five, Hearts))
      CommandHandler.scoreLay(gameUnderTest) should be(Seq(PointsScored(player1Id, 2), WinnerDeclared(player1Id)))
    }

    "scoring greater than 121 while scoring the Pone Hand" in {
      val gameUnderTest = initialAttributes
        .withCut(cardIdOf(Three, Clubs))
        .withScore(player2Id, 120)
      CommandHandler.scoreHands(gameUnderTest) should be(Seq(
        PoneScored(player2Id, Points(pairs = 2, fifteens = 2)),
        WinnerDeclared(player2Id),
        DealerScored(player1Id, Points(pairs = 2, fifteens = 4, runs = 3)),
        CribScored(player1Id, Points(pairs = 4)),
        DealerSwapped))
    }

    "scoring greater than 121 while scoring the Dealer Hand" in {
      val gameUnderTest = initialAttributes
        .withCut(cardIdOf(Three, Clubs))
        .withScore(player1Id, 120)
      CommandHandler.scoreHands(gameUnderTest) should be(Seq(
        PoneScored(player2Id, Points(pairs = 2, fifteens = 2)),
        DealerScored(player1Id, Points(pairs = 2, fifteens = 4, runs = 3)),
        WinnerDeclared(player1Id),
        CribScored(player1Id, Points(pairs = 4)),
        WinnerDeclared(player1Id),
        DealerSwapped))
    }

    "scoring greater than 121 while scoring the Crib" in {
      val gameUnderTest = initialAttributes
        .withCut(cardIdOf(Three, Clubs))
        .withScore(player1Id, 111)
      CommandHandler.scoreHands(gameUnderTest) should be(Seq(
        PoneScored(player2Id, Points(pairs = 2, fifteens = 2)),
        DealerScored(player1Id, Points(pairs = 2, fifteens = 4, runs = 3)),
        CribScored(player1Id, Points(pairs = 4)),
        WinnerDeclared(player1Id),
        DealerSwapped))
    }
  }

}