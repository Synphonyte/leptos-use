use leptos::prelude::*;
use leptos_use::docs::{demo_or_body, Note};
use leptos_use::{watch_debounced_with_options, WatchDebouncedOptions};

#[component]
fn Demo() -> impl IntoView {
    let (input, set_input) = signal("".to_string());
    let (updated, set_updated) = signal(0);

    let _ = watch_debounced_with_options(
        move || input.get(),
        move |_, _, _| {
            set_updated.update(|x| *x += 1);
        },
        1000.0,
        WatchDebouncedOptions::default().max_wait(Some(5000.0)),
    );

    view! {
        <input
            class="block"
            prop:value=move || input.get()
            on:input=move |e| set_input.set(event_target_value(&e))
            placeholder="Try to type anything..."
            type="text"
        />
        <Note>
            <code>"ms"</code>
            " is set to 1000ms and "
            <code>"max_wait"</code>
            " is set to 5000ms for this demo."
        </Note>
        <p>"Input: " {input}</p>
        <p>"Times Updated: " {updated}</p>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    let _ = leptos::mount::mount_to(demo_or_body(), || {
        view! { <Demo/> }
    });
}
