use leptos::*;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use dotenv;
    dotenv::dotenv().ok();

    use cribbage::app::*;
    use cribbage::ssr::database::*;
    use cribbage::ssr::fileserv::file_and_error_handler;

    use axum::Router;
    use axum_session::{SessionConfig, SessionLayer, SessionNullPool, SessionStore};
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};

    let session_config = SessionConfig::default().with_table_name("session_table");
    let session_store = SessionStore::<SessionNullPool>::new(None, session_config)
        .await
        .unwrap();

    init_database()
        .await
        .expect("Problem during database initialization");

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;

    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .layer(SessionLayer::new(session_store))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    logging::log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() { }
