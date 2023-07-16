use leptos::*;
use leptos_use::docs::demo_or_body;
use leptos_use::{use_raf_fn, utils::Pausable};

#[component]
fn Demo(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    let Pausable {
        pause,
        resume,
        is_active,
    } = use_raf_fn(cx, move |_| {
        set_count.update(|count| *count += 1);
    });

    view! { cx,
        <div>Count: { count }</div>
        <button on:click=move |_| pause() disabled=move || !is_active()>Pause</button>
        <button on:click=move |_| resume() disabled=is_active>Resume</button>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), |cx| {
        view! { cx, <Demo /> }
    })
}
