//! macros to create typed piles of cards from string literals.
//!
//! # Panics
//!
//! All macros will panic at runtime if the provided string is not a valid card representation.

/// Create an undefined (from a game point of view) [`Pile`] of cards.
#[macro_export]
macro_rules! pile {
    ($type:ty, $str:expr) => {
        Pile::<$type>::from_str($str).expect("valid pile")
    };
}

/// Create a Vec<Card>.
#[macro_export]
macro_rules! cards {
    ($str:expr) => {
        Vec::from(
            Pile::<std::any::TypeId>::from_str($str)
                .expect("valid pile")
                .as_ref(),
        )
    };
}

/// Create an [`Crib`] using the provided cards (&str).
#[macro_export]
macro_rules! crib {
    ($str:expr) => {
        Crib::from_str($str).expect("valid crib")
    };
}

/// Create an [`Deck`] using the provided cards (&str).
#[macro_export]
macro_rules! deck {
    ($str:expr) => {
        Deck::from_str($str).expect("valid deck")
    };
}

/// Create an [`Hand`] using the provided cards (&str).
#[macro_export]
macro_rules! hand {
    ($str:expr) => {
        Hand::from_str($str).expect("valid hand")
    };
}
