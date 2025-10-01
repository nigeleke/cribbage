use api::CardState;
use dioxus::prelude::*;

/// The component to display a card. The card itself can be presented face-up, face-down,
/// empty (which is a gap) or as a placeholder (which is a card sized dashed line).
/// The card can be "selectable", if on_selected is provided, in which case it will be
/// triggered when the card is selected / unselected.
#[component]
pub fn CardView(
    card: ReadOnlySignal<Card>,
    selected: Option<bool>,
    on_click: Option<EventHandler<Card>>,
) -> Element {
    let is_selected = selected.unwrap_or(false);

    let onclick = move |_| {
        if let Some(on_click) = on_click {
            on_click.call(card());
        }
    };

    rsx! {
        document::Script { src: asset!("/assets/js/elements.cardmeister.min.js")},
        document::Stylesheet { href: asset!("/assets/css/card_view.css")},
        div {
            class: "card-view",
            class: if is_selected { "selected" },
            class: if on_click.is_some() { "selectable" },
            onclick: onclick,
            match &*display.read() {
                CardDisplayState::FaceUp => rsx! { CardFace { card } },
                CardDisplayState::FaceDown => rsx! { CardBack { } },
                CardDisplayState::Placeholder => rsx! { CardPlace { } },
                CardDisplayState::Hidden => rsx! { },
            }
        }
    }
}

const WIDTH: &str = "100px";
const HEIGHT: &str = "160px";

#[component]
fn CardFace(card: ReadOnlySignal<Card>) -> Element {
    rsx! {
        div {
            class: "card",
            dangerous_inner_html: format!("<playing-card cid='{card}' style='display: inline-block; width: {WIDTH}; height: {HEIGHT};' />"),
        }
    }
}

#[component]
fn CardBack() -> Element {
    rsx! {
        div {
            class: "card",
            dangerous_inner_html: format!("<playing-card cid='00' backcolor='#546F82' style='display: inline-block; width: {WIDTH}; height: {HEIGHT};' />"),
        }
    }
}

#[component]
fn CardPlace() -> Element {
    rsx! {
        div {
            class: "placeholder",
            min_width: WIDTH,
            min_height: HEIGHT,
            span {}
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use dioxus::prelude::*;
    use dioxus_core::*;

    fn render(root: fn() -> Element) -> String {
        let mut dom = VirtualDom::new(root);
        let mut mutations = Mutations::default();
        dom.rebuild(&mut mutations);
        dioxus::ssr::render(&dom)
    }

    #[test]
    fn card_should_render_in_a_face_up_slot() {
        let rendered = render(|| {
            let card = Card("AS".into());
            rsx! {
                CardView {
                    card: card,
                    display: CardDisplayState::FaceUp,
                }
            }
        });
        insta::assert_snapshot!(rendered, @r#"<div class="card-view  "><div class="card"><playing-card cid='AS' style='display: inline-block; width: 100px; height: 160px;' /></div></div>"#);
    }

    #[test]
    fn card_should_render_in_a_face_down_slot() {
        let rendered = render(|| {
            let card = Card("AS".into());
            rsx! {
                CardView {
                    card: card,
                    display: CardDisplayState::FaceDown,
                }
            }
        });
        insta::assert_snapshot!(rendered, @r#"<div class="card-view  "><div class="card"><playing-card cid='00' backcolor='#546F82' style='display: inline-block; width: 100px; height: 160px;' /></div></div>"#);
    }

    #[test]
    fn card_should_render_in_a_placeholder() {
        let rendered = render(|| {
            let card = Card("AS".into());
            rsx! {
                CardView {
                    card: card,
                    display: CardDisplayState::Placeholder,
                }
            }
        });
        insta::assert_snapshot!(rendered, @r#"<div class="card-view  "><div class="placeholder" style="min-width:100px;min-height:160px;"><span></span></div></div>"#);
    }

    #[test]
    fn card_should_render_hidden() {
        let rendered = render(|| {
            let card = Card("AS".into());
            rsx! {
                CardView {
                    card: card,
                    display: CardDisplayState::Hidden,
                }
            }
        });
        insta::assert_snapshot!(rendered, @r#"<div class="card-view  "></div>"#);
    }

    #[test]
    fn card_should_render_selected() {
        let rendered = render(|| {
            let card = Card("AS".into());
            rsx! {
                CardView {
                    card: card,
                    display: CardDisplayState::FaceUp,
                    selected: true,
                }
            }
        });
        insta::assert_snapshot!(rendered, @r#"<div class="card-view selected "><div class="card"><playing-card cid='AS' style='display: inline-block; width: 100px; height: 160px;' /></div></div>"#);
    }

    #[test]
    fn card_should_allow_on_click() {
        let rendered = render(|| {
            let card = Card("AS".into());
            let on_click = |_: Card| {};
            rsx! {
                CardView {
                    card,
                    display: CardDisplayState::FaceUp,
                    on_click,
                }
            }
        });
        insta::assert_snapshot!(rendered, @r#"<div class="card-view  selectable"><div class="card"><playing-card cid='AS' style='display: inline-block; width: 100px; height: 160px;' /></div></div>"#);
    }
}
