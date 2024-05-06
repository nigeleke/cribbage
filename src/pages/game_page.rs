use crate::components::prelude::*;
use crate::services::prelude::*;
use crate::view::*;

use leptos::*;
use leptos_router::*;

#[derive(Clone, Params, PartialEq)]
pub struct GameParams {
    pub id: String,
}

#[component]
pub fn GamePage() -> impl IntoView {
    let params = use_params::<GameParams>();
    let id = move || params.with(|p| p.as_ref().map(|p| p.id.clone()).ok().unwrap_or_default());
    provide_context(id);
    let game = create_resource(id, get_game);

    view! {
        <Suspense fallback=loading>
            <ErrorBoundary fallback=|_| cannot_find_game>
                { move || match game() {
                    Some(result) => view_game_result(result).into_view(),
                    None => cannot_find_game().into_view(),
                } }
            </ErrorBoundary>
        </Suspense>
    }
}

fn loading() -> impl IntoView {
    view! { <span>"Loading ..."</span> }
}

fn cannot_find_game() -> impl IntoView {
    view! { <span>"Cannot find game ..."</span> }
}

fn view_game_result(game: Result<Game, ServerFnError>) -> impl IntoView {
    match game {
        Ok(game) => view_game(&game).into_view(),
        Err(_) => cannot_find_game().into_view(),
    }
}

fn view_game(game: &Game) -> impl IntoView {
    view!{ <Game game=game.clone() /> }
}
