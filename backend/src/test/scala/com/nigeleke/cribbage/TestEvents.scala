package com.nigeleke.cribbage

import java.util.UUID

import com.nigeleke.cribbage.actors.Game._
import com.nigeleke.cribbage.model.{Card, Deck}
import com.nigeleke.cribbage.model.Player.{Id => PlayerId}
import com.nigeleke.cribbage.suit.Face
import com.nigeleke.cribbage.suit.Suit

import scala.language.implicitConversions

object TestEvents {

  type FaceSuit = (Face, Suit)
  type PlayerFaceSuit = (PlayerId, FaceSuit)
  type PlayerFaceSuits = (PlayerId, Seq[FaceSuit])

  val deck: Deck = Deck()

  val player1Id: UUID = randomId
  val player1JoinedEvent: Seq[Event] = Seq(PlayerJoined(player1Id))

  val player2Id: UUID = randomId
  val player2JoinedEvent: Seq[Event] = Seq(PlayerJoined(player2Id))
  val playersJoinedEvents: Seq[Event] = player1JoinedEvent ++ player2JoinedEvent

  def dealerCutsEventsWith(card1 : FaceSuit, card2: FaceSuit) : Seq[Event] = Seq(
    DealerCutRevealed(player1Id, cardOf(card1)),
    DealerCutRevealed(player2Id, cardOf(card2))
  )

  val dealerSelectedEvent : Seq[Event] = Seq(DealerSelected(player1Id))

  val dealEvents : Seq[Event] = dealEventsWith((deck.take(12)))

  def dealEventsWith(cards: Seq[FaceSuit]) : Seq[Event] = Seq(
    HandsDealt(Map((player1Id, cardsOf(cards.take(6))), (player2Id, cardsOf(cards.drop(6).take(6)))), deck)
  )

  def discardEvents() : Seq[Event] = discardEventsWith(Seq(
    (player1Id, deck.take(2)),
    (player2Id, deck.drop(6).take(2))
  ))

  def discardEventsWith(discards: Seq[PlayerFaceSuits]) : Seq[Event] =
    discards.map(pfs => CribCardsDiscarded(pfs._1, pfs._2.map(cardOf)))

  val playCutEvent : Seq[Event] = playCutEventWith(deck.last)

  def playCutEventWith(cut: FaceSuit) : Seq[Event] = Seq(PlayCutRevealed(cardOf(cut)))

  def layEvents(lays: Seq[PlayerFaceSuit]): Seq[Event] = lays.map(lay => CardLaid(lay._1, cardOf(lay._2)))

  def passEvents(ids: PlayerId*): Seq[Event] = ids.map(Passed(_))

  val playCompletedEvent : Seq[Event] = Seq(PlayCompleted)

  val playsCompletedEvent : Seq[Event] = Seq(PlaysCompleted)

  def scoreEvent(playerId: PlayerId, points: Int) = Seq(PointsScored(playerId, points))

  def cardOf(face: Face, suit: Suit): Card = deck.filter(card => card.face == face && card.suit == suit).head
  def cardOf(faceSuit: FaceSuit): Card = cardOf(faceSuit._1, faceSuit._2)
  def cardsOf(faceSuits: Seq[FaceSuit]): Seq[Card] = faceSuits.map(cardOf(_))

  implicit def cardToFaceSuit(card: Card) : FaceSuit = (card.face, card.suit)
  implicit def cardsToFaceSuit(cards: Seq[Card]) : Seq[(Face, Suit)] = cards.map(card => (card.face, card.suit))

}
