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
      require(game.players.size < 2, s"Player $id cannot join $game")
      game.copy(players = game.players + id)
    }

    def withDealer(id: PlayerId): Game = {
      require(game.players.contains(id), s"Dealer $id is not player in $game")
      game.copy(optDealer = Some(id))
    }

    def withHand(id: PlayerId, hand: Hand) = {
      require(game.players.contains(id), s"Player $id is not in $game")
      require(hand.forall(card => game.deck.contains(card)), s"Cards for player $id are not in deck")
      val updatedHands = game.hands.updated(id, hand)
      val updatedDeck = game.deck.filterNot(card => hand.contains(card))
      game.copy(deck = updatedDeck, hands = updatedHands)
    }

    def withCribDiscard(id: PlayerId, cards: Cards): Game = {
      require(game.players.contains(id), s"Player $id is not in $game")
      require(cards.size == 2, s"Player $id must discard two cards")
      require((game.hands(id).toSet -- cards).size == 4, s"Player $id does not own cards being discarded")
      val updatedHand = game.hands(id).filterNot(cards.contains(_))
      val updatedCrib = game.crib ++ cards
      game.copy(hands = game.hands.updated(id, updatedHand), crib = updatedCrib)
    }

    def withCut(cut: Card): Game = {
      require(game.deck.contains(cut), s"Cut card $cut not in deck")
      game.copy(optCut = Some(cut))
    }

    def withNextToLay(id: PlayerId) = {
      require(game.players.contains(id), s"Player $id is not in $game")
      val updatedPlay = game.play.withNextToLay(id)
      game.copy(play = updatedPlay)
    }

    def withLay(id: PlayerId, card: Card) = {
      require(game.players.contains(id), s"Player $id is not in $game")
      require(game.hands(id).contains(card), s"Player $id ${game.hands(id)} does not contain $card")
      require(game.play.runningTotal + card.value <= 31, s"Player $id cannot lay $card in current play")
      val updatedHand = game.hands(id).filterNot(_ == card)
      val updatedPlay = game.play.withLay(Lay(id, card)).withNextToLay(opponent(id))
      game.copy(hands = game.hands.updated(id, updatedHand), play = updatedPlay)
    }

    def withPass(id: PlayerId) = {
      require(game.players.contains(id), s"Player $id is not in $game")
      require(game.hands(id).forall(card => game.play.runningTotal + card.value > 31), s"Player $id cannot pass")
      val updatedPlay = game.play.withPass().withNextToLay(opponent(id))
      game.copy(play = updatedPlay)
    }

    def withNextPlay() = {
      require(game.players.forall(game.hands(_).forall(card => game.play.runningTotal + card.value > 31)),
        s"""Cannot progress to next play with cards still available to lay:
           | ${game.play.current}
           | ${game.hands}""".stripMargin)
      val updatedPlay = game.play.withNextPlay()
      game.copy(play = updatedPlay)
    }

    def withPlaysReturned() = {
      require(game.hands.forall(_._2.isEmpty), s"All cards should have been played")
      val laysByPlayerId = (for {
        lays <- game.play.previous
        lay <- lays
      } yield lay).groupBy(_.playerId)
      val updatedHands = laysByPlayerId.view.mapValues(_.map(_.card)).toMap
      game.copy(hands = updatedHands, play = Play())
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
