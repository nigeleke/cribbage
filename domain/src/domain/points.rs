/// The points score for a player.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Points(usize);

impl From<usize> for Points {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for Points {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Points {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::ops::Add for Points {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::iter::Sum for Points {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.map(|p| p.0).sum::<usize>().into()
    }
}
