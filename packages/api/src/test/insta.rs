pub fn filtered_assert(s: String, f: impl Fn(String)) {
    insta::with_settings!({filters => vec![
        (r"Card\([A-Z2-9][HCDS]\)", "<card>"),
        (r"[a-z]+-[a-z]+-[a-z]+", "<name>"),
        (r"[0-9a-f]{8}(-[0-9a-z]{4}){3}-[0-9a-z]{12}", "<uuid>"),
    ]}, { f(s.into()) })
}
