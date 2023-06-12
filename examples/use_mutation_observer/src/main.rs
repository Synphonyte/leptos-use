use leptos::*;
use leptos_use::docs::demo_or_body;
use leptos_use::use_mutation_observer_with_options;
use std::time::Duration;

#[component]
fn Demo(cx: Scope) -> impl IntoView {
    let el = create_node_ref(cx);
    let (messages, set_messages) = create_signal(cx, vec![]);
    let (class_name, set_class_name) = create_signal(cx, String::new());
    let (style, set_style) = create_signal(cx, String::new());

    let mut init = web_sys::MutationObserverInit::new();
    init.attributes(true);

    use_mutation_observer_with_options(
        cx,
        el,
        move |mutations, _| {
            if let Some(mutation) = mutations.first() {
                set_messages.update(move |messages| {
                    messages.push(format!("{:?}", mutation.attribute_name()));
                });
            }
        },
        init,
    );

    let _ = set_timeout_with_handle(
        move || {
            set_class_name("test test2".to_string());
        },
        Duration::from_millis(1000),
    );

    let _ = set_timeout_with_handle(
        move || {
            set_style("color: red;".to_string());
        },
        Duration::from_millis(1550),
    );

    let enum_msgs = Signal::derive(cx, move || {
        messages.get().into_iter().enumerate().collect::<Vec<_>>()
    });

    view! { cx,
        <div node_ref=el class=class_name style=style>
            <For
                each=enum_msgs
                key=|message| message.0 // list only grows so this is fine here
                view=|cx, message| view! { cx, <div>"Mutation Attribute: " <code>{message.1}</code></div> }
            />
        </div>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), |cx| {
        view! { cx, <Demo /> }
    })
}
