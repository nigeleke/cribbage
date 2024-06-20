/** Cribbage can be a two, three or (at a push) four player game. This implementation is for two
  * players.
  */
  pub const NUMBER_OF_PLAYERS_IN_GAME: usize = 2;
  
  /** Six [Card]s are dealt to each Player by the current dealer. */
  pub const CARDS_DEALT_PER_HAND: usize = 6;
    
  /** [Player]s discard two cards each into the [Crib] leaving four for [Score]ing and [Plays]. */
  pub const CARDS_KEPT_PER_HAND: usize = 4;
  pub const CARDS_DISCARDED_TO_CRIB: usize = CARDS_DEALT_PER_HAND - CARDS_KEPT_PER_HAND;
    
  /** Each [Player] discarding two [Card]s to the [Crib] will mean four [Card]s end up there. */
  pub const CARDS_REQUIRED_IN_CRIB: usize = CARDS_DISCARDED_TO_CRIB * NUMBER_OF_PLAYERS_IN_GAME;
    
  /** Each [Plays.Play] cannot have a running total of more than 31. */
  pub const PLAY_TARGET: usize = 31;
  
  /** Short games can play to 61, but it is normal to play to 121, as in this implementation. */
  pub const WINNING_SCORE: usize = 121;
