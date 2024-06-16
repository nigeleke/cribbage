use leptos::*;

/// Display server errors to the user in a friendly way.
#[component]
pub fn Error() -> impl IntoView {
    // TODO: Make friendly error message.
    view!{ <span>"An error occurred..."</span> }
}
