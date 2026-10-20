use leptos::prelude::*;
use leptos_use_docs::demo_or_body;
use leptos_use_raf_fn::use_raf_fn;
use leptos_use_filters::Pausable;

#[component]
fn Demo() -> impl IntoView {
    let (count, set_count) = signal(0);

    let Pausable {
        pause,
        resume,
        is_active,
    } = use_raf_fn(move |_| {
        set_count.update(|count| *count += 1);
    });

    view! {
        <div>Count: {count}</div>
        <button on:click=move |_| pause() disabled=move || !is_active()>
            Pause
        </button>
        <button on:click=move |_| resume() disabled=is_active>
            Resume
        </button>
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
