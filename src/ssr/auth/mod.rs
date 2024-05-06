pub mod user;

use user::User;

use axum_extra::extract::CookieJar;
use leptos::*;
use leptos_axum::extract;
use uuid::Uuid;

pub async fn authenticated_user() -> Result<User, ServerFnError> {
    let cookies: CookieJar = extract().await?;
    let Some(cookie) = cookies.get("session") else {
        return Err(ServerFnError::MissingArg("Unknown User".into()))
    };

    let user_id = Uuid::parse_str(cookie.value_trimmed())?;
    logging::log!("user_id: {:?}", user_id);

    Ok(user_id.into())
}
