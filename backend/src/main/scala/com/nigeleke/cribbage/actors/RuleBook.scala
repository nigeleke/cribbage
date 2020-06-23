package com.nigeleke.cribbage.actors

import akka.NotUsed
import akka.actor.typed.Behavior
import akka.actor.typed.scaladsl.Behaviors
import com.nigeleke.cribbage.actors.rules.CutForDealRule
import com.nigeleke.cribbage.model.Game.{Id => GameId}

// SRR: Arbitrate the rules during the course of the Game
object RuleBook {

  def apply(gameId: GameId) : Behavior[NotUsed] = Behaviors.setup { context =>
    context.spawn(CutForDealRule(gameId), s"cut-for-deal-rule-$gameId")
    Behaviors.ignore
  }

}
