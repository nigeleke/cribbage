use super::pile::Pile;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CribType;
pub type Crib = Pile<CribType>;

pub trait HasCrib {
    fn crib(&self) -> &Crib;
    fn crib_mut(&mut self) -> &mut Crib;
}
