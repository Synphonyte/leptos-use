use leptos::ev::{click, keydown};
use leptos::html::A;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_use::{use_event_listener, use_window};

#[component]
fn Demo() -> impl IntoView {
    let _ = use_event_listener(use_window(), keydown, |evt| {
        log!("window keydown: '{}'", evt.key());
    });

    let element = create_node_ref::<A>();

    let _ = use_event_listener(element, click, |evt| {
        log!(
            "click from element '{:?}'",
            event_target::<web_sys::HtmlElement>(&evt).inner_text()
        );
        evt.stop_propagation();
        evt.prevent_default();
    });

    let (cond, set_cond) = signal(true);

    view! {
        <p>"Check in the dev tools console"</p>
        <p>
            <label>
                <input
                    type="checkbox"
                    on:change=move |evt| set_cond.set(event_target_checked(&evt))
                    prop:checked=move || cond.get()
                />
                "Condition enabled"
            </label>
        </p>
        <Show
            when=move || cond.get()
            fallback=move || {
                view! {
                    <a node_ref=element href="#">
                        "Condition"
                        <b>" false "</b>
                        "[click me]"
                    </a>
                }
            }
        >

            <a node_ref=element href="#">
                "Condition "
                <b>"true"</b>
                " [click me]"
            </a>
        </Show>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! { <Demo/> }
    })
}
