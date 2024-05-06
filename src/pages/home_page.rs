use crate::services::prelude::*;

use leptos::*;
use style4rs::style;

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
