use crate::components::prelude::*;
use crate::services::prelude::*;
use crate::view::prelude::Game;

use leptos::*;
use leptos_router::*;

#[derive(Clone, Default, Params, PartialEq)]
pub struct GameParams {
    pub id: String,
}

/// The main game page, enables loading of the current state of the game from the server.
/// The game must exist, or an error is shown.
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
                    Some(game) => { view! { <Game game /> } },
                    None => view! { <Loading /> },
                } }
            </ErrorBoundary>
        </Suspense>
    }
}
