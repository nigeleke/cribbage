use crate::components::{DebouncedInput, SelectGameAction, button::*, scroll_area::*};
use api::dto::{AvailabilityDTO, AvailableGameDTO, UserIdDTO};
use api::view::Since;
use dioxus::prelude::*;
use dioxus_primitives::scroll_area::ScrollDirection;

enum FetchAction {
    Replace,
    Append,
}

#[component]
pub fn AvailableGamesList() -> Element {
    let user_id = use_context::<Signal<UserIdDTO>>();

    let mut games = use_signal(Vec::<AvailableGameDTO>::default);
    let mut filter = use_signal(String::default);

    let show = |what: &str, items: &[AvailableGameDTO]| {
        debug!("***** WHAT: {what}");
        let items = items
            .iter()
            .map(|g| format!("{} {}", g.name, g.game_id.value()))
            .collect::<Vec<_>>();
        debug!("{:#?}", items);
        debug!("*****");
    };

    let mut last_since = use_signal(Since::default);

    let filtered_games = use_memo(move || {
        let filter = filter.read().to_ascii_lowercase();

        let f = if filter.is_empty() {
            games.read().clone()
        } else {
            games
                .read()
                .iter()
                .filter(|g| g.name.to_ascii_lowercase().contains(&filter))
                .cloned()
                .collect::<Vec<_>>()
        };

        show("filtered_games: f", &f);
        debug!("filtered_games: since: {:?}", last_since());

        f
    });

    let mut has_more = use_signal(|| false);

    let fetch_games = move |action: FetchAction, filter: String, since: Since| async move {
        let result = api::view::get_available_games(*user_id.read(), Some(filter), since).await;

        match result {
            Ok(response) => {
                has_more.set(response.has_more());
                last_since.set(response.since().clone());
                let mut fetched_games = Vec::from(response.games());
                show("fetch_games: fetched_games:", &fetched_games);
                debug!("fetch_games: since: {:?}", response.since());
                match action {
                    FetchAction::Replace => games.set(fetched_games),
                    FetchAction::Append => games.write().append(&mut fetched_games),
                }
            }

            Err(e) => error!("Failed to fetch games: '{e}'"),
        }
    };

    let _ = use_resource(move || async move {
        fetch_games(FetchAction::Replace, "".into(), Since::default()).await
    });

    let on_filter_changed = move |value: String| async move {
        filter.set(value.clone());
        fetch_games(FetchAction::Replace, value, Since::default()).await;
    };

    let on_load_more = move |_| async move {
        show("on_load_more: games:", &games.read());
        debug!("on_load_more: since: {:?}", last_since());
        fetch_games(FetchAction::Append, filter(), last_since()).await;
    };

    rsx! {
        Filter { on_filter_changed }
        InnerList { games: filtered_games() }
        LoadMoreButton { has_more, on_load_more }
    }
}

#[component]
fn InnerList(games: Vec<AvailableGameDTO>) -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/css/available_games_list.css") }
        ScrollArea {
            class: "available-games-list",
            direction: ScrollDirection::Vertical,
            ul {
                class: "available-games-list__list_items",
                for game in games.into_iter() {
                    li {
                        class: if matches!(&game.availability, AvailabilityDTO::Private) { "active" },
                        title: "{game.name}",
                        key: "{game.game_id.value()}",
                        SelectGameAction { game: game.clone() }
                    }
                }
            }
        }
    }
}

#[component]
fn Filter(on_filter_changed: Callback<String>) -> Element {
    let mut filter = use_signal(|| String::default());

    let on_debounced_input = move |value: String| {
        filter.set(value.clone());
        on_filter_changed.call(value);
    };

    rsx! {
        DebouncedInput {
            value: filter,
            placeholder: "🔍 Search games...",
            name: "available_games_filter",
            on_debounced_input,
        }
    }
}

#[component]
fn LoadMoreButton(has_more: ReadSignal<bool>, on_load_more: Callback<()>) -> Element {
    rsx! {
        Button {
            variant: ButtonVariant::Outline,
            disabled: !has_more(),
            onclick: move |_| on_load_more(()),
            "More..."
        }
    }
}
