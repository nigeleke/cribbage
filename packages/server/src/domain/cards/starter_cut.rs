pub use crate::domain::Card;

pub type StarterCut = Card;

pub trait HasStarterCut {
    fn starter_cut(&self) -> &StarterCut;
}
