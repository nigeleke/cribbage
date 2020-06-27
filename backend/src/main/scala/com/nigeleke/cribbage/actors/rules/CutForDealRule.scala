package com.nigeleke.cribbage.actors.rules

import akka.actor.typed.{ActorRef, Behavior}
import akka.actor.typed.scaladsl.Behaviors
import com.nigeleke.cribbage.actors.Game._
import com.nigeleke.cribbage.actors.RuleBook
import com.nigeleke.cribbage.model.{Card, Deck}
import com.nigeleke.cribbage.model.Player.{Id => PlayerId}

object CutForDealRule {

  type Command = RuleBook.Command

  type Event = RuleBook.Event

  def apply(notify: ActorRef[Event]) : Behavior[Command] = Behaviors.setup { context =>

    def waitForPlayers(players: Set[PlayerId]) : Behavior[Command] =
      Behaviors.receiveMessage {
        case PlayerJoined(playerId) =>
          if (players.isEmpty) waitForPlayers(players + playerId)
          else cutCardsUntilDealerSelected(players + playerId)

        case _ =>
          Behaviors.ignore
      }

    def cutCardsUntilDealerSelected(players: Set[PlayerId]) : Behavior[Command] = {
      Iterator
        .continually(cutCards(players))
        .dropWhile(sameRank)
        .take(1)
        .foreach(notifyDealerSelected)

      Behaviors.stopped
    }

    def cutCards(players: Set[PlayerId])  = {
      val cuts = players.zip(Deck.shuffled()).toMap
      println(s"cutCards: $cuts")
      val reveals = cuts.map(cut => DealerCutRevealed(cut._1, cut._2))
      reveals.foreach(reveal => notify ! reveal)
      cuts
    }

    def sameRank(cuts: Map[PlayerId, Card]) = {
      val same = cuts.view.groupBy(_._2.rank).size == 1
      println(s"sameRank: $same")
      same
    }

    def notifyDealerSelected(cuts: Map[PlayerId, Card]): Unit = {
      val cutsList = cuts.toList
      val (cut1, cut2) = (cutsList.head, cutsList.last)
      val dealer = if (cut1._2.rank < cut2._2.rank) cut1._1 else cut2._1
      println(s"notifyDealerSelected $dealer")
      notify ! DealerSelected(dealer)
    }

    waitForPlayers(Set.empty)
  }


}
