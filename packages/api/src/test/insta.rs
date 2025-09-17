pub fn filtered_assert(s: String, f: impl Fn(String)) {
    insta::with_settings!({filters => vec![
        (r"Card\([A-Z2-9][HCDS]\)", "<card>"),
        (r"Ulid\([0-9]+\)", "<ulid>"),
        (r"[a-z]+-[a-z]+-[a-z]+", "<name>")
    ]}, { f(s.into()) })
}
