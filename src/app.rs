// TODO:
//  - Add the markdown_view_leptos crate to the project
//  - Figure out how to convert the blog posts at compile time, and serve them in O(1) time
//  - Finish styling the websit, use a dark theme, and add some decorations
//  - Create a workflow to
//      - Edit blog posts in neovim or obsidian
//      - Polish them up
//      - Add them to the wizards-lair directory and publish them, with post metadata

use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

#[component]
pub fn App() -> impl IntoView {
    // Add a 404
    let fallback = || view! { "Page not found" }.into_view();

    view! {
        <Router>
            <nav>
                <a href="/">"Home"</a>
                <a href="/about">"About"</a>
                <a href="/posts">"Posts"</a>
            </nav>
            <main>
                <Routes fallback>
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/about") view=About />
                    <Route path=path!("/posts") view=Posts />
                </Routes>
            </main>
        </Router>
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Post {
    title: String,
    tags: Vec<String>,
}

#[derive(Debug)]
pub enum PostError {
    InvalidId,
    PostNotFound,
}

#[component]
pub fn About() -> impl IntoView {
    view! { <h1>"About me!!!"</h1> }
}

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <h1>"Welcome to my lair..."</h1>
        <h2>"A work in progress space on the internet"</h2>
        <StatusCafe />
        <footer>
            <GuestBook />
        </footer>
    }
}

// Shows guest book button
#[component]
pub fn GuestBook() -> impl IntoView {
    view! {
        <a href="http://users3.smartgb.com/g/g.php?a=s&i=g36-38298-df">
            <img src="assets/written_in_vi.gif" />
        </a>
    }
}

#[component]
pub fn Posts() -> impl IntoView {
    view! { <h1>"Blog Posts"</h1> }
}

#[component]
pub fn StatusCafe() -> impl IntoView {
    view! {
        <div id="statuscafe">
            <div id="statuscafe-username"></div>
            <div id="statuscafe-content"></div>
        </div>
        <script src="https://status.cafe/current-status.js?name=internet_wizard" defer></script>
        <br />
    }
}

#[component]
pub fn EmailSpam() -> impl IntoView {
    let (name, set_name) = signal("Controlled".to_string());
    let email = RwSignal::new("".to_string());
    let spam_me = RwSignal::new(true);

    view! {
        <input type="text" bind:value=(name, set_name) />
        <input type="email" bind:value=email />
        <label>
            "Please send me lots of spam email." <input type="checkbox" bind:checked=spam_me />
        </label>
        <p>"Name is: " {name}</p>
        <p>"Email is: " {email}</p>
        <Show when=move || spam_me.get()>
            <p>"You’ll receive cool bonus content!"</p>
        </Show>
    }
}
