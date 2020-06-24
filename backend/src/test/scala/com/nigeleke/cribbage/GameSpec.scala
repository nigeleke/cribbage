package com.nigeleke.cribbage

import akka.actor.testkit.typed.scaladsl.ScalaTestWithActorTestKit
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpecLike

class GameSpec extends ScalaTestWithActorTestKit with AnyWordSpecLike with Matchers {

  "A Game" should {
    "deal cards" when {
      "a dealer has been initially selected" ignore {
        fail()
      }
    }
  }

}
