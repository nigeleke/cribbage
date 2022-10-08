package com.nigeleke.cribbage

import model.*
import GameState.*
import org.scalatest.*
import org.scalatest.matchers.should.*
import org.scalatest.wordspec.*

class StartingGameSpec extends AnyWordSpec with Matchers:

  "A StartingGame" should {

    "have no players" in {
      val game = Game.create
      game.players should be(empty)
    }

    "allow a new player to join" in {
      val player = Player.createPlayer
      Game.create.addPlayer(player) match
        case Right(Game(Starting(players))) => players should contain only (player)
        case other                          => fail(s"Unexpected state $other")
    }

    "allow two new players to join" in {
      val player1 = Player.createPlayer
      val player2 = Player.createPlayer
      Game.create
        .addPlayer(player1)
        .flatMap(_.addPlayer(player2)) match
        case Right(Game(Starting(players))) => players should be(Set(player1, player2))
        case other                          => fail(s"Unexpected state $other")
    }

    "not allow three players to join" in {
      val (player1, player2, player3) =
        (Player.createPlayer, Player.createPlayer, Player.createPlayer)
      Game.create
        .addPlayer(player1)
        .flatMap(_.addPlayer(player2))
        .flatMap(_.addPlayer(player3)) match
        case Left(error) => error should be(s"Cannot add player $player3")
        case other       => fail(s"Unexpected state $other")
    }

    "not allow the same player to join twice" in {
      val player = Player.createPlayer
      Game.create
        .addPlayer(player)
        .flatMap(_.addPlayer(player)) match
        case Left(error) => error should be(s"Player $player already added")
        case other       => fail(s"Unexpected state $other")
    }

    "report existing player's scores as zero" in {
      val player1 = Player.createPlayer
      val player2 = Player.createPlayer
      val game    = Game.create
        .addPlayer(player1)
        .flatMap(_.addPlayer(player2))
      game.map(_.scores(player1)) match
        case Right(score) => score should be(Score.zero)
        case other        => fail(s"Unexpected state $other")
      game.map(_.scores(player2)) match
        case Right(score) => score should be(Score.zero)
        case other        => fail(s"Unexpected state $other")
    }

    "report non-existing player's score as zero" in {
      val player1 = Player.createPlayer
      val game    = Game.create
      game.scores(player1) should be(Score.zero)
    }

    "allow the game to be started with two players" in {
      val (player1, player2) = (Player.createPlayer, Player.createPlayer)
      Game.create
        .addPlayer(player1)
        .flatMap(_.addPlayer(player2))
        .flatMap(_.start) match
        case Right(Game(Discarding(deck, scores, hands, dealer, pone, crib))) =>
          scores(dealer) should be(Score(0, 0))
          scores(pone) should be(Score(0, 0))
          crib should be(empty)
          val dealerHand = hands(dealer)
          val poneHand   = hands(pone)
          dealerHand.size should be(cardsPerHand)
          poneHand.size should be(cardsPerHand)
          deck.size should be(Deck.fullDeck.size - maxPlayers * cardsPerHand)
          dealerHand.toSet intersect deck.toSet should be(empty)
          poneHand.toSet intersect deck.toSet should be(empty)
          dealerHand.toSet intersect poneHand.toSet should be(empty)
        case other                                                            => fail(s"Unexpected state $other")
    }

    "not allow the game to be started" when {
      "less than two players" in {
        val id1 = Player.createPlayer
        Game.create
          .addPlayer(id1)
          .flatMap(_.start) match
          case Left(error) => error should be(s"Cannot start game")
          case other       => fail(s"Unexpected state $other")
      }
    }
  }
