use leptos::*;
use leptos_use::docs::demo_or_body;
use leptos_use::use_cookie;
use leptos_use::utils::FromToStringCodec;
use rand::prelude::*;

#[component]
fn Demo() -> impl IntoView {
    let (counter, set_counter) = use_cookie::<u32, FromToStringCodec>("counter");

    let reset = move || set_counter(Some(random()));

    if counter().is_none() {
        reset();
    }

    let increase = move || {
        set_counter(counter().map(|c| c + 1));
    };

    view! {
        <p>Counter: {move || counter().map(|c| c.to_string()).unwrap_or("—".to_string())}</p>
        <button on:click=move |_| reset()>Reset</button>
        <button on:click=move |_| increase()>+</button>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), || {
        view! { <Demo/> }
    })
}
