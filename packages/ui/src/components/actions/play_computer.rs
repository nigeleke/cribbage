use crate::route::Route;
use api::UserIdDTO;
use dioxus::prelude::*;

#[component]
pub fn PlayComputerAction() -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();
    let navigator = use_navigator();

    let play_computer = move |_| {
        spawn(async move {
            match api::action::play_computer(*user_id.read()).await {
                Ok(game_id) => {
                    navigator.push(Route::GamePage { game_id });
                }
                Err(error) => {
                    warn!("HomePage:play_computer:error {error:?}");
                    let error = error.to_string();
                    navigator.push(Route::ErrorPage { error });
                }
            };
        });
    };

    rsx! {
        button { disabled: true, onclick: play_computer, "Play with Computer" }
    }
}
