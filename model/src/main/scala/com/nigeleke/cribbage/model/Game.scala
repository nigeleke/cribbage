package com.nigeleke.cribbage.model

import java.util.UUID

import Game.{Id => GameId}
import Player.{Id => PlayerId}

final case class Game(id: GameId,
                      deck: Deck,
                      players: Players,
                      optDealer: Option[PlayerId],
                      hands: Hands,
                      crib: Crib,
                      optCut: Option[Card],
                      play: Play,
                      scores: Scores)

object Game {
  type Id = UUID

  def apply(id: Id) : Game =
    Game(id,
      deck = Seq.empty,
      players = Set.empty,
      optDealer = None,
      hands = Map.empty,
      crib = Seq.empty,
      optCut = None,
      play = Play(),
      scores = Map.empty)

  implicit class GameOps(game: Game) {

    lazy val optPone : Option[PlayerId] = (for {
      dealer <- game.optDealer
      otherPlayers = game.players - dealer
    } yield otherPlayers.headOption).flatten

    def withDeck(deck: Deck): Game = game.copy(deck = deck)

    def withPlayer(id: PlayerId): Game = {
      require(game.players.size < 2)
      game.copy(players = game.players + id)
    }

    def withDealer(id: PlayerId): Game = {
      require(game.players.contains(id))
      game.copy(optDealer = Some(id))
    }

    def withHand(id: PlayerId, hand: Hand) = {
      require(game.players.contains(id))
      require(hand.forall(card => game.deck.contains(card)))
      val updatedHands = game.hands.updated(id, hand)
      val updatedDeck = game.deck.filterNot(card => hand.contains(card))
      game.copy(deck = updatedDeck, hands = updatedHands)
    }

    def withCribDiscard(id: PlayerId, cards: Cards): Game = {
      require(game.players.contains(id))
      require(cards.size == 2)
      require((game.hands(id).toSet -- cards).size == 4)
      val updatedHand = game.hands(id).filterNot(cards.contains(_))
      val updatedCrib = game.crib ++ cards
      game.copy(hands = game.hands.updated(id, updatedHand), crib = updatedCrib)
    }

    def withCut(cut: Card): Game = {
      require(game.deck.contains(cut))
      game.copy(optCut = Some(cut))
    }

    def withNextToLay(playerId: PlayerId) = {
      require(game.players.contains(playerId))
      val updatedPlay = game.play.withNextToLay(playerId)
      game.copy(play = updatedPlay)
    }

    def withLay(playerId: PlayerId, card: Card) = {
      require(game.players.contains(playerId))
      require(game.hands(playerId).contains(card), s"Player $playerId ${game.hands(playerId)} does not contain $card")
      require(game.play.runningTotal + card.value <= 31)
      val updatedHand = game.hands(playerId).filterNot(_ == card)
      val updatedPlay = game.play.withLay(Lay(playerId, card)).withNextToLay(opponent(playerId))
      game.copy(hands = game.hands.updated(playerId, updatedHand), play = updatedPlay)
    }

    def withPass(playerId: PlayerId) = {
      require(game.players.contains(playerId))
      require(game.hands(playerId).forall(card => game.play.runningTotal + card.value > 31))
      val updatedPlay = game.play.withPass().withNextToLay(opponent(playerId))
      game.copy(play = updatedPlay)
    }

    def withNextPlay() = {
      require(game.players.forall(game.hands(_).forall(card => game.play.runningTotal + card.value > 31)))
      val updatedPlay = game.play.withNextPlay()
      game.copy(play = updatedPlay)
    }

    def opponent(playerId: PlayerId) : PlayerId = {
      game.players.filterNot(_ == playerId).head
    }

    def withScore(id: PlayerId, points: Int): Game = {
      val currentScore = game.scores.getOrElse(id, Score(0,0))
      val updatedScore = Score(currentScore.front, currentScore.front + points)
      game.copy(scores = game.scores.updated(id, updatedScore))
    }
  }

}
