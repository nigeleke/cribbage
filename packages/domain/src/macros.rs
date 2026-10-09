/// Creates a [`Card`] from its two-character test representation.
///
/// The first character is the face and the second is the suit. For example,
/// `"AH"` represents the ace of hearts and `"TS"` represents the ten of spades.
///
/// # Panics
///
/// Panics if the string does not contain at least two bytes or either byte is
/// not a valid face or suit.
///
/// # Examples
///
/// ```text
/// let card = card!("AH");
/// ```
#[macro_export]
macro_rules! card {
    ($s:expr) => {{
        use std::str::FromStr;
        $crate::card::Card::from_str($s).expect("valid card required")
    }};
}

/// Creates a `Vec<Card>` from a compact sequence of two-character card representations.
///
/// For example, `"AH2DTS"` represents the ace of hearts, two of diamonds,
/// and ten of spades.
///
/// # Panics
///
/// Panics if the input contains an incomplete card or an invalid face or suit.
///
/// # Examples
///
/// ```text
/// let cards = cards!("AH2DTS");
/// assert_eq!(cards.len(), 3);
/// ```
#[macro_export]
macro_rules! cards {
    ($s:expr) => {{
        let bytes = $s.as_bytes();
        assert!(
            bytes.len().is_multiple_of(2),
            "invalid cards: odd number of bytes"
        );
        bytes
            .chunks_exact(2)
            .map(|bytes| $crate::card!(std::str::from_utf8(bytes).expect("valid card required")))
            .collect::<Vec<_>>()
    }};
}

/// Creates a [`Deck`] from a compact sequence of two-character card representations.
///
/// # Examples
///
/// ```text
/// let deck = deck!("AH2DTS");
/// ```
#[macro_export]
macro_rules! deck {
    ($s:expr) => {{ $crate::cards::Deck::from_iter($crate::cards!($s)) }};
}

/// Creates a [`Hand`] from a compact sequence of two-character card representations.
///
/// # Examples
///
/// ```text
/// let hand = hand!("AH2DTS");
/// ```
#[macro_export]
macro_rules! hand {
    ($s:expr) => {{ $crate::cards::Hand::from_iter($crate::cards!($s)) }};
}

/// Creates two [`Hand`]s from compact sequences of two-character card representations.
///
/// # Examples
///
/// ```text
/// let hands = hands!("AH2D", "TSKC");
/// ```
#[macro_export]
macro_rules! hands {
    ($s0:expr, $s1:expr) => {{
        [
            $crate::cards::Hand::from($crate::cards!($s0)),
            $crate::cards::Hand::from($crate::cards!($s1)),
        ]
        .into()
    }};
}

/// Creates a [`Crib`] from a compact sequence of two-character card representations.
///
/// # Examples
///
/// ```text
/// let crib = crib!("AH2D");
/// ```
#[macro_export]
macro_rules! crib {
    ($s:expr) => {{ $crate::cards::Crib::from_iter($crate::cards!($s)) }};
}

/// Creates a scoring [`Event`] using a scoring method, player, cards, and cut card.
///
/// The method must be a scoring constructor on [`Event`], the player must be a
/// [`Player`] variant, and the cut card uses the same two-character representation
/// as [`card!`].
///
/// # Examples
///
/// ```text
/// let event = event!(fifteens, Player0, cards!("5H5C"), "KD");
/// ```
#[macro_export]
macro_rules! event {
    ($method:ident, $player:ident, $cards:expr, $cut:literal) => {
        $crate::scoreboard::Score::$method(
            $crate::players::Player::$player,
            &$cards,
            $crate::card!($cut),
        )
    };
}

/// Creates [`Roles`] with the specified player as dealer.
///
/// # Examples
///
/// ```text
/// let roles = roles!(Player0);
/// ```
#[macro_export]
macro_rules! roles {
    ($player:ident) => {
        $crate::players::Roles::new($crate::Dealer::from($crate::Player::$player))
    };
}

/// Creates a slice of [`Play`] values from player/card pairs.
///
/// Each card uses the same two-character representation as [`card!`].
///
/// # Examples
///
/// ```text
/// let plays = plays![
///     (Player0, "5H"),
///     (Player1, "6S"),
/// ];
/// ```
#[macro_export]
macro_rules! plays {
    ($(($player:ident, $card:expr)),* $(,)?) => {
        &[
            $(
                $crate::plays::Play::new(
                    $crate::players::Player::$player,
                    $crate::card!($card),
                )
            ),*
        ]
    };
}
