//! macros to create individual cards from string literals.
//!
//! # Panics
//!
//! All macros will panic at runtime if the provided string is not a valid card representation.

/// Create a [`Card`].
#[macro_export]
macro_rules! card {
    ($s:expr) => {
        Card::from_str($s).expect("valid card")
    };
}

/// Create a [`Cut`].
#[macro_export]
macro_rules! cut {
    ($s:expr) => {
        Cut::from_str($s).expect("valid cut")
    };
}
