package com.nigeleke.cribbage.model

import Card.{ Id => CardId }
import Player.{ Id => PlayerId }

case class Play(playerId: PlayerId, cardId: CardId)
