use leptos::*;

/// Show consistent view for a loading state.
#[component]
pub fn Loading() -> impl IntoView {
    view!{ <span>"Loading..."</span> }
}
