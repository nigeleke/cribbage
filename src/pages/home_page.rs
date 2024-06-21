use crate::services::*;

use leptos::*;
use style4rs::style;

/// The inital home page for the app.
/// Allows a new game to be started.
/// TODO: Show user's games that aren't finished.
/// TODO: Show other games that can be joined.
#[component]
pub fn HomePage() -> impl IntoView {
    let class = style! {
        div {
            display: flex;
            flex-direction: row;
            justify-content: space-around;
        }
    };

    let on_click = move |_| {
        spawn_local(async {
            let _ = create_game().await;
        });
    };

    view! {
        class = class,
        <div>
            <button on:click=on_click>"Start new game"</button>
        </div>
    }
}
