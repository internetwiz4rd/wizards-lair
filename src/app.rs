// TODO:
//  - Add the markdown_view_leptos crate to the project
//  - Figure out how to convert the blog posts at compile time, and serve them in O(1) time
//  - Finish styling the websit, use a dark theme, and add some decorations
//  - Create a workflow to
//      - Edit blog posts in neovim or obsidian
//      - Polish them up
//      - Add them to the wizards-lair directory and publish them, with post metadata
//  - Look at a proc macro that injects the posts at compile time

// use markdown_view_leptos::markdown_view;
use leptos::{Params, prelude::*};
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes},
    // hooks::{use_params, use_query},
    params::Params,
    path,
};

#[component]
pub fn App() -> impl IntoView {
    // Add a 404
    let fallback = || view! { "Page not found" }.into_view();

    view! {
        <Router>
            <nav>
                <ul>
                    <li>
                        <a href="/">"Home"</a>
                    </li>
                    <li>
                        <a href="/about">"About"</a>
                    </li>
                    <li>
                        <a href="/blog">"Blog"</a>
                    </li>
                </ul>
            </nav>
            <main>
                <Routes fallback>
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/about") view=About />
                    <ParentRoute path=path!("/blog") view=Blog>
                        <Route path=path!(":id") view=Post />
                        <Route path=path!("") view=PostList />
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
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
        <div class="fixed">"Hello!!"</div>
    }
}

#[derive(Params, Clone, Debug, PartialEq, Eq)]
pub struct PostParams {
    id: Option<String>,
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
pub fn Blog() -> impl IntoView {
    view! {
        <h1>"Blog"</h1>
        <Outlet />
    }
}

#[component]
pub fn Post() -> impl IntoView {}

#[component]
pub fn PostList() -> impl IntoView {
    // let posts: Vec<String> = match get_posts() {
    //     Ok(p) => p
    //         .iter()
    //         .map(|r| r.to_str().map_or("".to_string(), |s| s.to_string()))
    //         .collect(),
    //     Err(e) => {
    //         vec!["No posts found :(".to_string(), e.to_string()]
    //     }
    // };
    //
    // view! { <ul>{posts.into_iter().map(|n| view! { <li>{n}</li> }).collect::<Vec<_>>()}</ul> }
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
