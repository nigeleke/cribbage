use api::dto::UserIdDTO;
use dioxus::prelude::*;

use crate::{Toast, components::button::*, route::Route};

#[component]
pub fn PlayComputerAction() -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();

    let mut game_id = use_signal(|| None);

    let navigator = use_navigator();

    use_effect(move || {
        if let Some(game_id) = game_id() {
            navigator.push(Route::GamePage { game_id });
        }
    });

    let mut play_computer_action = use_action(move || async move {
        let result = api::action::play_computer(*user_id.read()).await;
        match result {
            Ok(id) => game_id.set(Some(id)),
            Err(ref error) => {
                Toast::command_error("Play computer", error.to_string());
            }
        };
        result
    });

    let play_computer = move |_| play_computer_action.call();

    rsx! {
        Button {
            variant: ButtonVariant::Primary,
            disabled: true,
            onclick: play_computer,
            "Play with Computer"
        }
    }
}
