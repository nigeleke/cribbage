mod app;

fn main() {
    dioxus::logger::init(dioxus::logger::tracing::Level::DEBUG)
        .expect("logger requires initiatization");

    #[cfg(feature = "server")]
    dotenvy::dotenv().expect("environment settings required");

    #[cfg(not(feature = "server"))]
    dioxus::launch(app::App);

    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        let router = dioxus::server::router(app::App);
        Ok(router)
    });
}
