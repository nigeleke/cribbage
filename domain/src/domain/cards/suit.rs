use enum_iterator::Sequence;

/// A Card suit.
#[derive(Clone, Copy, Debug, Sequence, PartialEq, Eq)]
pub enum Suit {
    Hearts,
    Clubs,
    Diamonds,
    Spades,
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Hearts => "H",
            Self::Clubs => "C",
            Self::Diamonds => "D",
            Self::Spades => "S",
        };
        s.fmt(f)
    }
}

impl TryFrom<char> for Suit {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'H' => Ok(Self::Hearts),
            'C' => Ok(Self::Clubs),
            'D' => Ok(Self::Diamonds),
            'S' => Ok(Self::Spades),
            _ => Err("unknown suit".into()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_create_suit_from_valid_char() {
        for c in b"HCDS" {
            let _ = Suit::try_from(*c as char).expect("valid suit");
        }
    }

    #[test]
    fn cannot_create_suit_from_invalid_char() {
        let _ = Suit::try_from('#').expect_err("invalid suit");
    }
}
