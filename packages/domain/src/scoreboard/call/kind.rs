/// Represents the kind of call.
#[derive(Clone, Copy, PartialEq, Eq)]
#[rustfmt::skip]
pub enum CallKind {
    #[doc(hidden)] Fifteen,
    #[doc(hidden)] Pair,
    #[doc(hidden)] Triplet,
    #[doc(hidden)] Quadruplet,
    #[doc(hidden)] Run,
    #[doc(hidden)] Flush,
    #[doc(hidden)] LastCard,
    #[doc(hidden)] ThirtyOne,
    #[doc(hidden)] HisHeels,
    #[doc(hidden)] Nobs,
}

impl std::fmt::Debug for CallKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Fifteen => "15",
            Self::Pair => "pair",
            Self::Triplet => "3 pairs",
            Self::Quadruplet => "4 pairs",
            Self::Run => "run",
            Self::Flush => "flush",
            Self::LastCard => "last-card",
            Self::ThirtyOne => "31",
            Self::HisHeels => "heels",
            Self::Nobs => "nobs",
        })
    }
}
