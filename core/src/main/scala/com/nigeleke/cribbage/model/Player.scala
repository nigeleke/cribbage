package com.nigeleke.cribbage.model

case class Player(value: Player.Id) extends AnyVal {
  override def toString: String = value.toString.takeRight(6)
}

object Player:
  import java.util.UUID
  opaque type Id = UUID
  def createPlayer: Player = Player(UUID.randomUUID())