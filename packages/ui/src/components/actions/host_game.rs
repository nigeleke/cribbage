use crate::route::Route;
use api::UserIdDTO;
use dioxus::prelude::*;

#[component]
pub fn HostGameAction() -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();
    let navigator = use_navigator();

    let host_game = move |_| {
        spawn(async move {
            match api::action::host_game(*user_id.read()).await {
                Ok(game_id) => {
                    navigator.push(Route::GamePage { game_id });
                }
                Err(error) => {
                    warn!("HomePage:host_game:error {error:?}");
                    let error = error.to_string();
                    navigator.push(Route::ErrorPage { error });
                }
            }
        });
    };

    rsx! {
         button { onclick: host_game, "Play with Friends" }
    }
}
