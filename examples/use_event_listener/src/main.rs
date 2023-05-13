use leptos::ev::click;
use leptos::*;
use leptos_use::use_event_listener_ref;
use web_sys::HtmlDivElement;

#[component]
fn Demo(cx: Scope) -> impl IntoView {
    let element = create_node_ref(cx);

    let _ = use_event_listener_ref(cx, element, click, |evt| {
        log!(
            "click from element {:?}",
            event_target::<HtmlDivElement>(&evt)
        );
    });

    let (cond, set_cond) = create_signal(cx, true);

    view! { cx,
        <p>"Check in the dev tools console"</p>
        <p>
            <label>
                <input
                    type="checkbox" on:change=move |evt| set_cond(event_target_checked(&evt))
                    prop:checked=cond
                />
                "Condition enabled"
            </label>
        </p>
        <Show
            when=move || cond()
            fallback=move |cx| view! { cx, <div node_ref=element>"Condition false [click me]"</div> }
        >
            <div node_ref=element>"Condition true [click me]"</div>
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
