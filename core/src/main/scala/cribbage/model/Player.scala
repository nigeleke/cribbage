package cribbage
package model

import java.util.UUID

final case class Player(val id: UUID) extends AnyVal

object Player:
  def newPlayer: Player = Player(UUID.randomUUID())
