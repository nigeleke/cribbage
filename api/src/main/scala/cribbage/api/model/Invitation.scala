package cribbage.api.model

import java.util.UUID

final case class Invitation(id: Invitation.Id)

object Invitation:
  type Id = UUID

  def create: Invitation = Invitation(UUID.randomUUID())
