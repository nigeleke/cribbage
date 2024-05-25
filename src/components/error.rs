use leptos::*;

/// Display server errors to the user in a friendly way.
#[component]
pub fn Error() -> impl IntoView {
    view!{ <span>"An error occurred..."</span> }
}
