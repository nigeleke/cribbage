use cribbage_domain::prelude::{Card, Discard};
use serde::{Deserialize, Serialize};

use crate::game::PersistedCard;

/// A pair of cards in their persisted form, representing a discard.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(transparent)]
pub struct PersistedDiscard([PersistedCard; 2]);

impl From<&Discard> for PersistedDiscard {
    fn from(value: &Discard) -> Self {
        let card0 = PersistedCard::from(&value[0]);
        let card1 = PersistedCard::from(&value[1]);
        Self([card0, card1])
    }
}

impl TryFrom<&PersistedDiscard> for Discard {
    type Error = ();

    fn try_from(value: &PersistedDiscard) -> Result<Self, Self::Error> {
        let card0 = Card::try_from(&value.0[0])?;
        let card1 = Card::try_from(&value.0[1])?;
        Ok([card0, card1])
    }
}

#[cfg(test)]
#[coverage(off)]
mod tests {
    use cribbage_domain::prelude::Deck;

    use super::*;

    #[test]
    fn round_trip() {
        Deck::new()
            .to_vec()
            .chunks_exact(2)
            .map(|cards| Discard::try_from(cards).unwrap())
            .for_each(
                |expected| match Discard::try_from(&PersistedDiscard::from(&expected)) {
                    Ok(actual) => assert_eq!(actual, expected),
                    _ => panic!("decoded discard does not match encoded"),
                },
            );
    }
}
