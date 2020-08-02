package com.nigeleke.cribbage

package object ansi {

  private val color = "\u001b"

  val red   = color + "[31m"
  val black = color + "[30m"
  val blue  = color + "[34m"

  val reset = color + "[0m"

}
