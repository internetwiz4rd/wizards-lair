use leptos::prelude::*;
// use stylance::*;
// use leptos_router::components::*;
// use leptos_router::path;
// use leptos_router::hooks::use_params_map;

fn main() {
    leptos::mount::mount_to_body(Intro);
    leptos::mount::mount_to_body(StatusCafe);
    leptos::mount::mount_to_body(GuestBook);
}

#[component]
fn Intro() -> impl IntoView {
    view! { 
            <h1>"Welcome to my lair..."</h1>
            <h2>"A work in progress space on the internet"</h2>
            <p>"I need to create a place to put all my shit in here."</p>

            <p>"TODO"</p>
            <ul>
            <li>"Figure out site structure"</li>
            <li>"Create a skeleton with interactive functionality via Leptos"</li>
            </ul>
    }
}

// Shows guest book button
#[component]
fn GuestBook() -> impl IntoView {
    view! {
        <a href="http://users3.smartgb.com/g/g.php?a=s&i=g36-38298-df">
            <img src="written_in_vi.gif"/>
        </a>
    }
}

#[component]
fn StatusCafe() -> impl IntoView {
    view! {
        <div id="statuscafe"><div id="statuscafe-username"></div><div id="statuscafe-content"></div></div><script src="https://status.cafe/current-status.js?name=internet_wizard" defer></script>
        <br/>
    }
}
