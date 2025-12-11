#![feature(coverage_attribute)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![doc = include_str!("../README.md")]

mod app;

fn main() {
    dioxus::logger::init(dioxus::logger::tracing::Level::DEBUG).expect("logger needed on startup");

    #[cfg(not(feature = "server"))]
    dioxus::launch(app::App);

    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        use dioxus::server::axum::Extension;

        dotenvy::dotenv().expect("environment settings needed on startup");

        let router = dioxus::server::router(app::App)
            .layer(Extension(api::initialize_server_state().await?));

        Ok(router)
    });
}
