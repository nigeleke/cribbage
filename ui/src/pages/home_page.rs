use crate::services::*;

use leptos::*;
use thaw::*;

/// The inital home page for the app.
/// Allows a new game to be started.
/// TODO: Show user's games that aren't finished.
/// TODO: Show other games that can be joined.
#[component]
pub fn HomePage() -> impl IntoView {
    let on_click = move |_| {
        spawn_local(async {
            let _ = create_game().await;
        });
    };

    view! {
        <Space justify=SpaceJustify::Center>
            <Button on_click>"Start new game"</Button>
        </Space>
    }
}
