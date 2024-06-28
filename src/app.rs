use crate::error_template::{AppError, ErrorTemplate};

use crate::pages::{
    game_page::*,
    home_page::*,
};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use thaw::*;

/// The main cribbage application view.
#[component]
pub fn App() -> impl IntoView {
    let is_routing = create_rw_signal(false);
    let set_is_routing = SignalSetter::map(move |is_routing_data| {
        is_routing.set(is_routing_data);
    });

    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/cribbage.css"/>
        <Link rel="icon" type_="image/ico" href="/public/favicon.ico"/>
        <Script src="/elements.cardmeister.min.js"/>
        <Title text="Cribbage"/>

        <Router set_is_routing fallback=|| {
                let mut outside_errors = Errors::default();
                outside_errors.insert_with_default_key(AppError::NotFound);
                view! {
                    <ErrorTemplate outside_errors/>
                }
                .into_view()
        }>
            <TheProvider>
                <TheRouter is_routing />
            </TheProvider>
        </Router>
    }
}

#[component]
fn TheRouter(is_routing: RwSignal<bool>) -> impl IntoView {
    let loading_bar = use_loading_bar();
    _ = is_routing.watch(move |is_routing| {
        if *is_routing {
            loading_bar.start();
        } else {
            loading_bar.finish();
        }
    });

    view! {
        <>
            <header><a href="/">"Cribbage"</a></header>
            <main>
                <Routes>
                    <Route path="" view=HomePage />
                    <Route path="game/:id" view=GamePage />
                </Routes>
            </main>
            <footer>"Copyright © 2024; Nigel Eke - All rights reserved."</footer>
        </>
    }
}

#[component]
fn TheProvider(children: Children) -> impl IntoView {
    view! {
        <MessageProvider>
            <LoadingBarProvider>{children()}</LoadingBarProvider>
        </MessageProvider>
    }
}
