use crate::view::CardSlot;

use leptos::*;
use leptos_meta::Style;

/// The component to display a card. The card itself can be presented face-up, face-down,
/// empty (which is a gap) or as a placeholder (which is a card sized dashed line).
/// The card can be "selectable", if on_selected is provided, in which case it will be
/// triggered when the card is selected / unselected.
#[component]
pub fn Card(
    
    card: CardSlot,

    #[prop(optional)]
    opacity: String,
    
    #[prop(optional)]
    on_selected: Option<WriteSignal<bool>>,

) -> impl IntoView {

    let selected = create_rw_signal(false);
    let on_click = move |_| {
        if let Some(on_selected) = on_selected {
            selected.update(|s| *s = !*s);
            on_selected.update(|s| *s = selected() );
        }
    };

    let card = match card {
        CardSlot::FaceUp(card) => view! {
            <div class:selected=selected on:click=on_click>
                <card-t rank=card.face_name() suit=card.suit_name() opacity=opacity /> 
            </div>
        }.into_view(),

        CardSlot::FaceDown => view! {
            <card-t rank="0" backcolor="red" backtext="" />
        }.into_view(),
        
        CardSlot::Empty => view! {
            <div style="visibility: hidden"><card-t /></div> 
        }.into_view(),
        
        CardSlot::Placeholder => view! {
            <div style="border: 2px dashed ghostwhite"><div style="visibility: hidden"><card-t /></div></div> 
        }.into_view(),
    };

    view! {
        <div class="card-wrapper">{card}</div>
        <Style>
        ".card-wrapper {
            width: 120px;
        }
        div.selected {
            transform: translate(0px, -20px) rotate(5deg);
        }"
        </Style>
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::LeptosRuntime;
    use crate::view::Card;

    #[test]
    fn card_should_render_in_a_face_up_slot() {
        LeptosRuntime::new(
            || {
                let card = Card::from("AS");
                Card(CardProps { card: CardSlot::FaceUp(card), opacity: "".into(), on_selected: None })
            },
            |_: &View| {},
            |rendered: String| {
                assert!(rendered.contains(r#"<card-t rank="Ace" suit="Spades" opacity=""#));
            }
        ).run()
    }

    #[test]
    fn card_should_render_in_a_face_down_slot() {
        LeptosRuntime::new(
            || {
                Card(CardProps { card: CardSlot::FaceDown, opacity: "".into(), on_selected: None })
            },
            |_: &View| {},
            |rendered: String| {
                assert!(rendered.contains(r#"<card-t rank="0" backcolor="red" backtext="""#));
            }
        ).run()
    }

    #[test]
    fn card_should_render_in_an_empty_slot() {
        LeptosRuntime::new(
            || {
                Card(CardProps { card: CardSlot::Empty, opacity: "".into(), on_selected: None })
            },
            |_: &View| {},
            |rendered: String| {
                assert!(!rendered.contains(r#"style="border: 2px dashed ghostwhite;""#));
                assert!(rendered.contains(r#"style="visibility: hidden;"><card-t"#));
            }
        ).run()
    }

    #[test]
    fn card_should_render_in_as_a_placeholder() {
        LeptosRuntime::new(
            || {
                Card(CardProps { card: CardSlot::Placeholder, opacity: "".into(), on_selected: None })
            },
            |_: &View| {},
            |rendered: String| {
                assert!(rendered.contains(r#"style="border: 2px dashed ghostwhite;""#));
                assert!(rendered.contains(r#"style="visibility: hidden;"><card-t"#));
            }
        ).run()
    }

}