package com.nigeleke.cribbage.model

case class GameListItem(id: Int)

case class GameList(items: Seq[GameListItem])
