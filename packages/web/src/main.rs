mod app;

fn main() {
    dioxus::logger::init(dioxus::logger::tracing::Level::DEBUG).expect("logger needed on startup");

    #[cfg(not(feature = "server"))]
    dioxus::launch(app::App);

    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        use dioxus::fullstack::axum::Extension;

        dotenvy::dotenv().expect("environment settings needed on startup");
        let router = dioxus::server::router(app::App)
            .layer(Extension(api::initialize_server_state().await?));
        Ok(router)
    });
}
