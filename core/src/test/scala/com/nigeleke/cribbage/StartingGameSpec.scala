package com.nigeleke.cribbage

import effects.*
import model.*

import cats.data.Validated.*
import cats.data.ValidatedNel
import cats.syntax.validated.*

import org.scalatest.*
import org.scalatest.matchers.should.*
import org.scalatest.wordspec.*

class StartingGameSpec extends AnyWordSpec with Matchers:

  "A StartingGame" should {

    val game = Game()

    "have an id" in {
      game.id should be(a[GameId])
    }

    "have no players" in {
      game.players should be(empty)
    }

    "allow a new player to join" in {
      val id = Player.newId
      Game().validNel andThen
        addPlayer(id) match
        case Valid(game) =>
          game.players.size should be(1)
        case Invalid(errors) => failWith(errors)
    }

    "allow two new players to join" in {
      val id1 = Player.newId
      val id2 = Player.newId
      Game().validNel andThen
        addPlayer(id1) andThen
        addPlayer(id2) match
        case Valid(game) =>
          game.players.size should be(2)
        case Invalid(errors) => failWith(errors)
    }

    "not allow three players to join" in {
      val id1 = Player.newId
      val id2 = Player.newId
      val id3 = Player.newId
      val game = Game()
      game.validNel andThen
        addPlayer(id1) andThen
        addPlayer(id2) andThen
        addPlayer(id3) match
        case Valid(game)     => fail(s"Incorrectly added Player ${id3} to Game ${game.id}")
        case Invalid(errors) => passIfMatched(errors, List(s"Game ${game.id} does not need any further players"))
    }

    "not allow the same player to join twice" in {
      val id = Player.newId
      val game = Game()
      game.validNel andThen
        addPlayer(id) andThen
        addPlayer(id) match
        case Valid(game)     => fail(s"Incorrectly added Player ${id} twice to Game ${game.id}")
        case Invalid(errors) => passIfMatched(errors, List(s"Player ${id} has already joined Game ${game.id}"))
    }

    "allow the game to be started with two players" in {
      val id1 = Player.newId
      val id2 = Player.newId
      val game = Game()
      game.validNel andThen
        addPlayer(id1) andThen
        addPlayer(id2) andThen
        start match
        case Valid(_)        => succeed
        case Invalid(errors) => failWith(errors)
    }

    "not allow the game to be started with less than two players" in {
      val id1 = Player.newId
      val game = Game()
      game.validNel andThen
        addPlayer(id1) andThen
        start match
        case Valid(game)     => fail(s"Incorrectly allowed Game ${game.id} to start")
        case Invalid(errors) => passIfMatched(errors, List(s"Game ${game.id} requires more Players"))
    }

  }
