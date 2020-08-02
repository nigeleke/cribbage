package com.nigeleke.cribbage

import java.util.UUID

import com.nigeleke.cribbage.model.{Card, Deck}
import com.nigeleke.cribbage.model.Player.{Id => PlayerId}
import com.nigeleke.cribbage.model.{Face, Suit}

object TestModel {

  type FaceSuit = (Face, Suit)
  type PlayerFaceSuit = (PlayerId, FaceSuit)
  type PlayerFaceSuits = (PlayerId, Seq[FaceSuit])

  def randomId = UUID.randomUUID()

  val deck: Deck = Deck()

  val player1Id: UUID = randomId
  val player2Id: UUID = randomId

  def cardOf(face: Face, suit: Suit): Card = deck.filter(card => card.face == face && card.suit == suit).head
  def cardOf(faceSuit: FaceSuit): Card = cardOf(faceSuit._1, faceSuit._2)
  def cardsOf(faceSuits: Seq[FaceSuit]): Seq[Card] = faceSuits.map(cardOf(_))

}
