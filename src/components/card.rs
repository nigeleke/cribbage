use crate::view::CardSlot;

use leptos::*;
use style4rs::style;

#[component]
pub fn Card(
    #[prop()]
    card: CardSlot,
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
        CardSlot::FaceUp(card) => view! { <card-t rank=card.face_name() suit=card.suit_name() /> }.into_view(),
        CardSlot::FaceDown => view! { <card-t rank="0" backcolor="red" backtext="" /> }.into_view(),
        CardSlot::Empty => view! { <></> }.into_view(),
    };

    view!{
        class = class,
        <div>
            <div>{card_view}</div>
            <div>{label}</div>
        </div>
    }

}
