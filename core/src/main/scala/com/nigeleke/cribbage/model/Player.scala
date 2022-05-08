package com.nigeleke.cribbage.model

import Cards.*

import java.util.UUID

type PlayerId = Player.Id

object Player:
  opaque type Id = UUID
  def newId: PlayerId = UUID.randomUUID()
