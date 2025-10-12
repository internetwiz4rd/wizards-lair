use crate::components;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    // Add a 404
    let fallback = || view! { "Page not found" }.into_view();

    view! {
        <components::Intro />
        <components::StatusCafe />
        <components::GuestBook />
    }
}
