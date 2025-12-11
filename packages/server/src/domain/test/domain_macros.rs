macro_rules! card {
    ($s:expr) => {
        Card::from_str($s).expect("valid card")
    };
}

macro_rules! cards {
    ($str:expr) => {
        $str.cards_from().expect("valid cards")
    };
}

macro_rules! crib {
    ($str:expr) => {
        Crib::from_str($str).expect("valid crib")
    };
}

macro_rules! hand {
    ($str:expr) => {
        Hand::from_str($str).expect("valid hand")
    };
}

macro_rules! plays {
    ($plays:expr) => {
        Vec::from_iter($plays.into_iter().map(|(p, c)| {
            (Play::new(
                Player::from(*p),
                $crate::domain::test::domain_macros::card!(c),
            ))
        }))
    };
}

pub(crate) use card;
pub(crate) use cards;
pub(crate) use crib;
pub(crate) use hand;
pub(crate) use plays;
