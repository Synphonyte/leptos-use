use codee::string::FromToStringCodec;
use leptos::prelude::*;
use leptos_use::docs::demo_or_body;
use leptos_use::use_cookie;
use rand::prelude::*;

#[component]
fn Demo() -> impl IntoView {
    let (counter_a, set_counter_a) = use_cookie::<u32, FromToStringCodec>("counter_a");
    let (counter_b, set_counter_b) = use_cookie::<u32, FromToStringCodec>("counter_b");

    let reset_a = move || set_counter_a(Some(random()));
    let reset_b = move || set_counter_b(Some(random()));

    if counter_a().is_none() {
        reset_a();
    }
    if counter_b().is_none() {
        reset_b();
    }

    let increase_a = move || {
        set_counter_a(counter_a().map(|c| c + 1));
    };
    let increase_b = move || {
        set_counter_b(counter_b().map(|c| c + 1));
    };

    view! {
        <p>Counter A: {move || counter_a().map(|c| c.to_string()).unwrap_or("—".to_string())}</p>
        <button on:click=move |_| reset_a()>Reset</button>
        <button on:click=move |_| increase_a()>+</button>
        <p>Counter B: {move || counter_b().map(|c| c.to_string()).unwrap_or("—".to_string())}</p>
        <button on:click=move |_| reset_b()>Reset</button>
        <button on:click=move |_| increase_b()>+</button>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    let unmount_handle = leptos::mount::mount_to(demo_or_body(), || {
        view! { <Demo/> }
    });

    unmount_handle.forget();
}
