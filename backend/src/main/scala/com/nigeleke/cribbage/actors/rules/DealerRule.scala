package com.nigeleke.cribbage.actors.rules

import akka.actor.typed.scaladsl.Behaviors
import akka.actor.typed.{ActorRef, Behavior}
import com.nigeleke.cribbage.actors.Game._
import com.nigeleke.cribbage.model.Deck
import com.nigeleke.cribbage.model.Player.{Id => PlayerId}

object DealerRule extends Rule {

  def apply(notify: ActorRef[Event]) : Behavior[Command] =
    waitUntilDealRequired(notify, Set.empty)

  private def waitUntilDealRequired(notify: ActorRef[Event], players: Set[PlayerId]) : Behavior[Command] = {
    Behaviors.receiveMessage {
      case PlayerJoined(playerId) =>
        waitUntilDealRequired(notify, players + playerId)

      case DealerSelected(dealer: PlayerId) =>
        val deal = makeDeal(players)
        deal.foreach { playerCards =>
          notify ! HandDealt(playerCards._1, playerCards._2.map(_.id))
        }
        Behaviors.same

      case other =>
        Behaviors.same
    }
  }

  private def makeDeal(players: Set[PlayerId]) = {
    val deck = Deck.shuffled()
    val drops = (0 to players.size).zip(players)
    drops.map(nPid => (nPid._2 -> deck.drop(nPid._1 * 6).take(6))).toMap
  }
}
