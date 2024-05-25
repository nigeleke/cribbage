use crate::view::prelude::CardSlot;

use leptos::*;
use style4rs::style;

/// The component to display a card. The card itself can be presented face-up, face-down,
/// empty (which is a gap) or as a placeholder (which is a card sized dashed line).
/// An optional label can be shown below the card.
/// The card can be "selectable", if on_selected is provided, in which case it will be
/// triggered when the card is selected / unselected.
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
        CardSlot::Placeholder => view! { <div style="border: 2px dashed ghostwhite"><div style="visibility: hidden"><card-t /></div></div> }.into_view(),
    };

    view!{
        class = class,
        <div>
            <div>{card_view}</div>
            <div>{label}</div>
        </div>
    }

}
