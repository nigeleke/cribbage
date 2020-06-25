package com.nigeleke.cribbage.actors.rules

import akka.actor.typed.Behavior
import akka.actor.typed.eventstream.EventStream.{Publish, Subscribe}
import akka.actor.typed.scaladsl.Behaviors
import com.nigeleke.cribbage.actors.Game._
import com.nigeleke.cribbage.model.{Card, Deck}
import com.nigeleke.cribbage.model.Game.{Id => GameId}
import com.nigeleke.cribbage.model.Player.{Id => PlayerId}

object CutForDealRule {

  sealed trait Command
  private case class WrappedGameEvent(event: Event) extends Command

  private case class Cut(player: PlayerId, card: Card) {
    lazy val rank: Int = card.rank
  }

  def apply(gameId: GameId) : Behavior[Command] = Behaviors.setup { context =>
    val gameEventAdaptor = context.messageAdapter[Event](WrappedGameEvent(_))
    context.system.eventStream ! Subscribe[Event](gameEventAdaptor)
    waitForPlayers(gameId, Set.empty)
  }

  def waitForPlayers(gameId: GameId, players: Set[PlayerId]) : Behavior[Command] =
    Behaviors.receiveMessage {
      case WrappedGameEvent(event) => event match {
        case PlayerJoined(gid, playerId) if gid == gameId =>
          if (players.isEmpty) waitForPlayers(gameId, players + playerId)
          else cutCards(gameId, players + playerId)
      }
    }

  def cutCards(gameId: GameId, players: Set[PlayerId]) : Behavior[Command] =
    Behaviors.setup { context =>
      val cuts = players.zip(Deck.shuffled()).toMap
      val reveals = cuts.map(cut => DealerCutRevealed(gameId, cut._1, cut._2))
      reveals.foreach(reveal => context.system.eventStream ! Publish(reveal))
      val sameRank = cuts.groupBy(_._2.rank).size
      println(s"${cuts.values}\n$sameRank")
//      if (cut1.rank == cut2.rank) cutCards(gameId, players)
//      else notifyDealerSelected(gameId, cut1, cut2)
      Behaviors.same // TODO: Above
    }

  def notifyDealerSelected(gameId: GameId, cut1: Cut, cut2: Cut) : Behavior[Command] =
    Behaviors.setup { context =>
      val dealer = if (cut1.rank < cut2.rank) cut1.player else cut2.player
      context.system.eventStream ! Publish(DealerSelected(gameId, dealer))
      Behaviors.same
    }

}
