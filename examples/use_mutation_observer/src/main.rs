use leptos::html::Div;
use leptos::prelude::*;
use leptos_use::docs::demo_or_body;
use leptos_use::{use_mutation_observer_with_options, UseMutationObserverOptions};
use std::time::Duration;

#[component]
fn Demo() -> impl IntoView {
    let el = NodeRef::<Div>::new();
    let (messages, set_messages) = signal(vec![]);
    let (class_name, set_class_name) = signal(String::new());
    let (style, set_style) = signal(String::new());

    use_mutation_observer_with_options(
        el,
        move |mutations, _| {
            if let Some(mutation) = mutations.first() {
                set_messages.update(move |messages| {
                    messages.push(format!("{:?}", mutation.attribute_name()));
                });
            }
        },
        UseMutationObserverOptions::default().attributes(true),
    );

    let _ = set_timeout_with_handle(
        move || {
            set_class_name.set("test test2".to_string());
        },
        Duration::from_millis(1000),
    );

    let _ = set_timeout_with_handle(
        move || {
            set_style.set("color: red;".to_string());
        },
        Duration::from_millis(1550),
    );

    let enum_msgs =
        Signal::derive(move || messages.get().into_iter().enumerate().collect::<Vec<_>>());

    view! {
        <div node_ref=el class=move || class_name.get() style=move || style.get()>
            <For
                each=move || enum_msgs.get()
                // list only grows so this is fine here
                key=|message| message.0
                let:message
            >
                <div>"Mutation Attribute: " <code>{message.1}</code></div>
            </For>
        </div>
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
