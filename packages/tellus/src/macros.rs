#[macro_export]
macro_rules! persisted_card {
    ($card:literal) => {
        PersistedCard::from(&card!($card))
    };
}

#[macro_export]
macro_rules! persisted_cards {
    ($card:literal) => {
        PersistedCards::from(&cards!($card))
    };
}
