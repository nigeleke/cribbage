#[macro_export]
macro_rules! set_no_cache_response {
    () => {
        if cfg!(debug_assertions) {
            let context = dioxus::fullstack::server_context();
            context.headers_mut().insert(
                "Cache-Control",
                http::header::HeaderValue::from_static("no-store, no-cache, must-revalidate"),
            );
        }
    };
}

/// Create a slightly more human readable `String` from an identity (e.g. variable
/// name or function name). This simply replaces all underscores with spaces.
#[macro_export]
macro_rules! prettify {
    ($ident:ident) => {
        // Use stringify! to get the identifier as a string, then replace underscores
        concat!(stringify!($ident)).replace('_', " ")
    };
}
