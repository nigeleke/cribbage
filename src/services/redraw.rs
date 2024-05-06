use crate::view::Game as GameView;

use leptos::{html::Head, server_fn::request::browser::Request, *};
use leptos_router::*;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct MyQuery {
    foo: Option<String>,
}

#[server]
pub async fn redraw(game_id: String) -> Result<GameView, ServerFnError> {
    use crate::domain::prelude::{Card, Game, Player};
    use crate::ssr::auth;
    use crate::ssr::database::prelude::*;

    logging::log!(">> redraw: game_id: {}", game_id);

    let player: Player = auth::authenticated_user().await?.into();

    logging::log!(">> redraw::player: {:?}", player);

    use axum::extract::Query;
    use axum::http::HeaderMap;
    use leptos_axum::extract;

    let (header, query): (HeaderMap, Query<MyQuery>) = extract().await?;
    logging::log!(">> redraw::header: {:?}", header);
    logging::log!(">> redraw::params: {:?}", query.0);

    unreachable!();

    // let game_id = Uuid::try_parse(&id)?;

    // let connection = get_database().await;
    // connection.acquire().await?;

    // let mut transaction = connection.begin().await?;

    // let game = select_game(&mut transaction, &game_id).await?;
    // let f

    // transaction
    //     .commit()
    //     .await
    //     .map_err(ServerFnError::WrappedServerError)?;

    // let view: GameView = (game, player).into();

    // Ok(view)
}
