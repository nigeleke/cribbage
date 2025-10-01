fn main() {
    dioxus::logger::init(dioxus::logger::tracing::Level::DEBUG).expect("logger can be initiated");

    let app = frontend::App;

    #[cfg(feature = "server")]
    backend::launch_backend(app);

    #[cfg(not(feature = "server"))]
    frontend::launch_frontend(app);
}
