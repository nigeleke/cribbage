/// Show the current crib, in a position relevant to the dealer, and the current cut card.
#[component]
fn CribAndCut() -> impl IntoView {

    let game = use_context::<GameView>().unwrap();
    let dealer = game.dealer();

    match game {
        GameView::Starting(_) => empty_view().into_view(),
        GameView::Discarding(_, _, crib, _) => crib_and_cut_view(&crib, CardSlot::FaceDown, dealer.unwrap()).into_view(),
        GameView::Playing(_, _, _, cut, crib, _) => crib_and_cut_view(&crib, CardSlot::FaceUp(cut), dealer.unwrap()).into_view(),
        GameView::ScoringPone(_, _, _, cut, crib) => crib_and_cut_view(&crib, CardSlot::FaceUp(cut), dealer.unwrap()).into_view(),
        GameView::ScoringDealer(_, _, _, cut, crib) => crib_and_cut_view(&crib, CardSlot::FaceUp(cut), dealer.unwrap()).into_view(),
        GameView::ScoringCrib(_, _, _, cut, crib) => crib_and_cut_view(&crib, CardSlot::FaceUp(cut), dealer.unwrap()).into_view(),
    }
}

fn empty_view() -> impl IntoView {
    view! { <div></div> }
}

fn crib_and_cut_view(crib: &[CardSlot], cut: CardSlot, dealer: Role) -> impl IntoView {
    let class = style!{
        div {
            display: flex;
            flex-direction: column;
            justify-content: space-between;
        }
    };

    let empty_view = view! { <Card card=CardSlot::Empty /> };
    let crib_view = view! { <Crib crib=Vec::from(crib) stacked=true /> };

    view! {
        class = class,
        <div>
            {if dealer == Role::CurrentPlayer { crib_view.clone() } else { empty_view.clone() }}
            <Card card=cut />
            {if dealer == Role::Opponent { crib_view } else { empty_view }}
        </div>
    }

}