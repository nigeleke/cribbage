package com.nigeleke.cribbage

import model.{*, given}

import org.scalatest.*
import org.scalatest.matchers.should.*
import org.scalatest.wordspec.*

class StartingGameSpec extends AnyWordSpec with Matchers:

  "A StartingGame" should {

    val game = Game.create

    "have no players" in {
      game.players should be(empty)
    }

    "allow a new player to join" in {
      val id            = Player.createPlayer
      val expectedState = GameState.Starting(players = Set(id))
      val expectedGame  = Game(expectedState)
      game.addPlayer(id) match
        case Right(game) => game should be(expectedGame)
        case Left(error) => fail(s"Unexpected $error")
    }

    "allow two new players to join" in {
      val id1           = Player.createPlayer
      val id2           = Player.createPlayer
      val expectedState = GameState.Starting(players = Set(id1, id2))
      val expectedGame  = game.copy(state = expectedState)
      game
        .addPlayer(id1)
        .flatMap(_.addPlayer(id2)) match
        case Right(game) => game should be(expectedGame)
        case Left(error) => fail(error)
    }

    "not allow three players to join" in {
      val id1 = Player.createPlayer
      val id2 = Player.createPlayer
      val id3 = Player.createPlayer
      game
        .addPlayer(id1)
        .flatMap(_.addPlayer(id2))
        .flatMap(_.addPlayer(id3)) match
        case Right(game) => fail(s"Unexpected $game")
        case Left(error) => error should be(s"Cannot add player $id3")
    }

    "not allow the same player to join twice" in {
      val id = Player.createPlayer
      game
        .addPlayer(id)
        .flatMap(_.addPlayer(id)) match
        case Right(game) => fail(s"Unexpected $game")
        case Left(error) => error should be(s"Player $id already added")
    }

    "allow the game to be started with two players" in {
      val id1 = Player.createPlayer
      val id2 = Player.createPlayer
      game
        .addPlayer(id1)
        .flatMap(_.addPlayer(id2))
        .flatMap(_.start) match
        case Right(game) => game.state should be(a[GameState.Discarding])
        case Left(error) => fail(error)
    }

    "not allow the game to be started with less than two players" in {
      val id1 = Player.createPlayer
      game
        .addPlayer(id1)
        .flatMap(_.start) match
        case Right(game) => fail(s"Unexpected $game")
        case Left(error) => error should be(s"Cannot start game")
    }

  }
