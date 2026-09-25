#[macro_export]
macro_rules! face {
    ($value:expr) => {
        match $value {
            b'A' => crate::Face::Ace,
            b'2' => crate::Face::Two,
            b'3' => crate::Face::Three,
            b'4' => crate::Face::Four,
            b'5' => crate::Face::Five,
            b'6' => crate::Face::Six,
            b'7' => crate::Face::Seven,
            b'8' => crate::Face::Eight,
            b'9' => crate::Face::Nine,
            b'T' => crate::Face::Ten,
            b'J' => crate::Face::Jack,
            b'Q' => crate::Face::Queen,
            b'K' => crate::Face::King,
            _ => panic!("invalid face"),
        }
    };
}

#[macro_export]
macro_rules! suit {
    ($value:expr) => {
        match $value {
            b'H' => crate::Suit::Hearts,
            b'C' => crate::Suit::Clubs,
            b'D' => crate::Suit::Diamonds,
            b'S' => crate::Suit::Spades,
            _ => panic!("invalid suit"),
        }
    };
}

#[macro_export]
macro_rules! card {
    ($s:literal) => {{
        let bytes = $s.as_bytes();
        crate::Card::new(face!(bytes[0]), suit!(bytes[1]))
    }};
}

#[macro_export]
macro_rules! cards {
    ($s:literal) => {{
        $s.as_bytes()
            .chunks_exact(2)
            .map(|bytes| crate::Card::new(face!(bytes[0]), suit!(bytes[1])))
            .collect::<Vec<crate::Card>>()
    }};
}

#[macro_export]
macro_rules! deck {
    ($s:literal) => {{ crate::Deck::from(cards!($s)) }};
}

#[macro_export]
macro_rules! hand {
    ($s:literal) => {{ crate::Hand::from(cards!($s)) }};
}

#[macro_export]
macro_rules! hands {
    ($s0:literal, $s1:literal) => {{
        [
            crate::Hand::from(cards!($s0)),
            crate::Hand::from(cards!($s1)),
        ]
        .into()
    }};
}

#[macro_export]
macro_rules! crib {
    ($s:literal) => {{ crate::Crib::from(cards!($s)) }};
}

#[macro_export]
macro_rules! event {
    ($method:ident, $player:ident, $cards:expr, $cut:literal) => {
        Event::$method(crate::Player::$player, &$cards, card!($cut)).expect("valid event")
    };
}

#[macro_export]
macro_rules! roles {
    ($player:ident) => {
        crate::Roles::new(crate::Dealer::from(crate::Player::$player))
    };
}

#[macro_export]
macro_rules! plays {
    ($(($player:ident, $card:expr)),* $(,)?) => {
        &[
            $(
                crate::Play::new(
                    crate::Player::$player,
                    card!($card),
                )
            ),*
        ]
    };
}
