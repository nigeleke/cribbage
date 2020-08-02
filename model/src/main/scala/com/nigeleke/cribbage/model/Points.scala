package com.nigeleke.cribbage.model

import com.nigeleke.cribbage.ansi._

case class Points(pairs: Int = 0, fifteens: Int = 0, runs: Int = 0, flushes: Int = 0, heels: Int = 0) {

  val total : Int = pairs + fifteens + runs + flushes + heels

  override val toString =
    s"${blue}pair->$pairs 15->$fifteens run->$runs flush->$flushes heels->$heels => ${this.total}$reset"

}
