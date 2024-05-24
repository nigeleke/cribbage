use crate::view::CardSlot;

use leptos::*;
use style4rs::style;

#[component]
pub fn Card(
    #[prop()] card: CardSlot,
    #[prop(optional)] label: Option<String>,
    #[prop(optional)] on_selected: Option<WriteSignal<bool>>,

) -> impl IntoView {
    let class = style!{
        div {
            display: flex;
            flex-direction: column;
            width: 120px;
        }
    };
    let inner = style!{
        div.selected {
            transform: translate(0px, -20px) rotate(5deg);
        }
    };

    let selected = create_rw_signal(false);
    let on_click = move |_| {
        if let Some(on_selected) = on_selected {
            selected.update(|s| *s = !*s);
            on_selected.update(|s| *s = selected() );
        }
    };

    let label = label.unwrap_or("".into());
    let card_view = match card {
        CardSlot::FaceUp(card) => view! {
            class = inner,
            <div class:selected=selected on:click=on_click>
                <card-t rank=card.face_name() suit=card.suit_name() /> 
            </div>
        }.into_view(),
        CardSlot::FaceDown => view! { <card-t rank="0" backcolor="red" backtext="" /> }.into_view(),
        CardSlot::Empty => view! { <div style="visibility: hidden"><card-t /></div> }.into_view(),
    };

    view!{
        class = class,
        <div>
            <div>{card_view}</div>
            <div>{label}</div>
        </div>
    }

}
