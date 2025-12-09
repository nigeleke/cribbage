use api::dto::{GameIdDTO, PhaseDTO, UserGameDTO, UserIdDTO};
use dioxus::prelude::*;

use crate::{
    components::{
        CuttingForDeal, Discarding, Finished, InLobby, Playing, ScoringCrib, ScoringDealer,
        ScoringPone,
    },
    toast::Toast,
};

#[component]
pub fn GamePage(game_id: GameIdDTO) -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();
    let game_id = provide_context(game_id);

    let mut game = use_signal(Option::<UserGameDTO>::default);

    let mut game_stream = use_action(move || async move {
        let mut stream = api::stream::user_game_stream(*user_id.read(), game_id).await?;
        while let Some(Ok(updated_game)) = stream.next().await {
            game.set(Some(updated_game));
        }
        dioxus::Ok(())
    });

    let get_game_result = use_resource(move || async move {
        let current_game = api::view::get_game(*user_id.read(), game_id).await?;
        game_stream.call();
        game.set(Some(current_game));
        dioxus::Ok(())
    });

    use_effect(move || {
        if let Some(Err(error)) = get_game_result.result() {
            Toast::server_error("get game", error.to_string());
        }
    });

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/game_page.css") }
        div {
            class: "game-page",
            if let Some(game) = game() {
                ActiveGame { game }
            } else {
                div { "Loading..." }
            }
        }
    }
}

#[component]
fn ActiveGame(game: ReadSignal<UserGameDTO>) -> Element {
    provide_context(game);

    match game().phase {
        PhaseDTO::InLobby => rsx! { InLobby {} },
        PhaseDTO::CuttingForDeal => rsx! { CuttingForDeal {} },
        PhaseDTO::Discarding => rsx! { Discarding {} },
        PhaseDTO::Playing => rsx! { Playing {} },
        PhaseDTO::ScoringPone => rsx! { ScoringPone {} },
        PhaseDTO::ScoringDealer => rsx! { ScoringDealer {} },
        PhaseDTO::ScoringCrib => rsx! { ScoringCrib {} },
        PhaseDTO::Finished => rsx! { Finished {} },
    }
}
