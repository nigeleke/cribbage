package com.nigeleke.cribbage

import com.nigeleke.cribbage.model.*
import GameState.*

import org.scalatest.*
import matchers.should.*
import wordspec.*

class FinishedGameSpec extends AnyWordSpec with Matchers:

  val player1                               = Player.createPlayer
  val player2                               = Player.createPlayer
  def finishedState(test: Finished => Unit) =
    val state = Finished(Seq((player1, Score(0, 122)), (player2, Score(0, 120))).toMap)
    test(state)

  "A FinshedGame" should {
    "report the winner" in finishedState { state =>
      Game(state).winner should be(Some(player1))
    }

    "report the winner's score" in finishedState { state =>
      val game        = Game(state)
      val maybeWinner = game.winner
      maybeWinner
        .map(winner => game.scores(winner) should be(Score(0, 122)))
        .getOrElse(fail("No winner"))
    }

    "report the loser" in finishedState { state =>
      Game(state).loser should be(Some(player2))
    }

    "report the loser's score" in finishedState { state =>
      val game       = Game(state)
      val maybeLoser = game.loser
      maybeLoser
        .map(loser => game.scores(loser) should be(Score(0, 120)))
        .getOrElse(fail("No loser"))
    }
  }
