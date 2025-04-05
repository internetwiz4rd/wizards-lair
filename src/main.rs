use leptos::prelude::*;
// use leptos_router::components::*;
// use leptos_router::path;
// use leptos_router::hooks::use_params_map;

fn main() {
    leptos::mount::mount_to_body(Intro);
    leptos::mount::mount_to_body(CountingButton);
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

#[component]
fn CountingButton() -> impl IntoView {
    let (count, set_count) = signal(5);
    let double_count = move || count.get() *2;

    view! {
        <button
            on:click=move |_| {
                *set_count.write() += 1;
            }

            class=("red", move || count.get() % 2 == 1)
        >
            "Click me: " { count }
        </button>

        <ProgressBar max=100 progress=count/>
        <ProgressBar progress=Signal::derive(double_count)/>

        <button
            on:click=move |_| {
                *set_count.write() = 0;
            }
        >
            "Reset"
        </button>
    }
}

// Shows guest book button
#[component]
fn GuestBook() -> impl IntoView {
    view! {
        <a href="http://users3.smartgb.com/g/g.php?a=s&i=g36-38298-df">
            <img src="assets/written_in_vi.gif"/>
        </a>
    }
}

// Shows progress bar
#[component]
fn ProgressBar(
    // Maximum value for the progress bar
    #[prop(default = 100)]
    max: u16,
    // How much progress to disply
    #[prop(into)]
    progress: Signal<i32>
) -> impl IntoView {
    view! {
        <progress
            max=max
            value=progress
        />
        <br/>
    }
}
