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
