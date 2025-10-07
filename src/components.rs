use leptos::prelude::*;

#[component]
pub fn Intro() -> impl IntoView {
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
pub fn GuestBook() -> impl IntoView {
    view! {
        <a href="http://users3.smartgb.com/g/g.php?a=s&i=g36-38298-df">
            <img src="assets/written_in_vi.gif"/>
        </a>
    }
}

#[component]
pub fn StatusCafe() -> impl IntoView {
    view! {
        <div id="statuscafe"><div id="statuscafe-username"></div><div id="statuscafe-content"></div></div><script src="https://status.cafe/current-status.js?name=internet_wizard" defer></script>
        <br/>
    }
}

#[component]
pub fn EmailSpam() -> impl IntoView {
    let (name, set_name) = signal("Controlled".to_string());
    let email = RwSignal::new("".to_string());
    let spam_me = RwSignal::new(true);

    view! {
        <input type="text"
            bind:value=(name, set_name)
        />
        <input type="email"
            bind:value=email
        />
        <label>
            "Please send me lots of spam email."
            <input type="checkbox"
                bind:checked=spam_me
            />
        </label>
        <p>"Name is: " {name}</p>
        <p>"Email is: " {email}</p>
        <Show when=move || spam_me.get()>
            <p>"You’ll receive cool bonus content!"</p>
        </Show>
    }
}
