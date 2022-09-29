package com.nigeleke.cribbage.domain

import java.util.UUID

object Player:
  opaque type Id = UUID
  def newId: Player.Id = UUID.randomUUID().asInstanceOf[Player.Id]
