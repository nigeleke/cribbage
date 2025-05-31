#[cfg(feature = "server")]
use domain::Card as DomainCard;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Card(String);

#[cfg(feature = "server")]
impl From<&DomainCard> for Card {
    fn from(value: &DomainCard) -> Self {
        Self(value.cid())
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
