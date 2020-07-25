package com.nigeleke.cribbage.actors.handlers

import akka.actor.typed.ActorRef
import akka.persistence.typed.scaladsl.{Effect, EffectBuilder}
import com.nigeleke.cribbage.actors.Game._
import com.nigeleke.cribbage.actors.rules.Rules._
import com.nigeleke.cribbage.actors.rules.Rules
import com.nigeleke.cribbage.model.{Card, Cards, Deck, Game, Players}
import com.nigeleke.cribbage.model.Deck._
import com.nigeleke.cribbage.model.Player.{Id => PlayerId}

object CommandHandlers {

  def cutAtStartOfPlay(game: Game)(implicit notify: ActorRef[Command]) : EffectBuilder[Event, State] = {
    require(game.deck.size == 40)
    val deck = game.deck.shuffled
    val cut = deck.head // Will always be Some[Card]
    Effect.persist(PlayCutRevealed(cut)).thenRun(applyRules(scoreCutAtStartOfPlay))
  }


  def declareWinner(playerId: PlayerId) : EffectBuilder[Event, State] =
    Effect.persist(WinnerDeclared(playerId))

  def discardCribCards(game: Game, playerId: PlayerId, cards: Cards)(implicit notify: ActorRef[Command]) : EffectBuilder[Event, State] = {
    val playerInGame = game.players.contains(playerId)
    val twoCardsDiscarded = cards.size == 2
    val playerOwnsCards = (game.hands(playerId) intersect cards) == cards

    val discardPermitted = playerInGame && twoCardsDiscarded && playerOwnsCards

    // TODO:
//    if (discardPermitted) Effect.persist(CribCardsDiscarded(playerId, cards)).thenRun(applyRules(Rules.cutAtStartOfPlay))
//    else
      Effect.unhandled
  }

//  def initialise(game: Game) : EffectBuilder[Event, State] = {
//    val deck = Deck()
//    Effect.persist(DeckAllocated(deck))
//  }

  def join(game: Game, playerId: PlayerId)(implicit notify: ActorRef[Command]) : EffectBuilder[Event, State] = {
    require(game.players.size < 2)

    val alreadyJoined = game.players.contains(playerId)
    val players = game.players + playerId

    val cutAndDealEvents : Seq[Event] =
      if (players.size == 2) cutForDealEvents(players) ++ dealHandsEvents(players)
      else Seq.empty

    if (!alreadyJoined) Effect.persist(playerJoinedEvents(playerId) ++ cutAndDealEvents)
    else Effect.unhandled
  }

  private def playerJoinedEvents(playerId: PlayerId) : Seq[Event] = Seq(PlayerJoined(playerId))

  private def cutForDealEvents(players: Players) : Seq[Event] = {

    def cutDeck() = players.zip(Deck().shuffled).toMap

    def sameRank(cuts: Map[PlayerId, Card]) = cuts.values.groupBy(_.rank).size == 1

    val drawnCuts = Iterator.continually(cutDeck()).takeWhile(sameRank).toSeq
    val finalCuts = Iterator.continually(cutDeck()).dropWhile(sameRank).take(1).toSeq

    val reveals: Seq[Event] = for {
      cuts <- (drawnCuts ++ finalCuts)
      cut <- cuts
    } yield DealerCutRevealed(cut._1, cut._2)

    def selectDealer(cuts: Map[PlayerId, Card]) = cuts.minBy(_._2.rank)._1

    val dealerSelected = finalCuts.map(selectDealer).map(DealerSelected)

    reveals ++ dealerSelected
  }

  private def dealHandsEvents(players: Set[PlayerId]): Seq[Event] = {
    val deck = Deck().shuffled
    val hands = (0 to players.size).map(n => deck.drop(n*6).take(6))
    val deals = players.zip(hands)
    Seq(HandsDealt(deals.toMap, deck))
  }

  def layCard(game: Game, playerId: PlayerId, card: Card)(implicit notify: ActorRef[Command]) : EffectBuilder[Event, State] = {
    val playerIsNextToLay = playerId == game.play.optNextToLay.get
    val currentRunningTotal = game.play.runningTotal
    val inRange = (currentRunningTotal + card.value) <= 31
    val permitted = playerIsNextToLay && inRange

    if (permitted) Effect.persist(CardLaid(playerId, card)).thenRun(applyRules(scoreLay, endPlay))
    else Effect.unhandled
  }

  def pass(game: Game, playerId: PlayerId)(implicit notify: ActorRef[Command]) : EffectBuilder[Event, State] = {
    val playerIsNextToLay = playerId == game.play.optNextToLay.get
    val currentRunningTotal = game.play.runningTotal
    val someInRange = !game.hands(playerId).forall(card => (currentRunningTotal + card.value) > 31)
    val permitted = playerIsNextToLay && !someInRange

    if (permitted) Effect.persist(Passed(playerId)).thenRun(applyRules(endPlay))
    else Effect.unhandled
  }

  def completePlay(game: Game)(implicit notify: ActorRef[Command]) : EffectBuilder[Event, State] =
    Effect.persist(PlayCompleted).thenRun(applyRules(endPlays))

  def completePlays(game: Game)(implicit notify: ActorRef[Command]) : EffectBuilder[Event, State] =
// TODO:    Effect.persist(PlaysCompleted).thenRun(applyRules(scorePone, scoreDealer, deal))
    Effect.unhandled

  def pegScore(playerId: PlayerId, points: Int)(implicit notify: ActorRef[Command]) : EffectBuilder[Event, State] =
    Effect.persist(PointsScored(playerId, points)).thenRun(applyRules(Rules.declareWinner))

  def swapDealer()(implicit notify: ActorRef[Command]) : EffectBuilder[Event, State] =
// TODO:    Effect.persist(DealerSwapped).thenRun(applyRules(deal))
    Effect.unhandled

  private def applyRules(rules: (Game => Seq[Command])*)(state: State)(implicit notify: ActorRef[Command]) =
    rules.foreach { rule =>
      val commands = rule(state.game)
      commands.foreach(command => notify ! command)
    }

}