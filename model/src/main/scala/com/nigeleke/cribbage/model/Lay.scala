package com.nigeleke.cribbage.model

import Player.{ Id => PlayerId }

case class Lay(playerId: PlayerId, card: Card)
