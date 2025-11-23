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
        move |current_games: Vec<AvailableGameDTO>,
              filter: String,
              since_when: Since,
              replace: bool| async move {
            let result =
                api::view::get_available_games(*user_id.read(), Some(filter), since_when).await;
            match result {
                Ok(response) => {
                    if replace {
                        games.set(Vec::from(response.games()));
                    } else {
                        let mut new_games = response
                            .games()
                            .iter()
                            .filter_map(|g| (!current_games.contains(g)).then_some(g.clone()))
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

    let _ = use_resource({
        let games = games.read().clone();
        let filter = filter.read().clone();
        let since = since.read().clone();
        move || {
            let games = games.clone();
            let filter = filter.clone();
            let since = since.clone();
            async move { fetch_games(games, filter, since, false).await }
        }
    });

    let on_filter_changed = move |value: String| {
        filter.set(value.clone());
        spawn(async move {
            fetch_games(games(), value, api::view::Since::default(), true).await;
        });
    };

    let on_load_more = move |_| async move {
        fetch_games(games(), "".into(), since(), false).await;
    };

    rsx! {
        Filter { on_filter_changed }
        InnerList { games }
        LoadMore { has_more, on_load_more }
    }
}

#[component]
fn InnerList(games: ReadSignal<Vec<AvailableGameDTO>>) -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/css/available_games_list.css") }
        div {
            class: "available-games-list",
            ul {
                class: "available-games-list__list-items",
                for game in games().into_iter() {
                    li {
                        class: "available-games-list__list-item",
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
