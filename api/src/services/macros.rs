#[macro_export]
macro_rules! set_default_cache {
    () => {
        if cfg!(debug_assertions) {
            let context = dioxus::prelude::server_context();
            context.response_parts_mut().headers.insert(
                "Cache-Control",
                http::header::HeaderValue::from_static("no-store, no-cache, must-revalidate"),
            );
        }
    };
}
