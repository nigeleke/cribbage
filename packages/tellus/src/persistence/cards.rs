use cribbage_domain::prelude::Card;
use serde::{Deserialize, Serialize};

use crate::persistence::PersistedCard;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[repr(transparent)]
pub struct PersistedCards(Vec<PersistedCard>);

impl From<&[Card]> for PersistedCards {
    fn from(value: &[Card]) -> Self {
        Self(value.iter().map(PersistedCard::from).collect())
    }
}

impl TryFrom<&PersistedCards> for Vec<Card> {
    type Error = ();

    fn try_from(value: &PersistedCards) -> Result<Self, Self::Error> {
        value.0.iter().map(Card::try_from).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cribbage_domain::prelude::Deck;

    #[test]
    fn round_trip() {
        let expected = Deck::new().to_vec();
        match Vec::<Card>::try_from(&PersistedCards::from(expected.as_slice())) {
            Ok(actual) => assert_eq!(actual, expected),
            _ => panic!("decoded deck does not match encoded"),
        }
    }
}
