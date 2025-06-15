#![feature(coverage_attribute)]

use api::ApiState;
use dioxus::prelude::*;
use std::{any::Any, sync::Arc};

#[coverage(off)]
pub async fn launch_server(app: fn() -> Element) {
    dioxus::logger::initialize_default();

    let socket_addr = dioxus_cli_config::fullstack_address_or_localhost();

    let state = ApiState::setup().await.expect("api can be initialised");

    let state_arc = Arc::new(state);
    let context_providers: Arc<Vec<Box<dyn Fn() -> Box<dyn Any> + Send + Sync>>> =
        Arc::new(vec![{
            let state_clone = state_arc.clone();
            Box::new(move || Box::new(state_clone.clone()) as Box<dyn Any>)
                as Box<dyn Fn() -> Box<dyn Any> + Send + Sync>
        }]);

    let serve_config = ServeConfigBuilder::default().context_providers(context_providers);

    let router = axum::Router::new()
        .serve_dioxus_application(serve_config, app)
        .into_make_service();

    let listener = tokio::net::TcpListener::bind(socket_addr)
        .await
        .expect("bindable socket address");

    axum::serve(listener, router)
        .await
        .expect("servable server");
}
