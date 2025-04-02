use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(|| view! { 
        <h1>"Welcome to my lair..."</h1>
        <h2>"A work in progress space on the internet"</h2>
        <p>I need to create a place to put all my shit in here.</p>

        <p>TODO</p>
        <ul>
        <li>"Figure out site structure"</li>
        <li>"Create a skeleton with interactive functionality via Leptos"</li>
        </ul>

    });

    leptos::mount::mount_to_body(counting_button);
}

fn counting_button() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <button
            on:click=move |_| {
                *set_count.write() += 1;
            }
        >
            "Click me: " { count }
        </button>
    }
}

