use dioxus::logger::tracing::Level;

fn main() {
    dioxus::logger::init(Level::DEBUG).expect("logger can be initiated");

    #[cfg(feature = "server")]
    server_main();
    #[cfg(not(feature = "server"))]
    frontend_main();
}

#[cfg(feature = "server")]
fn server_main() {
    use backend::launch_server;
    use frontend::App;

    dotenvy::dotenv().expect(".env file");
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(launch_server(App));
}

#[cfg(not(feature = "server"))]
fn frontend_main() {
    use frontend::App;

    dioxus::launch(App);
}
