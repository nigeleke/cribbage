use api::dto::UserIdDTO;
use dioxus::prelude::*;

use crate::{components::button::*, route::Route};

#[component]
pub fn HostGameAction() -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();

    let mut game_id = use_signal(|| None);

    let navigator = use_navigator();

    use_effect(move || {
        if let Some(game_id) = game_id() {
            navigator.push(Route::GamePage { game_id });
        }
    });

    let mut host_game_action = use_action(move || async move {
        let result = api::action::host_game(*user_id.read()).await;
        match result {
            Ok(id) => game_id.set(Some(id)),
            Err(ref error) => {
                warn!("{error}");
                todo!() // Toast errors
            }
        }
        result
    });

    let host_game = move |_| host_game_action.call();

    rsx! {
         Button {
             variant: ButtonVariant::Primary,
             onclick: host_game,
             "Play with Friends"
         }
    }
}
