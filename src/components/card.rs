use crate::domain::prelude::Card;

use leptos::*;
use style4rs::style;

#[component]
pub fn Card(
    #[prop(optional)]
    card: Option<Card>,
    #[prop(optional)]
    label: Option<String>,
) -> impl IntoView {
    let class = style!{
        div {
            display: flex;
            flex-direction: column;
            width: 120px;
        }
    };

    let label = label.unwrap_or("".into());
    let card_view = match card {
        Some(card) => view! { <card-t rank=card.face_name() suit=card.suit_name() /> },
        None => view! { <card-t /> },
    };

    view!{
        class = class,
        <div>
            <div>{card_view}</div>
            <div>{label}</div>
        </div>
    }

}
