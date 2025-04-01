use leptos::prelude::*;
use neocities::{self, Neocities};

fn main() {
    println!("Hello, world!");

    let api_key = String::from("b286a1fe030f141ccb4d514897e2cc19");

    let instance: Neocities = Neocities::new(api_key);

    leptos::mount::mount_to_body(|| view! { <p>"Hello, world! This is Leptos!"</p> });
    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(|| view! { <p>"Hola todos :)"</p> });
}
