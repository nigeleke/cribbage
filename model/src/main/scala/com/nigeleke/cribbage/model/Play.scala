package com.nigeleke.cribbage.model

import Player.{ Id => PlayerId }

case class Play(playerId: PlayerId, card: Card)
