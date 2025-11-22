use super::filter::Filter;
use super::load_more::LoadMore;
use crate::components::SelectGameAction;
use api::view::Since;
use api::{AvailableGameDTO, UserIdDTO};
use dioxus::prelude::*;

#[component]
pub fn AvailableGamesList() -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();

    let mut games = use_signal(Vec::<AvailableGameDTO>::default);
    let mut filter = use_signal(String::default);
    let mut since = use_signal(Since::default);
    let mut has_more = use_signal(|| false);

    let fetch_games = {
        move |replace: bool| async move {
            let result =
                api::view::get_available_games(*user_id.read(), Some(filter()), since()).await;
            match result {
                Ok(response) => {
                    if replace {
                        games.set(Vec::from(response.games()));
                    } else {
                        let mut new_games = response
                            .games()
                            .iter()
                            .filter_map(|g| (!games.read().contains(g)).then_some(g.clone()))
                            .collect::<Vec<_>>();
                        games.write().append(&mut new_games);
                    }
                    since.set(response.since().clone());
                    has_more.set(response.has_more());
                }
                Err(e) => {
                    error!("Failed to fetch games: '{e}'");
                }
            }
        }
    };

    let _ = use_resource(move || fetch_games(true));

    use_effect(move || {
        let _ = filter();
        let _ = since();
        spawn(async move {
            fetch_games(true).await;
        });
    });

    let on_filter_changed = move |value| {
        filter.set(value);
        since.set(api::view::Since::default());
    };

    let on_load_more = move |_| async move {
        fetch_games(false).await;
    };

    rsx! {
        Filter { filter, on_filter_changed }
        InnerList { games }
        LoadMore { has_more, on_load_more }
    }
}

#[component]
fn InnerList(games: ReadSignal<Vec<AvailableGameDTO>>) -> Element {
    rsx! {
        div {
            class: "home-page__join-game__list",
            ul {
                class: "home-page__join-game__list-items",
                for game in games().into_iter() {
                    li {
                        class: "home-page__join-game__list-item",
                        class: if matches!(&game, AvailableGameDTO::Active { .. }) { "active" },
                        title: "{game.name()}",
                        key: "{game.id()}",
                        SelectGameAction { game: game.clone() }
                    }
                }
            }
        }
    }
}
