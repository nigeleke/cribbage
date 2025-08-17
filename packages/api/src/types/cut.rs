use super::Card;

pub type Cut = Card;

pub trait HasCut {
    fn cut(&self) -> Cut;
}
