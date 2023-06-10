use leptos::*;
use leptos_use::docs::{demo_or_body, Note};
use leptos_use::{watch_pausable, WatchPausableReturn};

#[component]
fn Demo(cx: Scope) -> impl IntoView {
    let input = create_node_ref(cx);
    let (log, set_log) = create_signal(cx, "".to_string());
    let (source, set_source) = create_signal(cx, "".to_string());

    let WatchPausableReturn {
        pause,
        resume,
        is_active,
        ..
    } = watch_pausable(cx, source, move |v, _, _| {
        set_log.update(|log| *log = format!("{log}Changed to \"{v}\"\n"));
    });

    let clear = move |_| set_log("".to_string());

    let pause = move |_| {
        set_log.update(|log| *log = format!("{log}Paused\n"));
        pause();
    };

    let resume = move |_| {
        set_log.update(|log| *log = format!("{log}Resumed\n"));
        resume();
    };

    view! { cx,
        <Note class="mb-2">"Type something below to trigger the watch"</Note>
        <input
            node_ref=input
            class="block"
            prop:value=source
            on:input=move |e| set_source(event_target_value(&e))
            type="text"
        />
        <p>"Value: " {source}</p>
        <button prop:disabled=move || !is_active() class="orange" on:click=pause>"Pause"</button>
        <button prop:disabled=is_active on:click=resume>"Resume"</button>
        <button on:click=clear>"Clear Log"</button>
        <br />
        <br />
        <Note>"Log"</Note>
        <pre>{log}</pre>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), |cx| {
        view! { cx, <Demo /> }
    })
}
