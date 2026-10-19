use crate::game::{PersistedCard, PersistedCards, PersistedDiscard, Users};
use crate::user::UserId;

/// A command representing an action requested on a game.
#[derive(Debug)]
pub enum GameCommand {
    /// Create a game with the specified users and deck.
    CreateGame {
        /// The users participating in the game.
        users: Users,
        /// The deck used for the cuts for dealer.
        deck: PersistedCards,
    },

    /// Record that a player has cut for the deal.
    CutForDeal {
        /// The user cutting the deck.
        user: UserId,
    },

    /// Deal the cards to the players using the specified deck.
    DealHands {
        /// The deck from which the cards are dealt.
        deck: PersistedCards,
    },

    /// Record the cards discarded by a player.
    Discard {
        /// The user discarding the cards.
        user: UserId,
        /// The two cards discarded by the user.
        discard: PersistedDiscard,
    },

    /// Cut the starter card.
    CutStarter,

    /// Play a card from a player's hand.
    PlayCard {
        /// The user playing the card.
        user: UserId,
        /// The card being played.
        card: PersistedCard,
    },

    /// Declare that a player cannot play a card.
    Go {
        /// The user declaring go.
        user: UserId,
    },

    /// Score the pone's hand.
    ScorePone,

    /// Score the dealer's hand.
    ScoreDealer,

    /// Score the crib.
    ScoreCrib,
}
