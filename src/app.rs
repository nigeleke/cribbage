use crate::error_template::{AppError, ErrorTemplate};

use crate::pages::{
    game_page::*,
    home_page::*,
};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/cribbage.css"/>
        <Stylesheet id="leptos-components" href="/pkg/cribbage-components.css"/>
        <Script src="/elements.cardmeister.min.js"/>
        <Title text="Cribbage"/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <header><a href="/">"Cribbage"</a></header>
            <main>
                <Routes>
                    <Route path="" view=HomePage />
                    <Route path="game/:id" view=GamePage />
                </Routes>
            </main>
            <footer>"Copyright © 2024; Nigel Eke - All rights reserved."</footer>
        </Router>
    }
}
