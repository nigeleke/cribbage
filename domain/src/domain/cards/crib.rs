use super::card_stock::CardStock;

/// The current Crib.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CribType;
pub type Crib = CardStock<CribType>;

pub trait HasCrib {
    fn crib(&self) -> &Crib;
}
