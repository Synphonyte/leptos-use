use leptos::*;
use leptos_use::docs::{demo_or_body, Note};
use leptos_use::use_active_element;

#[component]
fn Demo(cx: Scope) -> impl IntoView {
    let active_element = use_active_element(cx);
    let key = move || {
        format!(
            "{:?}",
            active_element.get()
                .map(|el| el.dataset().get("id"))
                .unwrap_or_default()
        )
    };

    view! { cx,
        <Note class="mb-3">"Select the inputs below to see the changes"</Note>

        <div class="grid grid-cols-1 md:grid-cols-3 gap-2">
            <For
                each=move || (1..7)
                key=|i| *i
                view=move |cx, i| view! { cx,
                    <input type="text" data-id=i class="!my-0 !min-w-0" placeholder=i />
                }
            />
        </div>

        <div class="mt-2">
            "Current Active Element: "
            <span class="text-primary">{ key }</span>
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
