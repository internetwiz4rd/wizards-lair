use crate::wizardslair::components::*;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Intro/>
        <StatusCafe/>
        <GuestBook/>
    } 
}
