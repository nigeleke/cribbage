package object cribbage {

  /** Cribbage can be a two, three or (at a push) four player game. This implementation is for two
    * players.
    */
  val NumberOfPlayersInGame = 2

  /** Six [Card]s are dealt to each Player by the current dealer. */
  val CardsDealtPerHand = 6

  /** [Player]s discard two cards each into the [Crib] leaving four for [Score]ing and [Plays]. */
  val CardsKeptPerHand = 4

  /** Each [Player] discarding two [Card]s to the [Crib] will mean four [Card]s end up there. */
  val CardsRequiredInCrib = (CardsDealtPerHand - CardsKeptPerHand) * NumberOfPlayersInGame

  /** Each [Plays.Play] cannot have a running total of more than 31. */
  val PlayTarget = 31

  /** When all [Player]s have [Laid] all [Cards] the [Score]ing can be performed. */
  val AllLaidCardCount = CardsKeptPerHand * NumberOfPlayersInGame

  /** Short games can play to 61, but it is normal to play to 121, as in this implementation. */
  val WinningScore = 121
}
