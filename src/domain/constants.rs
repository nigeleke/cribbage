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
  

  //     /** When all [Player]s have [Laid] all [Cards] the [Score]ing can be performed. */
  //     val AllLaidCardCount = CardsKeptPerHand * NumberOfPlayersInGame
    
  /** Short games can play to 61, but it is normal to play to 121, as in this implementation. */
  pub const WINNING_SCORE: usize = 121;
  
  
  
  // /// A collection of cards to show.
  // #[derive(Serialize, Deserialize, Debug)]
  // struct Cards(Vec<CardView>);
  
  // /// A player's hand.
  // #[derive(Serialize, Deserialize, Debug)]
  // pub struct Hand(Cards);
  
  // /// The current crib.
  // #[derive(Serialize, Deserialize, Debug)]
  // pub struct Crib(Cards);
  
  // /// The current cut.
  // #[derive(Serialize, Deserialize, Debug)]
  // pub struct Cut(CardView);
  
  // /// A points value.
  // #[derive(Serialize, Deserialize, Debug)]
  // struct Points(i32);
  
  // /// A score comprises the back and front pegs.
  // #[derive(Serialize, Deserialize, Debug)]
  // struct Score {
  //     back_peg: Points,
  //     front_peg: Points,
  // }
  
  // /// My score.
  // #[derive(Serialize, Deserialize, Debug)]
  // pub struct MyScore(Score);
  
  // /// The opponent's score.
  // #[derive(Serialize, Deserialize, Debug)]
  // pub struct OpponentScore(Score);
  
  
  // impl fmt::Display for PlayerId {
  //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
  //         write!(f, "{}", self.0)
  //     }
  // }
  
  // /// A player is either the current session or not.
  // #[derive(Serialize, Deserialize, Debug)]
  // pub enum Player {
  //     Me,
  //     Opponent,
  // }
  
  // /// A player state is their score, and the cards currently held in their hand.
  // #[derive(Serialize, Deserialize, Debug)]
  // struct PlayerState {
  //     score: Score,
  //     hand: Cards,
  // }
  
  // /// My current hand & score.
  // #[derive(Serialize, Deserialize, Debug)]
  // pub struct MyState(PlayerState);
  
  // /// Opponent's current hand & score.
  // #[derive(Serialize, Deserialize, Debug)]
  // pub struct OpponentState(PlayerState);
  
  // /// During play, a player lays a card. Passes are not shown.
  // #[derive(Serialize, Deserialize, Debug)]
  // struct Lay {
  //     player: Player,
  //     card: Card,
  // }
  
  // /// A collection of lays.
  // #[derive(Serialize, Deserialize, Debug)]
  // struct Lays(Vec<Lay>);
  
  // /// The play state is the next player to play, the current cards laid and historic cards laid.
  // #[derive(Serialize, Deserialize, Debug)]
  // pub struct Play {
  //     next_player: Player,
  //     current_play: Lays,
  //     historic_plays: Lays,
  // }
  
  // #[derive(Copy, Clone, Debug, Serialize, Deserialize)]
  // pub struct GameId(Ulid);
  
  // impl GameId {
  //     pub fn new() -> Self { GameId(Ulid::new()) }
  // }
  
  // impl fmt::Display for GameId {
  //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
  //         write!(f, "{}", self.0)
  //     }
  // }
  
  