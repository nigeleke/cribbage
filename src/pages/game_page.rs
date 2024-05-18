use crate::components::prelude::*;
use crate::view::Game;
use crate::services::prelude::*;

use leptos::*;
use leptos_router::*;

#[derive(Clone, Default, Params, PartialEq)]
pub struct GameParams {
    pub id: String,
}

#[component]
pub fn GamePage() -> impl IntoView {
    let params = use_params::<GameParams>; 
    let id = move || params().with(|params| {
        params.as_ref().map(|params| params.id.clone()).unwrap_or_default()
    });

    let game_state = create_rw_signal(None::<Game>);
    provide_context(Context::new(id(), game_state));
    let initial_game = create_resource(id, get_game);

    view! {
        <Suspense fallback=Loading>
            {
                create_effect(move |_| {
                    if let Some(Ok(game)) = initial_game() {
                        game_state.set(Some(game));
                    }
                }); 
            }
            <ErrorBoundary fallback=|_| Error>
                { move || match game_state() {
                    Some(game) => { view! { <Game game=game /> } },
                    None => view! { <Loading /> },
                } }
            </ErrorBoundary>
        </Suspense>
    }
}
