package com.nigeleke.cribbage

import akka.actor.testkit.typed.scaladsl.{LogCapturing, ScalaTestWithActorTestKit}
import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit
import akka.persistence.testkit.scaladsl.EventSourcedBehaviorTestKit.SerializationSettings
import com.nigeleke.cribbage.actors.Game
import com.nigeleke.cribbage.actors.Game._
import com.nigeleke.cribbage.model.Deck
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

  private val deck = Deck()
  private val (player1Id, player2Id) = (randomId, randomId)
  private val initialEvents: Seq[Event] =
    Seq(
      DeckAllocated(deck),
      PlayerJoined(player1Id),
      PlayerJoined(player2Id),
      DealerSelected(player1Id),
      HandDealt(player1Id, deck.take(6)),
      HandDealt(player2Id, deck.drop(6).take(6)),
      HandsDealt,
      CribCardsDiscarded(player1Id, deck.take(2)),
      CribCardsDiscarded(player2Id, deck.drop(6).take(2)),
      PlayCutRevealed(deck.drop(12).take(1).head))

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

    "initially have the next Player to Lay as the Pone" in {

    }

    "allow the next Player to Lay" when {
      "they have at least one valid card for the CurrentPlay" in playingGame { game =>
        val pone = game.optPone.head
        val card = game.hands(pone).head
        val command = LayCard(pone, card)
        val result = eventSourcedTestKit.runCommand(command)
        result.command should be(command)
        result.event should be(CardLaid(pone, card))
        result.state should be(Playing(game.withPlay(pone, card)))
      }
    }

    "not allow the next Player to Lay" when {
      "they have no valid cards for the CurrentPlay" ignore {}
    }

    "allow the next Player to Pass" when {
      "they have no valid cards for the CurrentPlay" ignore {}
    }

    "not allow the next Player to Pass" when {
      "they have at least one valid card for the CurrentPlay" ignore {}
    }

    "score the Lay" when { // Full play scoring in PlayScoreSpec
      "a Card is Played" ignore {}
    }

    "start next Lay" when {
      "current Lay completed" ignore {}
    }

    "start Scoring" when {
      "all Lays completed" ignore {}
    }

  }

}
