package com.nigeleke.cribbage.actors.rules

import com.nigeleke.cribbage.actors.RuleBook

trait Rule {

  type Command = RuleBook.Command
  type Event = RuleBook.Event

}
