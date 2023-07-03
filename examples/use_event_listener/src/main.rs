use leptos::ev::{click, keydown};
use leptos::html::A;
use leptos::*;
use leptos_use::use_event_listener;

#[component]
fn Demo(cx: Scope) -> impl IntoView {
    let _ = use_event_listener(cx, window(), keydown, |evt| {
        log!("window keydown: '{}'", evt.key());
    });

    let element = create_node_ref::<A>(cx);

    let _ = use_event_listener(cx, element, click, |evt| {
        log!(
            "click from element '{:?}'",
            event_target::<web_sys::HtmlElement>(&evt).inner_text()
        );
        evt.stop_propagation();
        evt.prevent_default();
    });

    let (cond, set_cond) = create_signal(cx, true);

    view! { cx,
        <p>"Check in the dev tools console"</p>
        <p>
            <label>
                <input
                    type="checkbox" on:change=move |evt| set_cond.set(event_target_checked(&evt))
                    prop:checked=move || cond.get()
                />
                "Condition enabled"
            </label>
        </p>
        <Show
            when=move || cond.get()
            fallback=move |cx| view! { cx,
                <a node_ref=element href="#">
                    "Condition"
                    <b>" false "</b>
                    "[click me]"
                </a>
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

    mount_to_body(|cx| {
        view! {cx,
            <Demo />
        }
    })
}
