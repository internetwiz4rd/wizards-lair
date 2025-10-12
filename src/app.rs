use crate::components;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <components::Intro />
        <components::StatusCafe />
        <components::GuestBook />
    }
}
