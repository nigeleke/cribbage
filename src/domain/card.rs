use enum_iterator::{all, Sequence};
use serde::{Deserialize, Serialize};

use std::fmt::Display;

/// The rank of a Card. Ace(1) to King(13).
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rank(usize);

impl From<usize> for Rank {
    fn from(value: usize) -> Self {
        Rank(value)
    }
}

/// The value of a Card. 1 to 10.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value(usize);

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        Value(value)
    }
}

/// A Card face.
#[derive(Clone, Copy, Debug, Deserialize, Sequence, Serialize, PartialEq)]
pub enum Face { Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King }

impl Face {
    fn rank(&self) -> Rank {
        match self {
            Face::Ace => 1,
            Face::Two => 2,
            Face::Three => 3,
            Face::Four => 4,
            Face::Five => 5,
            Face::Six => 6,
            Face::Seven => 7,
            Face::Eight => 8,
            Face::Nine => 9,
            Face::Ten => 10,
            Face::Jack => 11,
            Face::Queen => 12,
            Face::King => 13,
        }.into()
    }

    fn value(&self) -> Value {
        match self {
            Face::Ace => 1,
            Face::Two => 2,
            Face::Three => 3,
            Face::Four => 4,
            Face::Five => 5,
            Face::Six => 6,
            Face::Seven => 7,
            Face::Eight => 8,
            Face::Nine => 9,
            Face::Ten => 10,
            Face::Jack => 10,
            Face::Queen => 10,
            Face::King => 10,
        }.into()
    }
}

impl Display for Face {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Face::Ace => "A",
            Face::Two => "2",
            Face::Three => "3",
            Face::Four => "4",
            Face::Five => "5",
            Face::Six => "6",
            Face::Seven => "7",
            Face::Eight => "8",
            Face::Nine => "9",
            Face::Ten => "T",
            Face::Jack => "J",
            Face::Queen => "Q",
            Face::King => "K",
        };
        write!(f, "{}", s)
    }
}

/// A Card suit.
#[derive(Clone, Copy, Debug, Deserialize, Sequence, Serialize, PartialEq)]
enum Suit { Hearts, Clubs, Diamonds, Spades }

impl Suit {
    fn ansi_coloured(&self, s: &str) -> String {
        let colour = match self {
            Suit::Hearts | Suit::Diamonds => "\0x33[91;",
            &Suit::Clubs | &Suit::Spades => "\0x33[90;",
        };
        let reset = "\0x33[0;";
        format!("{}{}{}", colour, s, reset)
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Suit::Hearts => "♥",
            Suit::Clubs => "♣",
            Suit::Diamonds => "♦",
            Suit::Spades => "♠",
        };
        write!(f, "{}", s)
    }
}

/// A playing card.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct Card(Face, Suit);

impl Card {
    pub fn all() -> Vec<Self> { 
        let cards_for_suit = |s: Suit| all::<Face>().map(move |f| Card(f, s));
        all::<Suit>().flat_map(cards_for_suit).collect::<Vec<_>>()
    }

    fn face(&self) -> Face { self.0 }
    fn suit(&self) -> Suit { self.1 }

    pub fn face_name(&self) -> String { format!("{:?}", self.face()).trim_matches('"').into() }
    pub fn suit_name(&self) -> String { format!("{:?}", self.suit()).trim_matches('"').into() }

    pub(crate) fn rank(&self) -> Rank { self.face().rank() }
    pub(crate) fn value(&self) -> Value { self.face().value() }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let face_suit = format!("{}{}", self.face(), self.suit());
        let face_suit = self.suit().ansi_coloured(&face_suit);
        write!(f, "{}", face_suit)
    }
}