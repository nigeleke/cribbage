use std::sync::Arc;

use dioxus::prelude::*;

pub fn launch_backend(app: fn() -> Element) {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(launch_server(app));
}

async fn launch_server(app: fn() -> Element) {
    println!("launch server 0");
    dotenvy::dotenv().expect("required .env file should exist");

    println!("launch server 1");
    let socket_addr = dioxus::cli_config::fullstack_address_or_localhost();

    println!("launch server 2");
    let state = api::ApiState::setup()
        .await
        .expect("api can be initialised");
    println!("launch server 3");
    let serve_config = ServeConfig::builder()
        .context_provider(move || state.clone())
        .build()
        .expect("valid dioxus config");

    println!("launch server 4");
    let router = axum::Router::new()
        .serve_dioxus_application(serve_config, app)
        .into_make_service();

    println!("launch server 5");
    let listener = tokio::net::TcpListener::bind(socket_addr)
        .await
        .expect("bindable socket address");

    println!("launch server 6");
    axum::serve(listener, router)
        .await
        .expect("servable server");
}
