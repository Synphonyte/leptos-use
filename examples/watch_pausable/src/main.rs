use leptos::html::Input;
use leptos::prelude::*;
use leptos_use::docs::{demo_or_body, Note};
use leptos_use::{watch_pausable, WatchPausableReturn};

#[component]
fn Demo() -> impl IntoView {
    let input = NodeRef::<Input>::new();
    let (log, set_log) = signal("".to_string());
    let (source, set_source) = signal("".to_string());

    let WatchPausableReturn {
        pause,
        resume,
        is_active,
        ..
    } = watch_pausable(
        move || source.get(),
        move |v, _, _| {
            set_log.update(|log| *log = format!("{log}Changed to \"{v}\"\n"));
        },
    );

    let clear = move |_| set_log.set("".to_string());

    let pause = move |_| {
        set_log.update(|log| *log = format!("{log}Paused\n"));
        pause();
    };

    let resume = move |_| {
        set_log.update(|log| *log = format!("{log}Resumed\n"));
        resume();
    };

    view! {
        <Note class="mb-2">"Type something below to trigger the watch"</Note>
        <input
            node_ref=input
            class="block"
            prop:value=move || source.get()
            on:input=move |e| set_source.set(event_target_value(&e))
            type="text"
        />
        <p>"Value: " {source}</p>
        <button prop:disabled=move || !is_active.get() class="orange" on:click=pause>
            "Pause"
        </button>
        <button prop:disabled=move || is_active.get() on:click=resume>
            "Resume"
        </button>
        <button on:click=clear>"Clear Log"</button>
        <br/>
        <br/>
        <Note>"Log"</Note>
        <pre>{log}</pre>
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
