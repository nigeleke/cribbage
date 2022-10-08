package com.nigeleke.cribbage.model

/** A player in the game.
  * @param id
  *   A unique identifier for the player.
  */
case class Player(id: Player.Id) extends AnyVal:
  def toPrettyString: String = id.toString.takeRight(6)

object Player:
  import java.util.UUID
  opaque type Id = UUID

  /** Create a new player with a unique identifier.
    * @constructor
    * @return
    *   The player.
    */
  def createPlayer: Player = Player(UUID.randomUUID())
