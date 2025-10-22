pub use dioxus::logger::tracing::Level;
pub use dioxus::prelude::*;
pub use frontend::App;

fn main() {
    dioxus::logger::init(Level::DEBUG).expect("logger needed on startup");

    #[cfg(not(feature = "server"))]
    dioxus::launch(App);

    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        dotenvy::dotenv().expect("environment settings needed on startup");
        let _ = backend::SERVER_STATE.initialize();
        let router = dioxus::server::router(App);
        Ok(router)
    });
}
