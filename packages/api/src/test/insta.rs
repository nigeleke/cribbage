pub fn filtered_assert(s: String, f: impl Fn(String)) {
    insta::with_settings!({filters => vec![
        (r"Card\([A-Z2-9][HCDS]\)", "<card>"),
        (r"GameId\([0-9A-Z]{26}\)", "<gameid>"),
        (r"[a-z]+-[a-z]+-[a-z]+", "<name>"),
        (r"Ulid\([0-9]+\)", "<ulid>"),
        (r"UserId\([0-9A-Z]{26}\)", "<userid>"),
    ]}, { f(s.into()) })
}
