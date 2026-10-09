use cribbage_domain::prelude::{Card, Face, Suit};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[repr(transparent)]
pub struct PersistedCard(u8);

impl From<&Card> for PersistedCard {
    fn from(value: &Card) -> Self {
        let suit_index = value.suit() as u8;
        let face_index = value.face() as u8;
        Self(suit_index * 13 + face_index)
    }
}

impl TryFrom<&PersistedCard> for Card {
    type Error = ();

    fn try_from(value: &PersistedCard) -> Result<Self, Self::Error> {
        let suit = Suit::from_repr(value.0 / 13).ok_or(())?;
        let face = Face::from_repr(value.0 % 13).ok_or(())?;
        Ok(Card::new(face, suit))
    }
}

#[cfg(test)]
mod tests {
    use cribbage_domain::prelude::Deck;

    use super::*;

    #[test]
    fn round_trip() {
        Deck::new().to_vec().iter().for_each(|expected| {
            match Card::try_from(&PersistedCard::from(expected)) {
                Ok(actual) => assert_eq!(actual, *expected),
                _ => panic!("decoded card does not match encoded"),
            };
        });
    }
}
