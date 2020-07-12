package com.nigeleke

import java.util.UUID

import akka.actor.testkit.typed.scaladsl.ActorTestKit

package object cribbage {

  def randomId = UUID.randomUUID()

  def drain()(implicit testKit: ActorTestKit) = testKit.createTestProbe().expectNoMessage()

}
