use serde::{Deserialize, Serialize};

use crate::domain::Card;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cards(Vec<Card>);

impl Cards {
    pub fn value(&self) -> &[Card] {
        &self.0
    }
}

impl std::str::FromStr for Cards {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len().is_multiple_of(2) {
            let card_chunks = |cards: &str| {
                cards
                    .chars()
                    .collect::<Vec<_>>()
                    .chunks(2)
                    .map(|chunk| chunk.iter().collect::<String>())
                    .collect::<Vec<_>>()
            };

            let cards = card_chunks(s)
                .iter()
                .flat_map(|cid| Card::from_str(cid.as_str()))
                .collect::<Vec<_>>();

            Ok(Cards(cards))
        } else {
            Err("invalid string length for cards".into())
        }
    }
}
