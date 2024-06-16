use super::player::Player;

use enum_iterator::{all, Sequence};
use serde::{Deserialize, Serialize};
use yansi::Paint;

use std::collections::HashMap;
use std::fmt::Display;
use std::iter::Sum;
use std::ops::{Add, Sub};

/// The rank of a Card. Ace(1) to King(13).
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rank(usize);

impl From<usize> for Rank {
    fn from(value: usize) -> Self {
        Rank(value)
    }
}

impl From<Rank> for usize {
    fn from(value: Rank) -> usize {
        value.0
    }
}

impl Sub for Rank {
    type Output = Rank;

    fn sub(self, rhs: Self) -> Self::Output {
        Rank(self.0 - rhs.0)
    }
}

/// The value of a Card. 1 to 10.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value(usize);

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        Value(value)
    }
}

impl From<Value> for usize {
    fn from(value: Value) -> Self {
        value.0
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        Value(self.0 + rhs.0)
    }
}

impl Sum for Value {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(0.into(), |acc, x| acc + x)
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

#[cfg(test)]
impl From<char> for Face {
    fn from(value: char) -> Self {
        match value {
            'A' => Face::Ace,
            '2' => Face::Two,
            '3' => Face::Three,
            '4' => Face::Four,
            '5' => Face::Five,
            '6' => Face::Six,
            '7' => Face::Seven,
            '8' => Face::Eight,
            '9' => Face::Nine,
            'T' => Face::Ten,
            'J' => Face::Jack,
            'Q' => Face::Queen,
            'K' => Face::King,
            _ => panic!("Unknown face"),
        }
    }
}

/// A Card suit.
#[derive(Clone, Copy, Debug, Deserialize, Sequence, Serialize, PartialEq)]
pub enum Suit { Hearts, Clubs, Diamonds, Spades }

impl Suit {
    fn yansi(&self, s: &str) -> String {
        match self {
            Suit::Hearts | Suit::Diamonds => format!("{}", Paint::red(s)),
            Suit::Clubs | Suit::Spades => format!("{}", Paint::black(s)),
        }
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

#[cfg(test)]
impl From<char> for Suit {
    fn from(value: char) -> Self {
        match value {
            'H' => Suit::Hearts,
            'C' => Suit::Clubs,
            'D' => Suit::Diamonds,
            'S' => Suit::Spades,
            _ => panic!("Unknown suit"),
        }
    }
}

/// A playing card.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct Card(Face, Suit);

pub type Cut = Card;
pub type Cuts = HashMap<Player, Cut>;

#[cfg(test)]
impl From<&str> for Card {
    fn from(cid: &str) -> Self {
        let mut chars = cid.chars();
        Self(Face::from(chars.next().unwrap()), Suit::from(chars.next().unwrap()))
    }
}

impl Card {
    pub fn all() -> Vec<Self> { 
        let cards_for_suit = |s: Suit| all::<Face>().map(move |f| Card(f, s));
        all::<Suit>().flat_map(cards_for_suit).collect::<Vec<_>>()
    }

    pub fn face(&self) -> Face { self.0 }
    pub fn suit(&self) -> Suit { self.1 }

    pub fn face_name(&self) -> String { format!("{:?}", self.face()).trim_matches('"').into() }
    pub fn suit_name(&self) -> String { format!("{:?}", self.suit()).trim_matches('"').into() }

    pub(crate) fn rank(&self) -> Rank { self.face().rank() }
    pub(crate) fn value(&self) -> Value { self.face().value() }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let face = self.face();
        let suit = self.suit();
        let face_suit = format!("{}{}", face, suit);
        let face_suit = suit.yansi(&face_suit);
        write!(f, "{}", face_suit)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn cards_have_definitive_names() {
        let suits = "HCDS";
        let faces = "A23456789TJQK";

        let expected_suit_name = vec!["Hearts", "Clubs", "Diamonds", "Spades"];
        let expected_face_name = vec!["Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen", "King"];

        for (si, suit) in suits.chars().enumerate() {
            for (fi, face) in faces.chars().enumerate() {
                let cid = format!("{}{}", face, suit);
                let card = Card::from(cid.as_str());
                assert_eq!(card.suit_name(), expected_suit_name[si]);
                assert_eq!(card.face_name(), expected_face_name[fi])
            }
        }
    }
}